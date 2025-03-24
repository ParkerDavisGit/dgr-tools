use std::io::BufReader;
use std::fs::read;
use hex::ToHex;

// test.bin
// 70 00 BF 01 
// 70 22 01 00 01 
// 70 25 0B 01 
// 70 25 07 00 
// 70 25 10 00 
// 70 33 02 00 00 64 
// 70 33 03 00 00 64
//


// Operation    
// output -> 70 02 a1 a2 .... an
//
// ParsedCommand
// output -> command: a1, a2, ..., an

pub enum Operations {
    ZeroZero {
        opcode: u8,
        arg1: u8,
        arg2: u8,
    },
    TwoTwo {
        opcode: u8,
        arg1: u8,
        arg2: u8,
        arg3: u8,
    },
    TwoFive {
        opcode: u8,
        arg1: u8,
        arg2: u8,
    },
    ThreeThree {
        opcode: u8,
        arg1: u8,
        arg2: u8,
        arg3: u8,
        arg4: u8,
    },
}

fn main() {
    //let data = read("data/e00_004_003.bytecode").unwrap();
    let mut data = read("data/test.bin").unwrap().into_iter();
    let mut ops: Vec<Operations> = Vec::new();

    loop {
        let cmd = data.next();
        match cmd {
            None => {
                break;
            }
            Some(_) => {
            
            }
        }

        let cmd = data.next();
        match vec![cmd.unwrap()].encode_hex::<String>().as_str() {
            "00" => {
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
            }
            "22" => {
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
            }
            "25" => {
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
            }
            "33" => {
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
                let cmd = data.next().unwrap();
                print!("{} ", cmd);
            }
            _ => {
                println!("Invalid opcode!");
            }
        }
        println!("");
    }
}
