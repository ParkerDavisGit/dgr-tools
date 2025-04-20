use eyre::Context;
use log;

use std::fs::read;
use hex::ToHex;
use std::fs::File;
use std::io::prelude::*;

use byteorder::{ByteOrder, LittleEndian, BigEndian};

use crate::opcode::Opcode;


pub fn decompile_lin(filename: String, output_folder: String) -> eyre::Result<()> {
    log::info!("decompiling {}", filename);

  //let mut data = read(filename.clone()).unwrap().into_iter().peekable();
    let mut data = match read(filename.clone()) {
        Ok(opened_file) => { opened_file.into_iter().peekable() }
        Err(_) => { eyre::bail!("File \"{}\" could not be opened.", filename) }
    };

    let mut ops:  Vec<Opcode> = Vec::new();
    let mut idx:  usize = 0;

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

    idx += 16;

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

        let cmd: u8 = match data.next() {
            Some(op) => { op }
            None => { eyre::bail!("End of file found prematurly") }
        };
        idx += 1;

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
                eyre::bail!("Invalid opcode '{}' at index {}", default, idx);
            }
        };

        // Special Cases
        // Check Flag A and B can have different numbers of arguments.
        // There is a pattern, but more research needed for confirmation.
        if opcode_info.0 == "CheckFlagA" {
            let mut temp_hex_vec: Vec<u8> = vec![0x70, 0x00];

            while data.peek() != Some(&0x70) {
                temp_hex_vec.push(data.next().expect("This better not be called (decompiler CheckFlagA)"));
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

            while data.peek() != Some(&0x70) {
                temp_hex_vec.push(data.next().expect("This better not be called (decompiler CheckFlagB)"));
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

            temp_hex_vec.push(match data.next() {
                Some(op) => { op }
                None => { eyre::bail!("End of file found prematurly (0x70, 0x02, ---)") }
            });

            temp_hex_vec.push(match data.next() {
                Some(op) => { op }
                None => { eyre::bail!("End of file found prematurly (0x70, 0x02, 0xXX, ---)") }
            });

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
            temp_hex_vec.push(match data.next() {
                Some(op) => { op }
                None => { eyre::bail!("End of file found prematurly.") }
            });
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
                        &[LittleEndian::read_u16(&[
                            match data.next() {
                                Some(op) => { op }
                                None => { eyre::bail!("End of file found prematurly") }
                            }, 
                            match data.next() {
                                Some(op) => { op }
                                None => { eyre::bail!("End of file found prematurly") }
                            }
                        ])]).unwrap().chars().next().unwrap();
                
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

    //let output_filename = filename.rsplit_once("/").unwrap().1.split(".").next().unwrap();
    let output_filename = match filename.rsplit_once("/") {
        // Grab everything after the last slash in the filepath
        Some((_, output_file_with_extension)) => {
            // Remove the file extension
            match output_file_with_extension.split(".").next() {
                Some(output_filename) => { output_filename }
                // This'll call if the input file doesn't have an extension
                None => { output_file_with_extension }
            }
        },
        None => { eyre::bail!("Output Directory not found") }
    };

    let mut file = File::create(output_folder + "/" + output_filename + ".txt").wrap_err("Output Directory not found")?;
    

    // AND write it all down
    let mut indent_level = 0usize;
    let mut flag_check: bool  = false;
    let mut in_choice_text: bool = false;
    let mut line_idx = 1usize;

    // Write each opcode down
    for line in ops {
        // Flag check runs the next line only if it passes
        if flag_check {
            write!(file, "{}{{\n"  , indent(indent_level))
                .wrap_err(format!("Could not write line {}", line_idx))?;
            write!(file, "{}{}\n"  , indent(indent_level+1), line)
                .wrap_err(format!("Could not write line {}", line_idx+1))?;
            write!(file, "{}}}\n"  , indent(indent_level))
                .wrap_err(format!("Could not write line {}", line_idx+2))?;

            line_idx += 3;
            flag_check = false;
            continue;
        }
        
        match line.name.as_str() {
            "CheckCharacter" => {
                while indent_level > 0 {
                    indent_level -= 1;
                    write!(file, "{}}}\n", indent(indent_level))
                        .wrap_err(format!("Could not write line {}", line_idx))?;
                    line_idx += 1;
                    in_choice_text = false;
                }
                
                write!(file, "{}{}\n", indent(indent_level), line)
                    .wrap_err(format!("Could not write line {}", line_idx))?;
                write!(file, "{}{{\n", indent(indent_level))
                    .wrap_err(format!("Could not write line {}", line_idx+1))?;

                line_idx += 2;
                indent_level += 1;
            }
            "CheckObject" => {
                while indent_level > 0 {
                    indent_level -= 1;
                    write!(file, "{}}}\n", indent(indent_level))
                        .wrap_err(format!("Could not write line {}", line_idx))?;

                    line_idx += 1;
                    in_choice_text = false;
                }

                write!(file, "{}{}\n", indent(indent_level), line)
                    .wrap_err(format!("Could not write line {}", line_idx))?;
                write!(file, "{}{{\n", indent(indent_level))
                    .wrap_err(format!("Could not write line {}", line_idx+1))?;
                
                line_idx += 2;
                indent_level += 1;
            }
            "IfFlagCheck" => {
                write!(file, "{}{}\n", indent(indent_level), line)
                    .wrap_err(format!("Could not write line {}", line_idx))?;

                line_idx += 1;
                flag_check = true;
            }

            "SetChoiceText" => {
                if in_choice_text {
                    indent_level -= 1;
                    write!(file, "{}}}\n", indent(indent_level))
                        .wrap_err(format!("Could not write line {}", line_idx))?;
                    
                    line_idx += 1;
                }
                else {
                    in_choice_text = true;
                }

                write!(file, "{}{}\n", indent(indent_level), line)
                    .wrap_err(format!("Could not write line {}", line_idx))?;
                write!(file, "{}{{\n", indent(indent_level))
                    .wrap_err(format!("Could not write line {}", line_idx+1))?;

                line_idx += 2;
                indent_level += 1;
            }

            "Text" => {
                let output = match line.text_id {
                    Some(text_id) => { text_entries.get(text_id as usize) }
                    // This shouldn't be called, as to get to this point, 
                    // a text opcode object should always be instantiated with a text id.
                    // This one's the consiquence of my actions and will probably
                    // Be out of here after my opcode refactor.
                    None => { eyre::bail!("Text opcode does not contain valid text id.") }
                };

                match output {
                    None => {
                        log::error!("Text line with id '{}' not found.", line.text_id.expect("Text id lost between line 376 and now."));
                        continue;
                    }
                    Some(text_line) => { write!(
                        file, "{}Text(\"{}\")\n", 
                        indent(indent_level), 
                        text_line)
                            .wrap_err(format!("Could not write line {}", line_idx))?;

                        line_idx += 1;
                    }
                }
            }
            _ => {
                write!(file, "{}{}\n", indent(indent_level), line)
                    .wrap_err(format!("Could not write line {}", line_idx+1))?;
                line_idx += 1;
            }
        }
    }

    while indent_level > 0 {
        indent_level -= 1;
        let _ = write!(file, "{}}}\n", indent(indent_level));
    }
    
    log::info!("wrote to file");

    Ok(())
}



fn indent(amount: usize) -> String {
    "    ".repeat(amount)
}