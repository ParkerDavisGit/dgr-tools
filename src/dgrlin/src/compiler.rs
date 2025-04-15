use log;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

use byteorder::{ByteOrder, LittleEndian, BigEndian, WriteBytesExt};

use crate::opcode::Opcode;


pub fn text_to_byte(filename: String) -> Result<(), eyre::Report> {
    log::info!("compiling {}", filename);

    let f = File::open(filename)?;
    let reader = BufReader::new(f).lines().flatten();

    log::info!("opened file");

    let mut bytes: Vec<u8> = Vec::new();
    let mut text_list: Vec<String> = Vec::new();
    let mut text_id: u32 = 0u32;

    // SECTION 0 [ HEADER ]
    // 2 0 0 0 16 0 0 0    <- File Identifier
    // 0 0 0 0  0 0 0 0    <- Buffer to insert needed byte numbers later.
    bytes.append(&mut vec![2u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8,
                           0u8, 0u8, 0u8, 0u8,  0u8, 0u8, 0u8, 0u8]);
    
    // SECTION 1 [ OPCODES ]
    // Mainly just take each line and see what sticks.
    // TODO: stop and tell python when something doesn't work here
    // I WANT LINE NUMBERS HERE
    for line in reader {
        let try_from_result: (Option<Opcode>, Option<String>) = Opcode::try_from_string(line, text_id);

        match try_from_result.0 {
            Some(operation) => {
                bytes.append(&mut operation.to_hex());
                match try_from_result.1 {
                    Some(line) => {
                        text_list.push(line);
                        text_id += 1;
                    },
                    None => {}
                }
            },
            None => {},
        }
        return Ok(());
    }

    // SECTION 2 [ TEXT SCRIPT OFFSETS ]
    // First, Buffer to nearest multiple of 16
    // Extra buffer PROBABLY won't break anything
    let buffer_amount = if bytes.len() % 16 == 0 {
        0
    } else {
        16 - bytes.len() % 16
    };
    
    for _ in 0..buffer_amount {
        bytes.push(0u8)
    }

    // Starts with text.len(), which is text_id at the current moment
    let mut text_address: u32 = bytes.len() as u32;
    let mut text_address_vec: Vec<u8> = Vec::new();
    let _ = text_address_vec.write_u32::<LittleEndian>(text_address);
    bytes[8] = text_address_vec[0];
    bytes[9] = text_address_vec[1];
    bytes[10] = text_address_vec[2];
    bytes[11] = text_address_vec[3];

    log::debug!("{}", text_address);

    let _ = bytes.write_u32::<LittleEndian>(text_id);
    
    // Then things get weird
    // Each following 2 bytes are the number of bytes between the start of section 2
    // 2-byte 0x00 buffers in between entries.
    // And the corresponding entry in section 3.
    // Eg. Text(3) calls line 4: "This is a line of dialog"
    // Sec. 2 starts at 0x007E, 4th line is at 0x017E. 4th Entry in Sec. 2 is '00 10' (Little Endian)
    let mut offset = text_id * 4u32 + 4u32;
    for line in text_list {
        let _ = bytes.write_u32::<LittleEndian>(offset);
        offset += 2u32 + line.len() as u32;
    }
    let _ = bytes.write_u32::<LittleEndian>(offset);

    // SECTION 3 [ TEXT SCRIPT ]


    // SECTION 0 AGAIN [ ADD BYTE NUMBERS ]
    let mut text_address_vec: Vec<u8> = Vec::new();
    let _ = text_address_vec.write_u32::<LittleEndian>(text_address);
    bytes[12] = text_address_vec[0];
    bytes[13] = text_address_vec[1];
    bytes[14] = text_address_vec[2];
    bytes[15] = text_address_vec[3];


    log::info!("compiled file");

    // And write to file
    // Luckily, this one is handled well
    let mut file = File::create("output/output.bin")?;
    let _ = file.write(&bytes[..]);
    
    log::info!("wrote to file");

    Ok(())
}
