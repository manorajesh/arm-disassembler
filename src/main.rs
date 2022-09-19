use std::io::stdin;
mod compressed;
mod base_inst;

fn main() {
    let mut buffer = String::new();
    stdin()
    .read_line(&mut buffer)
    .expect("Failed to read line");

    let inst = u32::from_str_radix(&buffer[..buffer.len()-2], 16).expect("Failed to parse hex");

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
}
