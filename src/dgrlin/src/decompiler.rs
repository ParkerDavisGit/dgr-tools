use log;

use std::fs::read;
use hex::ToHex;
use std::fs::File;
use std::io::prelude::*;

use byteorder::{ByteOrder, LittleEndian, BigEndian};

use crate::opcode::Opcode;


pub fn byte_to_text(filename: String, output_folder: String) -> eyre::Result<()> {
    log::info!("decompiling {}", filename);
    //let mut data = read("data/e00_004_003.bytecode").unwrap().into_iter().peekable();
    let mut data = read(filename.clone()).unwrap().into_iter().peekable();
    let mut ops: Vec<Opcode> = Vec::new();
    let mut idx = 0usize;

    log::info!("opened file");

    // SECTION 0 [ HEADER ]
    if data.next() != Some(0x02) {
        log::error!("Not a valid .lin file");
        //return Err(eyre::Report::new(""));
    }
    let _ = (data.next(), data.next(), data.next());

    if data.next() != Some(0x10) {
        log::error!("Not a valid .lin file");
        return Ok(())
    }
    let _ = (data.next(), data.next(), data.next());

    // I shouldn't need the addresses provided here
    let _ = (data.next(), data.next(), data.next(), data.next());
    let _ = (data.next(), data.next(), data.next(), data.next());

    // SECTION 1 [ OPCODES ]
    loop {
        if data.peek() == None { // .lin with no text
            break;
        }
        if data.peek() != Some(&0x70) { // Text Begins
            break;
        }

        let _ = data.next();

        idx += 1;

        let cmd: u8 = data.next().unwrap();
        let opcode_info: (&str, u8) = match vec![cmd].encode_hex::<String>().as_str() {
            "00" => ("0x00", 2u8),
            "02" => ("Text", 2u8),
            "03" => ("TextBoxFormat", 1u8),
            "04" => ("PostProcessingFilter", 4u8),
            "05" => ("Movie", 2u8),
            "06" => ("Animation", 8u8),
            "08" => ("Voice", 5u8),
            "09" => ("Music", 3u8),
            "0a" => ("Sound", 3u8),
            "0b" => ("SoundB", 2u8),
            "0c" => ("AddTruthBullets", 2u8),
            "0d" => ("AddPresents", 3u8),
            "0e" => ("UnlockSkill", 2u8),
            "0f" => ("StudentTitleEntry", 3u8),
            "14" => ("TrialCamera", 3u8),
            "15" => ("LoadMap", 3u8),
            "19" => ("LoadScript", 3u8),
            "1a" => ("StopScript", 0u8),
            "1b" => ("RunScript", 3u8),
            "1c" => ("0x1C", 0u8),
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
            "2e" => ("CameraShake", 2u8),
            "30" => ("ShowBackground", 3u8),
            "33" => ("0x33", 4u8),
            "34" => ("GoToLabel", 2u8),
            "35" => ("CheckFlagA", 255u8),
            "36" => ("CheckFlagB", 255u8),
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

        // Special Cases
        // Check Flag A and B can have different numbers of arguments.
        // There is a pattern, but more research needed for confirmation.
        if opcode_info.0 == "CheckFlagA" {
            let mut temp_hex_vec: Vec<u8> = vec![0x70, 0x00];

            while data.peek().unwrap() != &0x70 {
                temp_hex_vec.push(data.next().unwrap());
            }

            ops.push(Opcode {
                name: opcode_info.0.to_string(),
                hexcode: temp_hex_vec,
                text_id: None
            });

            continue;
        }

        if opcode_info.0 == "CheckFlagB" {
            let mut temp_hex_vec: Vec<u8> = vec![0x70, 0x00];

            while data.peek().unwrap() != &0x70 {
                temp_hex_vec.push(data.next().unwrap());
            }

            ops.push(Opcode {
                name: opcode_info.0.to_string(),
                hexcode: temp_hex_vec,
                text_id: None
            });

            continue;
        }

        // Text's argument is the array index of it's corresponding line of text
        // Stored at end of file, grabbed later.
        if opcode_info.0 == "Text" {
            let mut temp_hex_vec: Vec<u8> = vec![0x70, 0x00];

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

        // Get the hex values
        // I'm realizing there are some redundancies here.
        // Might need refactoring
        let mut temp_hex_vec: Vec<u8> = vec![0x70, 0x00];

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
    // Don't really need this to read the text
    // Recalculated when compiling back into hex
    loop {
        // For textless .lins
        if data.peek() == None {
            break;
        }
        // This section ends with [0xFF, 0xFE]
        if data.next() == Some(0xFF) { 
            if data.next() == Some(0xFE) {
                break;
            }
        }
    }
    

    // SECTION 3 [ THE TEXT SCRIPT ]
    // Lines of text are null-terminated strings seperated by [0xFF, 0xFE]
    let mut text_entries: Vec<String> = Vec::new();
    loop {
        if data.peek() == None {
            break;
        }

        // Use up the line seperator
        if data.peek() == Some(&0xFF) {
            let _ = (data.next(), data.next());
        }

        // Collect each character (2 Bytes) and treat as ascii.
        // This implies the ability to use UTF-8
        // More research needed
        let line: String = {
            let mut next_string_chars: Vec<char> = Vec::new();

            loop {
                let next_char: char = 
                    String::from_utf16(
                        &[LittleEndian::read_u16(&[data.next().unwrap(), data.next().unwrap()])]
                    ).unwrap().chars().next().unwrap();
                
                // let next_char: u8 = data.next().unwrap();
                // let extra: u8 = data.next().unwrap();
                
                // // BACKSLASH
                // // Newlines should be written in plaintext
                // // Converted back into 0x0A when compiling.
                if next_char == 0x0A as char {
                    next_string_chars.push('\\');
                    next_string_chars.push('n');
                    continue;
                }

                // NULL Character
                // Ends the line
                if next_char == 0x00 as char {
                    break;
                }

                next_string_chars.push(next_char as char);
            }

            next_string_chars.into_iter().collect()
        };
        text_entries.push(line);
    }

    log::info!("decompiled file");

    let output_filename = filename.rsplit_once("/").unwrap().1.split(".").next().unwrap();
    let mut file = File::create(output_folder + "/" + output_filename + ".txt")?;

    // AND write it all down
    let mut indentation_level = 0usize;
    let mut flag_check: bool  = false;
    let mut in_choice_text: bool = false;

    // Many many unhandled results here, being thrown out by 'let _ ='
    // Fix this
    for line in ops {
        // Flag check runs the next line only if it passes
        if flag_check {
            let _ = write!(file, "{}{{\n"  , "    ".repeat(indentation_level));
            let _ = write!(file, "{}{}\n"  , "    ".repeat(indentation_level+1), line);
            let _ = write!(file, "{}}}\n"  , "    ".repeat(indentation_level));
            flag_check = false;
            continue;
        }
        
        match line.name.as_str() {
            "CheckCharacter" => {
                while indentation_level > 0 {
                    indentation_level -= 1;
                    let _ = write!(file, "{}}}\n", "    ".repeat(indentation_level));
                    in_choice_text = false;
                }
                
                let _ = write!(file, "{}{}\n", "    ".repeat(indentation_level), line);
                let _ = write!(file, "{}{{\n", "    ".repeat(indentation_level));

                indentation_level += 1;
            }
            "CheckObject" => {
                while indentation_level > 0 {
                    indentation_level -= 1;
                    let _ = write!(file, "{}}}\n", "    ".repeat(indentation_level));
                    in_choice_text = false;
                }

                let _ = write!(file, "{}{}\n", "    ".repeat(indentation_level), line);
                let _ = write!(file, "{}{{\n", "    ".repeat(indentation_level));

                indentation_level += 1;
            }
            "IfFlagCheck" => {
                let _ = write!(file, "{}{}\n", "    ".repeat(indentation_level), line);
                flag_check = true;
            }

            "SetChoiceText" => {
                // if indentation_level != 0 {
                //     indentation_level -= 1;
                //     let _ = write!(file, "{}}}\n", "    ".repeat(indentation_level));
                // }
                
                if in_choice_text {
                    indentation_level -= 1;
                    let _ = write!(file, "{}}}\n", "    ".repeat(indentation_level));
                }
                else {
                    in_choice_text = true;
                }

                let _ = write!(file, "{}{}\n", "    ".repeat(indentation_level), line);
                let _ = write!(file, "{}{{\n", "    ".repeat(indentation_level));
                indentation_level += 1;
            }

            "Text" => {
                let output = text_entries.get(line.text_id.unwrap() as usize);
                match output {
                    None => {
                        log::error!("Text line with id '{}' not found.", line.text_id.unwrap());
                        continue;
                    }
                    _ => { let _ = write!(
                        file, "{}Text(\"{}\")\n", 
                        "    ".repeat(indentation_level), 
                        output.unwrap()); 
                    }
                }
            }
            _ => {
                let _ = write!(file, "{}{}\n", "    ".repeat(indentation_level), line);
            }
        }
    }

    while indentation_level > 0 {
        indentation_level -= 1;
        let _ = write!(file, "{}}}\n", "    ".repeat(indentation_level));
    }
    
    log::info!("wrote to file");

    Ok(())
}