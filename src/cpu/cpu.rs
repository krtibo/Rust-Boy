use std::io::prelude::*;
use std::fs::File;
use opcode::Opcode;
#[allow(dead_code)]

const CYCLES : u32 = 69905; // 4194304 (clock cycle) / 60

#[allow(non_snake_case)]
pub struct CPU {
    pub A : u8,
    pub B : u8,
    pub C : u8,
    pub D : u8,
    pub E : u8,
    pub F : u8,
    pub H : u8,
    pub L : u8,
    pub SP : u16,
    pub PC : u16,
    pub FLAG : u8,
    // C (carry), H (half carry), N (substract), Z (zero)
    // C H N Z
    pub RAM : [u8; 8192],
    pub STACK : Vec<u16>,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            A : 0,
            B : 0,
            C : 0,
            D : 0,
            E : 0,
            F : 0,
            H : 0,
            L : 0,
            SP : 0xFFFE,
            PC : 0,
            FLAG : 0,
            RAM : [0; 8192],
            STACK : Vec::new(),
        }
    }



    pub fn cycle(&mut self) {
        let mut cycle : u32 = 0;
        let mut opcode : Opcode = Opcode::new();
        opcode.init();


        while cycle < CYCLES && self.PC < 8192 {
            // fetch and decode opcode
            //opcode.fetch(self);
            cycle += opcode.execute(self) as u32;
            let (a, b) = self.assemble_debug_data(opcode.last_instruction.to_string());
            println!("Program Counter: {} \n Last instruction: {} \n Registers: {:?}", self.PC, a, b);

        }
        // render screen
    }



    pub fn load_rom(&mut self, path : String) {
        let mut rom_buffer : Vec<u8> = Vec::new();
        let mut f = File::open(path)
        .expect("Error with file loading!");

        f.read_to_end(&mut rom_buffer)
        .expect("Error with file reading!");

        for i in 0..rom_buffer.len() {
            print!("{:x} ", &rom_buffer[i]);
            if i % 16 == 0 && i > 0 {
                println!();
            }
        }
        println!("\nROM length (in bytes): {}", rom_buffer.len());

        for i in 0..rom_buffer.len() {
            self.RAM[i] = rom_buffer[i];
        }
        println!("ROM copying done!");
    }


#[allow(dead_code)]
    pub fn read_ram(&self, address : u16) -> u8 {
        self.RAM[address as usize]
    }


    pub fn assemble_debug_data(&self, last_instruction : String) -> (String, Vec<String>) {

           let actual_reg : Vec<String> = vec![
           format!("0x{:X}", self.A),
           format!("0x{:X}", self.B),
           format!("0x{:X}", self.C),
           format!("0x{:X}", self.D),
           format!("0x{:X}", self.E),
           format!("0x{:X}", self.F),
           format!("0x{:X}", self.H),
           format!("0x{:X}", self.L),
           format!("0x{:X}", self.SP),
           format!("0x{:X}", self.PC),
           format!("0b{:b}", self.FLAG),
           format!("{}", self.STACK.len()),
           ];

       (last_instruction, actual_reg)
    }

}
