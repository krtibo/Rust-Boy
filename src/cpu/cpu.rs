use std::io::prelude::*;
use std::fs::File;
use std::collections::LinkedList;
use opcode::Opcode;
use cpu::debugger::Debugger;
use cpu::debugger::DebugData;
use std::{thread, time};
use ppu::PPU;
extern crate minifb;
use self::minifb::{Key, KeyRepeat, Window, WindowOptions, Scale};
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
    pub IR : bool,
    pub RAM : [u8; 65536],
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
            IR : false,
            RAM : [0; 65536],
            STACK : LinkedList::new(),
        }
    }



    pub fn cycle(&mut self) {
        let mut cycle : u32 = 0;
        let mut opcode : Opcode = Opcode::new();
        /* let mut debugger : Debugger = Debugger::new(); */
        let mut debug_data : DebugData = DebugData::new();
        /* let mut vram : VRAM = VRAM::new(); */
        let mut ppu : PPU = PPU::new();

        opcode.init();

        loop {
            while cycle < CYCLES && self.PC <= 65535 {
                // fetch and decode opcode
                let instr_time : u8 = opcode.execute(self);
                cycle += instr_time as u32;
                ppu.update_n_sync(self, instr_time);
                

                let data = self.assemble_debug_data(opcode.last_instruction.to_string(),
                                                    opcode.last_opcode,
                                                    opcode.lhs,
                                                    opcode.rhs,
                                                    opcode.operand_mode);


                debug_data.parse_data_from_cpu(data);
                 
                //thread::sleep(time::Duration::from_millis(100));

/*                  if debugger.window.is_key_pressed(Key::Space, KeyRepeat::No) {
                    loop {
                        debugger.update_window(&debug_data);
                        if debugger.window.is_key_pressed(Key::Space, KeyRepeat::No) {
                            break;
                        }
                    }
                } */

            }
            //debugger.update_window(&debug_data);
            ppu.render();
            /* vram.print_vram(self); */
            //println!("END OF THE CYCLE ------------------------");
            cycle = 0;
        } 
        
/*
        while debugger.window.is_open() && !debugger.window.is_key_down(Key::Escape){
            debugger.update_window(&debug_data);
        }
        */
        // render screen
    }



    pub fn load_bootrom(&mut self, path : String) {
        let mut rom_buffer : Vec<u8> = Vec::new();
        let mut f = File::open(path)
        .expect("Error with file loading!");

        f.read_to_end(&mut rom_buffer)
        .expect("Error with file reading!");

        let mut count = 0;
        for i in 0..rom_buffer.len() {
            count += 1;
            //print!("{:02X} ", &rom_buffer[i]);
            if count == 16 {
                //println!();
                count = 0;
            }
        }
        println!("\nBOOT ROM length (in bytes): {}", rom_buffer.len());

        for i in 0..rom_buffer.len() {
            self.RAM[i] = rom_buffer[i];
        }
        println!("BOOT ROM copying done!");
    }

    pub fn load_rom(&mut self, path : String) {
        let mut rom_buffer : Vec<u8> = Vec::new();
        let mut f = File::open(path)
        .expect("Error with file loading!");

        f.read_to_end(&mut rom_buffer)
        .expect("Error with file reading!");

        let mut count = 0;
        for i in 0x100..rom_buffer.len() {
            count += 1;
            //print!("{:02X} ", &rom_buffer[i]);
            if count == 16 {
                //println!();
                count = 0;
            }
        }
        println!("\nROM length (in bytes): {}", rom_buffer.len());

        for i in 0x0100..rom_buffer.len() {
            self.RAM[i] = rom_buffer[i];
        }
        println!("ROM copying done!");
    }


#[allow(dead_code)]
    pub fn write_ram(&mut self, address : u16, value : u8) {
        self.RAM[address as usize] = value;
    }


    pub fn assemble_debug_data(&mut self, 
        last_instruction : String, 
        last_opcode : u8,
        lhs : u16, 
        rhs : u16, 
        operand_mode : u8) -> (String, Vec<String>) {

           let actual_reg : Vec<String> = vec![
           format!("0x{:02X}", self.A),
           format!("0x{:02X}", self.B),
           format!("0x{:02X}", self.C),
           format!("0x{:02X}", self.D),
           format!("0x{:02X}", self.E),
           format!("0x{:02X}", self.F),
           format!("0x{:02X}", self.H),
           format!("0x{:02X}", self.L),
           format!("0x{:02X}", self.SP),
           format!("0x{:02X}", self.PC),
           format!("0b{:04b}", self.FLAG >> 4),
           format!("{}", self.STACK.len()),
           ];

           

           if operand_mode == 0 {
               /* println!("{}",format!("0x{:02X} : {}", last_opcode, last_instruction)); */
               (format!("0x{:02X} : {}", last_opcode, last_instruction), actual_reg)
           } else
           if operand_mode == 1 {
/*                println!("{}",format!("0x{:02X} : {} 0x{:X}", 
                                    last_opcode, 
                                    last_instruction, 
                                    rhs)); */
               (format!("0x{:02X} : {} 0x{:X}", last_opcode, last_instruction, rhs), actual_reg)
           } else {
/*                println!("{}",format!("0x{:02X} : {} 0x{:X} 0x{:X}", 
               last_opcode, 
               last_instruction, 
               lhs, 
               rhs)); */
               (format!("0x{:02X} : {} 0x{:X} 0x{:X}", last_opcode, last_instruction, lhs, rhs), actual_reg)
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

    pub fn get_flag(&self, f : &str) -> u8 {

        if f == "Z" {
            if self.FLAG & 0b1000_0000 == 0 {
                return 0
            } else {
                return 1
            }
        } 

        if f == "N" {
            if self.FLAG & 0b0100_0000 == 0 {
                return 0
            } else {
                return 1
            }
        } 

        if f == "H" {
            if self.FLAG & 0b0010_0000 == 0 {
                return 0
            } else {
                return 1
            }
        }  

        if f == "C" {
            if self.FLAG & 0b0001_0000 == 0 {
                return 0
            } else {
                return 1
            }
        }

        0
    }


    pub fn test_bytes(&mut self, bytes : &[u8]) -> (String, Vec<String>) {
        self.PC = 0;
        let mut index = 0;
        for i in bytes {
            self.RAM[index] = *i;
            index += 1;
        }

        let mut opcode : Opcode = Opcode::new();
        opcode.init();
        opcode.execute(self) as u32;
        let data = self.assemble_debug_data(opcode.last_instruction.to_string(),
                                    opcode.last_opcode,
                                    opcode.lhs,
                                    opcode.rhs,
                                    opcode.operand_mode);
        println!("{}", data.0);
    
        data
    }

}

























pub struct VRAM {
    pub vram : [u32; 128*64],
    pub window : Window,
}

impl VRAM {

    pub fn new() -> VRAM {

        VRAM {
            vram : [0; 128*64],
            window : Window::new("VRAM Map",
                         128,
                         64,
                         WindowOptions {
                             resize: false,
                             scale: Scale::X4,
                             ..WindowOptions::default()})
                             .unwrap(),
        }
    }

    pub fn print_vram(& mut self, cpu : &CPU) {

        for i in 0x8000..0xA000 {
            
            if cpu.RAM[i] > 0 {
                //println!("{:04X} : {:02X}", i, cpu.RAM[i]);
                self.vram[i - 0x8000] = 0xFF_00_FF_00;
                //print!("{:X} ", cpu.RAM[i]);
            } else {

                if i >= 0x9800 && i <= 0x9BFF {
                    self.vram[i - 0x8000] = 0xFF_FF_00_00;
                    
                } 

                if i >= 0x8000 && i <= 0x8FFF {
                    self.vram[i - 0x8000] = 0xFF_00_00_FF;
                } 
            }


        }
        self.vram[0x9800-0x8000] = 0xFF_FF_00_00;
        self.window.update_with_buffer(&self.vram).unwrap();
    }

}
