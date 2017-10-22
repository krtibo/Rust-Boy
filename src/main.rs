pub mod cpu;
pub mod test;
use cpu::*;
use test::*;


fn main() {
    

    
    // let mut rust_boy : CPU = CPU::new();
    // rust_boy.load_rom(String::from("./rom/test.gb"));

    /*
    rust_boy.load_rom(String::from("/home/tiibo/Cloud/PROGRAMMING/Rust_Boy/rom/boot_rom.gb"));
    rust_boy.cycle();
    */

    let mut rb : CPU = CPU::new();
    let mut t : Test = Test::new(rb);
    t.test_parser();
}