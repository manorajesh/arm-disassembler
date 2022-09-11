fn decode_c_addi(inst: u16) -> String {
    println!("addi a5,a5,{}", (inst & 0b0000000001111100) >> 2);

    let mut dis_inst = String::new(); // disassembled instruction
    
    // funct
    dis_inst.push_str(get_funct(inst & 0b1110000000000000).as_str());

    // rd
    dis_inst.push_str(get_reg(inst & 0b0000111110000000).as_str());
    dis_inst.push_str(",");
    dis_inst.push_str(get_reg(inst & 0b0000111110000000).as_str());

    // incomplete

    dis_inst
    /*
    0000011110000101
    &   0000000001111100
    =   0000000000000100
    */
}

fn get_reg(inst: u16) -> String {
    let mut reg = String::new();
    match inst {
        000000 => reg.push_str("zero "),
        000001 => reg.push_str("ra "),
        000010 => reg.push_str("sp "),
        000011 => reg.push_str("gp "),
        000100 => reg.push_str("tp "),
        000101 => reg.push_str("t0 "),
        001000 => reg.push_str("s0 "),
        001010 => reg.push_str("a0 "),
        001011 => reg.push_str("a1 "),
        001100 => reg.push_str("a2 "),
        001101 => reg.push_str("a3 "),
        001110 => reg.push_str("a4 "),
        001111 => reg.push_str("a5 "),
        _ => panic!("Invalid register"),
    }
    reg
}

fn get_funct(inst: u16) -> String {
    let mut funct = String::new();
    match inst {
        000 => funct.push_str("addi "),
        _ => panic!("Invalid funct"),
    }
    funct
}

fn main() {
    decode_c_addi(0x078d); //addi a5,a5,01
    decode_c_addi(0x0789); //addi a5,a5,02
}

/*
C.ADDI
num + 01 addi a5,a5,01 0785 000 0 01111 00001 01
num + 02 addi a5,a5,02 0789 000 0 01111 00010 01
num + 03 addi a5,a5,03 078d 000 0 01111 00011 01
num + 04 addi a5,a5,04 0791 000 0 01111 00100 01
num + 05 addi a5,a5,05 0795 000 0 01111 00101 01
*/
