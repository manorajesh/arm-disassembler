use std::io::{stdin, Read};
use std::str;
mod base_inst;
mod compressed;

fn main() {
    let mut index;

    let mut buffer = {
        let mut bytes = Vec::new();

        for byte in stdin().bytes() {
            bytes.push(byte.unwrap());
        }

        let buffer = match str::from_utf8(&bytes) {
            Ok(v) => v.to_string(),
            Err(_) => {
                let mut buffer = String::new();
                for byte in bytes {
                    buffer.push(format!("{:x}", byte).parse().unwrap());
                }
                buffer
            }
        };
        buffer
    };

    buffer = buffer.trim().chars().rev().collect();

    loop {
        let inst = {
            index = 0;
            let mut inst = String::new();
            while index < 8 && buffer.len() != 0 {
                inst.push(buffer.pop().unwrap());
                index += 1;
            }
            if inst.ends_with("3") {
                u32::from_str_radix(&inst, 16).unwrap()
            } else {
                if inst.len() > 4 {
                    for _ in 0..4 {
                        buffer.push(inst.pop().unwrap());
                    }
                }
                u32::from_str_radix(&inst, 16).unwrap()
            }
        };

        let line;
        if inst & 0b11 == 0b11 {
            line = base_inst::base_inst_from(inst);
        } else {
            line = compressed::rvc_from(inst as u16);
        }

        if line == String::new() {
            println!("db {:b}", inst);
        } else {
            println!("{}", line);
        }

        if buffer.len() == 0 {
            break;
        }
    }
}
