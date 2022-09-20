use std::io::stdin;
mod base_inst;
mod compressed;

fn main() {
    let mut buffer = String::new();
    let mut index;
    stdin().read_line(&mut buffer).expect("Failed to read line");
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