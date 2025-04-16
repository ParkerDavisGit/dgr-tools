use log;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

use byteorder::{LittleEndian, WriteBytesExt};

use crate::opcode::Opcode;


pub fn text_to_byte(filename: String) -> eyre::Result<()> {
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
    bytes.append(&mut vec![0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00]);
    bytes.append(&mut vec![0x00; 8]);
    
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
            // One reason this may be called is a lack of double quotes surrounding line
            // This will be thrown back up to the python window to inform user.
            None => {},
        }
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
        bytes.push(0x00)
    }

    // Starts with text.len(), which is text_id at the current moment
    let text_address: u32 = bytes.len() as u32;
    let mut text_address_vec: Vec<u8> = Vec::new();
    let _ = text_address_vec.write_u32::<LittleEndian>(text_address);

    // Update Header
    bytes.splice(8..=11, text_address_vec);

    let _ = bytes.write_u32::<LittleEndian>(text_id);

    let hexed_text_lines: Vec<Vec<u8>> = 
        text_list
            .into_iter()
            .map(|line: String| {
                let mut line_in_hex: Vec<u8> = Vec::new();
                let mut chars = line.chars();
                loop {
                    let c = chars.next();
                    match c {
                        None => {
                            break;
                        },
                        Some('\\') => match chars.next() {
                            Some('n') => { 
                                let _ = line_in_hex.write_u16::<LittleEndian>(0x0A);
                            },
                            Some(c3) => {
                                let _ = line_in_hex.write_u16::<LittleEndian>('\\' as u16);
                                let _ = line_in_hex.write_u16::<LittleEndian>(c3 as u16);
                            }
                            None => {}
                        },
                        Some(the_char) => {
                            let _ = line_in_hex.write_u16::<LittleEndian>(the_char as u16);
                        }
                    }
                }
                line_in_hex.push(0x00);
                line_in_hex.push(0x00);
                line_in_hex
            }
            )
            .collect();
    
    // Then things get weird
    // Each following 2 bytes are the number of bytes between the start of section 2
    // 2-byte 0x00 buffers in between entries.
    // And the corresponding entry in section 3.
    // Eg. Text(3) calls line 4: "This is a line of dialog"
    // Sec. 2 starts at 0x007E, 4th line is at 0x017E. 4th Entry in Sec. 2 is '00 10' (Little Endian)
    let mut offset: u32 = text_id * 4 + 8;
    for hex_line in hexed_text_lines.clone() {
        let _ = bytes.write_u32::<LittleEndian>(offset);
        offset += 2 + (hex_line.len() as u32);
    }
    let _ = bytes.write_u32::<LittleEndian>(offset);

    // SECTION 3 [ TEXT SCRIPT ]
    for mut hex_line in hexed_text_lines {
        bytes.push(0xFF);
        bytes.push(0xFE);

        bytes.append(&mut hex_line);
    }

    // SECTION 0 AGAIN [ ADD BYTE NUMBERS ]
    let mut text_address_vec: Vec<u8> = Vec::new();
    let _ = text_address_vec.write_u32::<LittleEndian>(bytes.len() as u32);

    // Update Header
    bytes.splice(12..=15, text_address_vec);


    log::info!("compiled file");

    // And write to file
    // Luckily, this one is handled well
    let mut file = File::create("output/output.bin")?;
    let _ = file.write(&bytes[..]);
    
    log::info!("wrote to file");

    Ok(())
}
