pub mod cpu;
use cpu::*;


fn main() {
    let mut rust_boy : CPU = CPU::new();
    rust_boy.load_rom(String::from("./rom/test.gb"));
    rust_boy.cycle();
}
