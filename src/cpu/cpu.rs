use std::io::prelude::*;
use std::fs::File;
use std::collections::LinkedList;
use opcode::Opcode;
use cpu::debugger::Debugger;
use cpu::debugger::DebugData;
use std::{thread, time};
extern crate minifb;
use self::minifb::{Key, KeyRepeat};
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
    pub FLAG : u8, // ZNHC0000
    // C (carry), H (half carry), N (substract), Z (zero)
    
    pub RAM : [u8; 65535],
    pub STACK : LinkedList<u8>,
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
            RAM : [0; 65535],
            STACK : LinkedList::new(),
        }
    }



    pub fn cycle(&mut self) {
        let mut cycle : u32 = 0;
        let mut opcode : Opcode = Opcode::new();
        let mut debugger : Debugger = Debugger::new();
        let mut debug_data : DebugData = DebugData::new();
        opcode.init();


        while cycle < CYCLES && self.PC < 65535 {
            // fetch and decode opcode
            //opcode.fetch(self);
            cycle += opcode.execute(self) as u32;

            let data = self.assemble_debug_data(opcode.last_instruction.to_string(),
                                                opcode.lhs,
                                                opcode.rhs,
                                                opcode.operand_mode);

            debug_data.parse_data_from_cpu(data);
            debugger.update_window(&debug_data);
            thread::sleep(time::Duration::from_millis(100));

            if debugger.window.is_key_pressed(Key::P, KeyRepeat::No) {
                loop {
                    debugger.update_window(&debug_data);
                    if debugger.window.is_key_pressed(Key::P, KeyRepeat::No) {
                        break;
                    }
                }
            }


        }

        while debugger.window.is_open() && !debugger.window.is_key_down(Key::Escape){
            debugger.update_window(&debug_data);
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


    pub fn assemble_debug_data(&self, last_instruction : String, lhs : u16, rhs : u16, operand_mode : u8) -> (String, Vec<String>) {

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

           println!("{}",format!("{} 0x{:X} 0x{:X}", last_instruction, lhs, rhs));

           if operand_mode == 0 {
               (last_instruction, actual_reg)
           } else
           if operand_mode == 1 {
               (format!("{} 0x{:X}", last_instruction, rhs), actual_reg)
           } else {
               (format!("{} 0x{:X} 0x{:X}", last_instruction, lhs, rhs), actual_reg)
           }
    }


    pub fn set_flag(&mut self, f : &str) {

        if f == "Z" {
            self.FLAG |= 0b1000_0000;
        }

        if f == "N" {
            self.FLAG |= 0b0100_0000;
        }

        if f == "H" {
            self.FLAG |= 0b0010_0000;
        }

        if f == "C" {
            self.FLAG |= 0b0001_0000;
        }
    }


    pub fn reset_flag(&mut self, f : &str) {

        if f == "Z" {
            self.FLAG &= 0b0111_1111;
        }

        if f == "N" {
            self.FLAG &= 0b1011_1111;
        }

        if f == "H" {
            self.FLAG &= 0b1101_1111;
        }

        if f == "C" {
            self.FLAG &= 0b1110_1111;
        }
    }

}
