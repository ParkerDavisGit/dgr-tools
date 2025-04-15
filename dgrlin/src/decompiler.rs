use log;

use std::fs::read;
use hex::ToHex;
use std::fs::File;
use std::io::prelude::*;

use byteorder::{ByteOrder, LittleEndian, BigEndian};

use crate::opcode::Opcode;


pub fn byte_to_text(filename: String) -> Result<(), eyre::Report> {
    log::info!("decompiling {}", filename);
    //let mut data = read("data/e00_004_003.bytecode").unwrap().into_iter().peekable();
    let mut data = read(filename).unwrap().into_iter().peekable();
    let mut ops: Vec<Opcode> = Vec::new();
    let mut idx = 0usize;

    log::info!("opened file");

    // SECTION 0 [ HEADER ]
    if data.next() != Some(2u8) {
        log::error!("Not a valid .lin file");
        return Ok(())
    }
    let _ = (data.next(), data.next(), data.next());

    if data.next() != Some(16u8) {
        log::error!("Not a valid .lin file");
        return Ok(())
    }
    let _ = (data.next(), data.next(), data.next());

    // I don't think I need the addresses provided here
    let _ = (data.next(), data.next(), data.next(), data.next());
    let _ = (data.next(), data.next(), data.next(), data.next());

    // SECTION 1 [ OPCODES ]
    loop {
        if data.peek() == None { // .lin with no text
            break;
        }
        if data.peek() != Some(&112u8) { // Text Begins
            break;
        }

        let _ = data.next();

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
                // The only instance of Big Endian in this entire stupid format.
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

    // SECTION 2 [ SKIP ALL THE LINE ADDRESS DATA ]
    loop {
        // This section ends with [0xFF, 0xFE]
        if data.next() == Some(255u8) { 
            if data.next() == Some(254u8) {
                break;
            }
        }
    }

    // SECTION 3 [ THE TEXT SCRIPT ]
    // Each text line ends with [0x00, 0x00, 0xFF, 0xFE]
    let mut text_entries: Vec<String> = Vec::new();
    loop {
        if data.peek() == None {
            break;
        }

        // Use up the line seperator
        if data.peek() == Some(&255u8) {
            let _ = (data.next(), data.next());
        }

        let line: String = {
            let mut next_string_chars: Vec<char> = Vec::new();

            loop {
                let next_char: u8 = data.next().unwrap();
                let extra: u8 = data.next().unwrap();
                
                // BACKSLASH
                // Checking for newline
                if next_char == 10u8 {
                    next_string_chars.push('\\');
                    next_string_chars.push('n');
                    continue;
                }

                // NULL Character
                // Ends the line
                if next_char == 0u8 {
                    if extra == 0u8 {
                        break;
                    }
                }

                next_string_chars.push(next_char as char);
            }

            next_string_chars.into_iter().collect()
        };

        text_entries.push(line);
    }

    // for line in text_entries.clone() {
    //     println!("{}", line);
    // }

    log::info!("decompiled file");
    //let ops: Vec<Result<Op, &'static str>> = ops.into_iter().flatten().collect();

    let mut file = File::create("output/output.txt")?;

    for line in ops {
        match line.name.as_str() {
            "Text" => {
                let output = text_entries.get(line.text_id.unwrap() as usize);
                match output {
                    None => {
                        log::error!("Text line with id '{}' not found.", line.text_id.unwrap());
                        continue;
                    }
                    _ => { let _ = write!(file, "Text(\"{}\")\n", output.unwrap()); }
                }
            }
            _ => {
                let _ = write!(file, "{}\n", line);
            }
        }
    }
    
    log::info!("wrote to file");

    Ok(())
}