use pyo3::prelude::*;


use std::io::BufReader;
use std::fs::read;
use std::fmt;
use hex::ToHex;
use std::fs::File;
use std::io::prelude::*;

pub mod opcode;
use opcode::Op;

// pub enum Op {
//     Zero {
//         hexcode: Vec<u8>,
//     },
//     ScreenFade {
//         hexcode: Vec<u8>,
//     },
//     ChangeUi {
//         hexcode: Vec<u8>,
//     },
//     ThreeThree {
//         hexcode: Vec<u8>,
//     },
//     SetFlag {
//         hexcode: Vec<u8>,
//     },
//     LoadMap {
//         hexcode: Vec<u8>,  // MAP ID, 
//     },
//     Sprite {
//         hexcode: Vec<u8>, // ???, CHAR ID, SPRITE ID, ?Visible?, ???
//     },
//     Music {
//         hexcode: Vec<u8>,
//     },
//     ShowBackground {
//         hexcode: Vec<u8>,
//     },
//     Speaker {
//         hexcode: u8, // Speaker ID
//     },
//     TextBoxFormat { // 03
//         hexcode: u8, // Speaker ID
//     },
//     Text { 
//         hexcode: Vec<u8>,
//     },
//     WaitFrame { 
//         hexcode: Vec<u8>,
//     },
//     WaitInput { 
//         hexcode: Vec<u8>,
//     },
//     Sound { 
//         hexcode: Vec<u8>,
//     }, 
//     ScreenFlash {
//         hexcode: Vec<u8>, // 7
//     },
//     Animation {
//         hexcode: Vec<u8>, // 8
//     },
//     Voice { 
//         hexcode: Vec<u8>, // 5
//     },
//     SpriteFlash { 
//         hexcode: Vec<u8>, // 5
//     },
//     GoToLabel {
//         hexcode: Vec<u8>, // 2
//     },
//     CheckCharacter {
//         hexcode: Vec<u8>, // 1
//     },
//     CheckObject {
//         hexcode: Vec<u8>, // 1
//     },
//     CheckFlagA {
//         hexcode: Vec<u8>, // Varies
//     },
//     If_FlagCheck {
//         hexcode: Vec<u8>, // Varies
//     },
//     SetLabel { 
//         hexcode: Vec<u8>, // 2
//     },
//     SetChoiceText {
//         hexcode: Vec<u8>, // 1
//     },
//     LoadScript {
//         hexcode: Vec<u8>, // 3
//     },
//     StopScript {
//         hexcode: Vec<u8>,
//     },
// }

/// Formats the sum of two numbers as string.
#[pyfunction]
fn compile(op: String) -> PyResult<String> {
    if op == "compile" {
        text_to_byte().unwrap();
    }
    else if op == "decompile" {
        byte_to_text().unwrap();
    }
    
    Ok("Done!".to_string())
}


fn text_to_byte() -> Result<(), eyre::Report> {
    let f = File::open("data/output.txt")?;
    let mut reader = BufReader::new(f).lines().flatten();

    let mut bytes: Vec<u8> = Vec::new();

    for line in reader {
        bytes.append(&mut Op::try_from(line).unwrap().to_hex());
    }

    println!("Parsed!");
    //let ops: Vec<Result<Op, &'static str>> = ops.into_iter().flatten().collect();

    let mut file = File::create("output.bin")?;

    file.write(&bytes[..]);
    
    println!("Writen!");

    Ok(())
}

fn byte_to_text() -> Result<(), eyre::Report> {
    //let mut data = read("data/e00_004_003.bytecode").unwrap().into_iter().peekable();
    let mut data = read("data/new_output.bin").unwrap().into_iter().peekable();
    let mut ops: Vec<Op> = Vec::new();

    loop {
        if data.peek() == None {
            break;
        }
        if data.next().unwrap() != 112u8 {
            break;
        }

        let cmd = data.next();
        match vec![cmd.unwrap()].encode_hex::<String>().as_str() {
            "00" => {
                ops.push(Op::Zero {
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(),
                })
            }
            "02" => {
                ops.push(Op::Text { // 02
                    arg1: data.next().unwrap(), // Text ID
                    arg2: data.next().unwrap(), // Text ID 2
                })
            }
            "03" => {
                ops.push(Op::TextBoxFormat { // 03
                    arg1: data.next().unwrap(), // Speaker ID
                })
            }
            "06" => {
                ops.push(Op::Animation {
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                    arg4: data.next().unwrap(), 
                    arg5: data.next().unwrap(), 
                    arg6: data.next().unwrap(), 
                    arg7: data.next().unwrap(), 
                    arg8: data.next().unwrap(), 
                })
            }
            "08" => {
                ops.push(Op::Voice { // 08
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                    arg4: data.next().unwrap(), 
                    arg5: data.next().unwrap(), 
                })
            }
            "09" => {
                ops.push(Op::Music {
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                })
            }
            "0a" => {
                ops.push(Op::Sound { // 0A
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                })
            }
            "15" => {
                ops.push(Op::LoadMap {
                    arg1: data.next().unwrap(),  // Map ID
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                })
            }
            "19" => {
                ops.push(Op::LoadScript {
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                })
            }
            "1a" => {
                ops.push(Op::StopScript)
            }
            "1e" => {
                ops.push(Op::Sprite { // 1E
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                    arg4: data.next().unwrap(), 
                    arg5: data.next().unwrap(), 
                })
            }
            "1f" => {
                ops.push(Op::ScreenFlash {// 1F
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                    arg4: data.next().unwrap(), 
                    arg5: data.next().unwrap(), 
                    arg6: data.next().unwrap(), 
                    arg7: data.next().unwrap(), 
                })
            }
            "20" => {
                ops.push(Op::SpriteFlash { // 20
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                    arg4: data.next().unwrap(), 
                    arg5: data.next().unwrap(), 
                })
            }
            "21" => {
                ops.push(Op::Speaker {
                    arg1: data.next().unwrap(),  // Speaker ID
                })
            }
            "22" => {
                ops.push(Op::ScreenFade {
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                })
            }
            "25" => {
                ops.push(Op::ChangeUi {
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(),
                })
            }
            "26" => {
                ops.push(Op::SetFlag {
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                })
            }
            "27" => {
                ops.push(Op::CheckCharacter {
                    arg1: data.next().unwrap(),
                })
            }
            "29" => {
                ops.push(Op::CheckObject {
                    arg1: data.next().unwrap(),
                })
            }
            "2a" => {
                ops.push(Op::SetLabel { // 2A
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                })
            }
            "2b" => {
                ops.push(Op::SetChoiceText {
                    arg1: data.next().unwrap(),
                })
            }
            "30" => {
                ops.push(Op::ShowBackground {
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                })
            }
            "33" => {
                ops.push(Op::ThreeThree {
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                    arg4: data.next().unwrap(),
                })
            }
            "34" => {
                ops.push(Op::GoToLabel {// 34
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                })
            }
            "35" => {
                let mut args: Vec<u8> = Vec::new();
                let mut idx = 0u8;

                while data.peek().unwrap() != &112u8 {
                    args.push(data.next().unwrap());
                    idx = idx + 1u8;
                }

                match args.len() {
                    4 => {
                        ops.push(Op::CheckFlagA4 { // 35
                            arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3],
                        });
                    }

                    19 => {
                        ops.push(Op::CheckFlagA19 { // 35
                            arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3], 
                            arg5: args[4], arg6: args[5], arg7: args[6], arg8: args[7], 
                            arg9: args[8], arg10: args[9], arg11: args[10], arg12: args[11], 
                            arg13: args[12], arg14: args[13], arg15: args[14], arg16: args[15], 
                            arg17: args[16], arg18: args[17], arg19: args[18],
                        });
                    }

                    24 => {
                        ops.push(Op::CheckFlagA24 { // 35
                            arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3], 
                            arg5: args[4], arg6: args[5], arg7: args[6], arg8: args[7], 
                            arg9: args[8], arg10: args[9], arg11: args[10], arg12: args[11], 
                            arg13: args[12], arg14: args[13], arg15: args[14], arg16: args[15], 
                            arg17: args[16], arg18: args[17], arg19: args[18], arg20: args[19],
                            arg21: args[20], arg22: args[21], arg23: args[22], arg24: args[23],
                        });
                    }

                    length => {
                        eyre::bail!("Invalid CheckFlagA Length: {}", length);
                    }
                }
            }
            "3a" => {
                ops.push(Op::WaitInput)
            }
            "3b" => {
                ops.push(Op::WaitFrame)
            }
            "3c" => {
                ops.push(Op::If_FlagCheck)
            }
            default => {
                for line in ops {
                    println!("{}", line);
                }
                eyre::bail!("Invalid opcode: {}", default);
            }
        }
    }
    println!("Parsed!");
    //let ops: Vec<Result<Op, &'static str>> = ops.into_iter().flatten().collect();

    let mut file = File::create("output.txt")?;

    for line in ops {
        write!(file, "{}\n", line);
    }
    
    println!("Writen!");

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn dgrlin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compile, m)?)?;
    Ok(())
}
