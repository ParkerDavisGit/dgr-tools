use pyo3::prelude::*;

use log;

use std::io::BufReader;
use std::fs::read;
use std::fmt;
use hex::ToHex;
use std::fs::File;
use std::io::prelude::*;

use byteorder::{ByteOrder, LittleEndian, BigEndian, WriteBytesExt};

pub mod opcode;
use opcode::Opcode;

fn text_to_byte(filename: String) -> Result<(), eyre::Report> {
    log::info!("compiling {}", filename);

    let f = File::open(filename)?;
    let mut reader = BufReader::new(f).lines().flatten();

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
    //let ops: Vec<Result<Op, &'static str>> = ops.into_iter().flatten().collect();

    let mut file = File::create("output/output.bin")?;

    file.write(&bytes[..]);
    
    log::info!("wrote to file");

    Ok(())
}

fn byte_to_text(filename: String) -> Result<(), eyre::Report> {
    log::info!("decompiling {}", filename);
    //let mut data = read("data/e00_004_003.bytecode").unwrap().into_iter().peekable();
    let mut data = read(filename).unwrap().into_iter().peekable();
    let mut ops: Vec<Opcode> = Vec::new();
    let mut idx = 0usize;

    log::info!("opened file");

    loop {
        if data.peek() == None {
            break;
        }
        if data.next() != Some(112u8) {
            break;
        }

        idx += 1;

        let cmd: u8 = data.next().unwrap();
        let opcode_info: (&str, u8) = match vec![cmd].encode_hex::<String>().as_str() {
            "00" => ("0x00", 2u8),
            "02" => ("Text", 2u8),
            "03" => ("TextBoxFormat", 1u8),
            "06" => ("Animation", 8u8),
            "08" => ("Voice", 5u8),
            "09" => ("Music", 3u8),
            "0a" => ("Sound", 3u8),
            "15" => ("LoadMap", 3u8),
            "19" => ("LoadScript", 3u8),
            "1a" => ("StopScript", 0u8),
            "1e" => ("Sprite", 5u8),
            "1f" => ("ScreenFlash", 7u8),
            "20" => ("SpriteFlash", 5u8),
            "21" => ("Speaker", 1u8),
            "22" => ("ScreenFade", 3u8),
            "25" => ("ChangeUi", 2u8),
            "26" => ("SetFlag", 3u8),
            "27" => ("CheckCharacter", 1u8),
            "29" => ("CheckObject", 1u8),
            "2a" => ("SetLabel", 2u8),
            "2b" => ("SetChoiceText", 1u8),
            "30" => ("ShowBackground", 3u8),
            "33" => ("0x33", 4u8),
            "34" => ("GoToLabel", 2u8),
            "35" => ("CheckFlagA", 255u8),
            "3a" => ("WaitInput", 0u8),
            "3b" => ("WaitFrame", 0u8),
            "3c" => ("IfFlagCheck", 0u8),
            default => {
                for line in ops {
                    println!("{}", line);
                }
                eyre::bail!("Invalid opcode '{}' at line {}", default, idx);
            }
        };

        if opcode_info.0 == "CheckFlagA" {
            let mut temp_hex_vec: Vec<u8> = vec![112u8, 0u8];

            while data.peek().unwrap() != &112u8 {
                temp_hex_vec.push(data.next().unwrap());
            }

            ops.push(Opcode {
                name: opcode_info.0.to_string(),
                hexcode: temp_hex_vec,
                text_id: None
            });

            continue;
        }

        if opcode_info.0 == "Text" {
            let mut temp_hex_vec: Vec<u8> = vec![112u8, 0u8];

            temp_hex_vec.push(data.next().unwrap());
            temp_hex_vec.push(data.next().unwrap());

            ops.push(Opcode {
                text_id: Some(BigEndian::read_u16(&temp_hex_vec[2..4])),
                name: opcode_info.0.to_string(),
                hexcode: temp_hex_vec
            });

            continue;
        }

        let mut temp_hex_vec: Vec<u8> = vec![112u8, 0u8];

        for _ in 0..opcode_info.1 {
            temp_hex_vec.push(data.next().unwrap());
        }

        ops.push(Opcode {
            name: opcode_info.0.to_string(),
            hexcode: temp_hex_vec,
            text_id: None
        })
    }
    log::info!("decompiled file");
    //let ops: Vec<Result<Op, &'static str>> = ops.into_iter().flatten().collect();

    let mut file = File::create("output/output.txt")?;

    for line in ops {
        write!(file, "{}\n", line);
    }
    
    log::info!("wrote to file");

    Ok(())
}


#[pyfunction]
fn compile(filename: String) -> PyResult<String> {
    text_to_byte(filename);
    Ok("Done!".to_string())
}


#[pyfunction]
fn decompile(filename: String) -> PyResult<String> {
    byte_to_text(filename);
    Ok("Done!".to_string())
}



/// A Python module implemented in Rust.
#[pymodule]
fn dgrlin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(compile, m)?)?;
    m.add_function(wrap_pyfunction!(decompile, m)?)?;
    Ok(())
}
