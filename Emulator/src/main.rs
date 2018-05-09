/*************************************************************************

                    ===    THIS IS RUST BOY    ===

    This is the main file, the RUST BOY's entry point. It handles the
    command line arguments, and passes the paths and options to the CPU.

*************************************************************************/

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
        println!("╭─────────────────────────────────────────────────────╮");
        println!("╞══════════════    THIS IS RUST BOY    ═══════════════╡");
        println!("├─────────────────────────────────────────────────────┤");
        println!("│ USAGE:                                              │");
        println!("│                                                     │");
        println!("│ If BOOT ROM is supplied, and placed next to the     │");
        println!("│ executable:                                         │");
        println!("│                                                     │");
        println!("│ ./rust_boy /path/rom.gb -X1                         │");
        println!("│                                                     │");
        println!("│ * 1st argument is the path of the Game ROM          │");
        println!("│ * 2nd argument is the scale constant (X1, X2, X4)   │");
        println!("│                                                     │");
        println!("├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤");
        println!("│ If BOOT ROM is not supplied:                        │");
        println!("│                                                     │");
        println!("│ ./rust_boy /path/boot_rom.gb /path/rom.gb -X1       │");
        println!("│                                                     │");
        println!("│ * 1st argument is the path of the BOOT ROM          │");
        println!("│ * 2nd argument is the path of the Game ROM          │");
        println!("│ * 3rd argument is the scale constant (X1, X2, X4)   │");
        println!("│                                                     │");
        println!("╰─────────────────────────────────────────────────────╯");
        return
    }

    if args.len() == 3 {
        let path = args[1].clone();

        match args[2].as_ref() {
            "-X1" => rust_boy = CPU::new(String::from("boot_rom.gb"), path, 1),
            "-X2" => rust_boy = CPU::new(String::from("boot_rom.gb"), path, 2),
            "-X4" => rust_boy = CPU::new(String::from("boot_rom.gb"), path, 4),
            _    => {
                println!("* Your argument is invalid: {:?}  *", args[2]);
                return
            }
        }

        rust_boy.load_bootrom();
        rust_boy.load_rom();
        rust_boy.cycle();
    }

    if args.len() == 4 {
        let path = args[2].clone();
        let bootrom_path = args[1].clone();

        match args[3].as_ref() {
            "-X1" => rust_boy = CPU::new(bootrom_path, path, 1),
            "-X2" => rust_boy = CPU::new(bootrom_path, path, 2),
            "-X4" => rust_boy = CPU::new(bootrom_path, path, 4),
            _    => {
                println!("* Your argument is invalid: {:?}  *", args[2]);
                return
            }
        }

        rust_boy.load_bootrom();
        rust_boy.load_rom();
        rust_boy.cycle();
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
