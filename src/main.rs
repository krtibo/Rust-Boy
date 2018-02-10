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

    // tests
    //rust_boy.load_rom(String::from("./rom/test_roms/01-special.gb"));             PASSED!
    //rust_boy.load_rom(String::from("./rom/test_roms/02-interrupts.gb"));          FAILED!
    //rust_boy.load_rom(String::from("./rom/test_roms/03-op sp,hl.gb"));            PASSED!
    //rust_boy.load_rom(String::from("./rom/test_roms/04-op r,imm.gb"));            PASSED!
    //rust_boy.load_rom(String::from("./rom/test_roms/05-op rp.gb"));               PASSED!
    //rust_boy.load_rom(String::from("./rom/test_roms/06-ld r,r.gb"));              PASSED!
    //rust_boy.load_rom(String::from("./rom/test_roms/07-jr,jp,call,ret,rst.gb"));  PASSED!
    //rust_boy.load_rom(String::from("./rom/test_roms/08-misc instrs.gb"));         PASSED!
    //rust_boy.load_rom(String::from("./rom/test_roms/09-op r,r.gb"));              PASSED!
    //rust_boy.load_rom(String::from("./rom/test_roms/10-bit ops.gb"));             PASSED!
    //rust_boy.load_rom(String::from("./rom/test_roms/11-op a,(hl).gb"));           PASSED!


    rust_boy.cycle();


    let t : Test = Test::new(rust_boy);
    // t.test_parser();
}
