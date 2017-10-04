pub mod cpu;
use cpu::*;


fn main() {
    

    
    let mut rust_boy : CPU = CPU::new();
    // rust_boy.load_rom(String::from("./rom/test.gb"));
    rust_boy.load_rom(String::from("./rom/boot_rom.gb"));
    rust_boy.cycle();
    

    /* 
    
    -- 8-BIT HALF CARRY DETECTING EXAMPLE --

    let a : u8 = 0b00111100;
    let b : u8 = 0b00010000;

    if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
        println!("HALF CARRY DUDE");
    } 
    else {
        println!("NO HALF CARRY");
    }

    
    -- 16-BIT HALF CARRY DETECTING EXAMPLE --

    let a : u16 = 0b0000_1000_0000_0000;
    println!("{:b}", a);
    let b : u16 = 0b0000_1000_0000_0000;
    println!("{:b}", b);

    if (((a & 0x0fff) + (b & 0x0fff)) & 0x1000) == 0x1000 {
        println!("HALF CARRY DUDE");
    } 
    else {
        println!("NO HALF CARRY");
    }

    */

}
