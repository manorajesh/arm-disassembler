fn decode_rio(inst: u16) -> String { // Integer Register-Immediate Operation
    let mut dis_inst = String::new(); // disassembled instruction
    
    // funct
    dis_inst.push_str(get_funct(inst & 0b1110000000000000).as_str());
    dis_inst.push_str(" ");

    // rd
    dis_inst.push_str(get_reg((inst & 0b0000111110000000) >> 7).as_str()); // same
    dis_inst.push_str(",");
    dis_inst.push_str(get_reg((inst & 0b0000111110000000) >> 7).as_str()); // same

    // imm
    dis_inst.push_str(",");
    let mut imm = ((inst & 0b0000000001111100) >> 2) as i32;
    if (inst & 0b0001000000000000) >> 12 != 0{
        imm = -imm;
    }
    dis_inst.push_str((imm).to_string().as_str());

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
        0b00000 => reg.push_str("zero"),
        0b00001 => reg.push_str("ra"),
        0b00010 => reg.push_str("sp"),
        0b00011 => reg.push_str("gp"),
        0b00100 => reg.push_str("tp"),
        0b00101 => reg.push_str("t0"),
        0b01000 => reg.push_str("s0"),
        0b01010 => reg.push_str("a0"),
        0b01011 => reg.push_str("a1"),
        0b01100 => reg.push_str("a2"),
        0b01101 => reg.push_str("a3"),
        0b01110 => reg.push_str("a4"),
        0b01111 => reg.push_str("a5"),
        _ => {
            if inst > 0b100000 { // 32
                reg.push_str("x");
                reg.push_str((0b100000 - inst).to_string().as_str());
            } else {
                panic!("Invalid register")
            }
        },
    }
    reg
}

fn get_funct(inst: u16) -> String {
    let mut funct = String::new();
    match inst {
        0b000 => funct.push_str("addi"),
        _ => panic!("Invalid funct"),
    }
    funct
}

fn main() {
    println!("{}", decode_rio(0x078d)); //addi a5,a5,01
    println!("{}", decode_rio(0x0789)); //addi a5,a5,02
    println!("{}", decode_rio(0x057d)); //addi a0,a0,31
    println!("{}", decode_rio(0x1141)); //addi sp,sp,-16
}

/*
C.ADDI
num + 01 addi a5,a5,01 0785 000 0 01111 00001 01
num + 02 addi a5,a5,02 0789 000 0 01111 00010 01
num + 03 addi a5,a5,03 078d 000 0 01111 00011 01
num + 04 addi a5,a5,04 0791 000 0 01111 00100 01
num + 05 addi a5,a5,05 0795 000 0 01111 00101 01
*/
