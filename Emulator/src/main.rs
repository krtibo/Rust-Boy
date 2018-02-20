#![allow(unused_variables)]

pub mod cpu;
pub mod test;
pub mod ppu;
use cpu::*;
use std::env;

fn main() {

    let mut rust_boy : CPU;
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("* * * * * * * * * * * * * * * * * * * * * * * * * * * *");
        println!("* No ROM supplied, please insert a ROM via argument!  *");
        println!("* * * * * * * * * * * * * * * * * * * * * * * * * * * *");
        return
    }

    if args.len() == 2 && args[1] != "-d" && args[1] != "-r" {
        let path = args[1].clone();
        rust_boy = CPU::new(String::from("./rom/boot_rom.gb"), path, 1);
        rust_boy.load_bootrom();
        rust_boy.load_rom();
        rust_boy.cycle();
    }

    if args.len() == 3 {
        let path = args[1].clone();

        match args[2].as_ref() {
            "-d" => rust_boy = CPU::new(String::from("./rom/boot_rom.gb"), path, 1),
            "-r" => rust_boy = CPU::new(String::from("./rom/boot_rom.gb"), path, 2),
            _    => {
                println!("* * * * * * * * * * * * * * * * * *");
                println!("* Your argument is invalid: {:?}  *", args[2]);
                println!("* * * * * * * * * * * * * * * * * *");
                return
            }
        }

        rust_boy.load_bootrom();
        rust_boy.load_rom();
        rust_boy.cycle();
    }

    if args.len() == 4 {
        let path = args[1].clone();

        if (args[2] == "-d" && args[3] == "-r") || (args[2] == "-r" && args[3] == "-d") {
            rust_boy = CPU::new(String::from("./rom/boot_rom.gb"), path, 3);
            rust_boy.load_bootrom();
            rust_boy.load_rom();
            rust_boy.cycle();
        } else {
            println!("* * * * * * * * * * * * * * * * * * * * *");
            println!("* Your arguments are invalid: {:?} {:?} *", args[2], args[3]);
            println!("* * * * * * * * * * * * * * * * * * * * *");
            return
        }
    }


/*
    let mut rust_boy : CPU = CPU::new();
    rust_boy.load_bootrom(String::from("./rom/boot_rom.gb"));
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
*/
}
