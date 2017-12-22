#![allow(unused_variables)]

pub mod cpu;
pub mod test;
pub mod ppu;
use cpu::*;
use test::*;



fn main() {



    let mut rust_boy : CPU = CPU::new();
    //rust_boy.load_rom(String::from("./rom/test.gb"));


    //rust_boy.load_bootrom(String::from("./rom/boot_rom.gb"));
    rust_boy.load_rom(String::from("./rom/mario.gb"));
    //rust_boy.load_rom(String::from("./rom/test_roms/04-op r,imm.gb"));
    rust_boy.cycle();


    let t : Test = Test::new(rust_boy);
    // t.test_parser();
}
