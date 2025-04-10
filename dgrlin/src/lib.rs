use pyo3::prelude::*;

use log;

use std::io::BufReader;
use std::fs::read;
use std::fmt;
use hex::ToHex;
use std::fs::File;
use std::io::prelude::*;

use byteorder::{ByteOrder, LittleEndian, BigEndian};

pub mod opcode;
use opcode::Opcode;

fn text_to_byte(filename: String) -> Result<(), eyre::Report> {
    log::info!("compiling {}", filename);

    let f = File::open(filename)?;
    let mut reader = BufReader::new(f).lines().flatten();

    log::info!("opened file");

    let mut bytes: Vec<u8> = Vec::new();
    let mut text: Vec<String> = Vec::new();
    let mut text_id: u16 = 0u16;

    for line in reader {
        let try_from_result: (Opcode, Option<String>) = Opcode::try_from_string(line, text_id);
        bytes.append(&mut try_from_result.0.to_hex());
        match try_from_result.1 {
            Some(line) => {
                log::info!("line printed, {}: {}", text_id, line);
                text.push(line);
                text_id += 1;
            },
            None => {}
        }
    }

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
