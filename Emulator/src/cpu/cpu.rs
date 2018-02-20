#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use std::io::prelude::*;
use std::fs::File;
use std::collections::LinkedList;
use opcode::Opcode;
use cpu::debugger::Debugger;
use cpu::debugger::DebugData;
use ppu::PPU;
use timer::Timer;
use joypad::Joypad;
use interrupt::Interrupt;
use memorymap::MemoryMap;
extern crate minifb;
use self::minifb::{Key, KeyRepeat};

const CYCLES : u32 = 69905; // 4194304 (clock cycle) / 60

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
    pub IR : bool,
    pub RAM : [u8; 65536],
    pub STACK : LinkedList<u8>,
    freq_change : bool,
    boot_rom : bool,
    pub boot_rom_path : String,
    pub rom_path : String,
    pub options : u8,
}

impl CPU {

    pub fn new(path_b : String, path_r : String, o : u8) -> CPU {
        CPU {
            A : 0x01,
            B : 0x00,
            C : 0x13,
            D : 0x00,
            E : 0xD8,
            F : 0xB0,
            H : 0x01,
            L : 0x4D,
            SP : 0xFFFE,
            PC : 0x00,
            IR : false,
            RAM : [0; 65536],
            STACK : LinkedList::new(),
            freq_change : false,
            boot_rom : false,
            boot_rom_path : path_b,
            rom_path : path_r,
            options : o,
        }
    }

    pub fn cycle(&mut self) {
        let mut cycle : u32 = 0;
        let mut opcode : Opcode = Opcode::new();
        let mut debugger : Debugger = Debugger::new();
        let mut debug_data : DebugData = DebugData::new();
        let mut mem : MemoryMap = MemoryMap::new();
        let mut ppu : PPU = PPU::new();
        let mut timer : Timer = Timer::new();
        let mut joypad : Joypad = Joypad::new();
        let mut interrupt : Interrupt = Interrupt::new();
        opcode.init();

        // main loop - this repeats forever
        loop {

            // cycle loop - one "lap" equals one CPU operation @60 FPS
            while cycle < CYCLES && self.PC <= 65534 {

                // if the boot ROM has been run, copy the cartridge ROM header to its place
                if self.PC == 0x100 && self.boot_rom == false {
                    self.boot_rom = true;
                    self.load_rom_header();
                }

                // scan for any pressed button BEFORE any operation begins
                joypad.scan_window_button_pressed(&ppu.window, self);
                // update the 0xFF00 register with the actual joypad state
                self.RAM[0xFF00] = joypad.update_state(self);

                // fetch and do the instruction
                let instr_time : u8 = opcode.execute(self);
                // add the instuction time to the cycle counter
                cycle += instr_time as u32;
                // synchronize the PPU
                ppu.update_n_sync(self, instr_time);

                // assemble the instruction specs and give them to the debugger
                let data =
                    self.assemble_debug_data(opcode.last_instruction.to_string(),
                    opcode.last_opcode,
                    opcode.lhs,
                    opcode.rhs,
                    opcode.operand_mode);

                debug_data.parse_data_from_cpu(data);

                // if the user stops the emulator with the space button,
                // go in a loop and render the debugger window continuously
                if debugger.window.is_key_pressed(Key::Space, KeyRepeat::No){
                    loop {
                        debugger.update_window(&debug_data);
                        mem.print_ram(self);

                        // if the left control is pressed, resume the emulator
                        if debugger.window.is_key_pressed(Key::LeftCtrl, KeyRepeat::No) {
                            break
                        }
                    }
                }

                // frequency change request handler
                if self.freq_change {
                    timer.update_freq(self);
                    self.freq_change = false;
                }

                // check and handle the interrupts
                //self.interrupt_checker();
                interrupt.interrupt_checker(self);
                // update the timers
                timer.update(self, instr_time);

            } // cycle loop

            // update the debugger, memory map and render the PPU VRAM
            debugger.update_window(&debug_data);
            mem.print_ram(self);
            ppu.render();
            cycle = 0;

        } // main loop

    } // fn cycle

    pub fn load_bootrom(&mut self) {
        let mut rom_buffer : Vec<u8> = Vec::new();

        // open the Boot ROM
        let mut f = File::open(&self.boot_rom_path)
        .expect("Error with file loading!");

        // read the Boot ROM to rom_buffer
        f.read_to_end(&mut rom_buffer)
        .expect("Error with file reading!");

        // copy rom_buffer to RAM 0x0..0x100
        for i in 0..rom_buffer.len() {
            self.RAM[i] = rom_buffer[i];
        }

    }

    pub fn load_rom(&mut self) {
        let mut rom_buffer : Vec<u8> = Vec::new();
        let mut f = File::open(&self.rom_path)
        .expect("Error with file loading!");

        f.read_to_end(&mut rom_buffer)
        .expect("Error with file reading!");

        for i in 0x100..rom_buffer.len() {
            self.RAM[i] = rom_buffer[i];
        }
        println!("ROM copying done! {:x}", self.RAM[0x65]);
    }

    pub fn load_rom_header(&mut self) {
        let mut rom_buffer : Vec<u8> = Vec::new();
        let mut f = File::open(&self.rom_path)
        .expect("Error with file loading!");

        f.read_to_end(&mut rom_buffer)
        .expect("Error with file reading!");

        for i in 0..0x100 {
            self.RAM[i] = rom_buffer[i];
        }
    }

    pub fn write_ram(&mut self, address : u16, value : u8) {

        if address == 0xFF02 && value == 0x81 {
            print!("{}", self.RAM[0xFF01] as char);
        }

        if address == 0xFF46 {
            //println!("DMA!");
            self.dma(value);
        }

        if address == 0xFF07 {
            let old_freq = self.RAM[0xFF07] & 0x3;
            self.RAM[address as usize] = value;
            let new_freq = self.RAM[0xFF07] & 0x3;

            if old_freq != new_freq {
                self.freq_change = true;
            }
            return
        }

        if address == 0xFF04 || address == 0xFF44 {
            self.RAM[address as usize] = 0;
            return
        }

        self.RAM[address as usize] = value;
    }

    pub fn dma(&mut self, value : u8) {
        let addr : u16 = (value as u16) << 8;

        for i in 0..0xA0 {
            let n : u8 = self.RAM[(addr + i) as usize];
            self.write_ram(0xFE00 + i, n);
        }
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
       format!("0b{:04b}", self.F >> 4),
       format!("{}", self.STACK.len()),
       ];

       if operand_mode == 0 {
           (format!("0x{:02X} : {}", last_opcode, last_instruction), actual_reg)
       } else
       if operand_mode == 1 {
           (format!("0x{:02X} : {} 0x{:X}", last_opcode, last_instruction, rhs), actual_reg)
       } else {
           (format!("0x{:02X} : {} 0x{:X} 0x{:X}", last_opcode, last_instruction, lhs, rhs), actual_reg)
       }
    }

    pub fn set_flag(&mut self, f : &str) {

        if f == "Z" {
            self.F |= 0b1000_0000;
        }

        if f == "N" {
            self.F |= 0b0100_0000;
        }

        if f == "H" {
            self.F |= 0b0010_0000;
        }

        if f == "C" {
            self.F |= 0b0001_0000;
        }
    }

    pub fn reset_flag(&mut self, f : &str) {

        if f == "Z" {
            self.F &= 0b0111_1111;
        }

        if f == "N" {
            self.F &= 0b1011_1111;
        }

        if f == "H" {
            self.F &= 0b1101_1111;
        }

        if f == "C" {
            self.F &= 0b1110_1111;
        }
    }

    pub fn get_flag(&self, f : &str) -> u8 {

        if f == "Z" {
            if self.F & 0b1000_0000 == 0 {
                return 0
            } else {
                return 1
            }
        }

        if f == "N" {
            if self.F & 0b0100_0000 == 0 {
                return 0
            } else {
                return 1
            }
        }

        if f == "H" {
            if self.F & 0b0010_0000 == 0 {
                return 0
            } else {
                return 1
            }
        }

        if f == "C" {
            if self.F & 0b0001_0000 == 0 {
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

    pub fn set_bit(n : u8, reg : u8) -> u8 {
        let mut value : u8 = 0;
        let mask : u8 = 1 << n;
        value = reg | mask;
        value
    }

    pub fn reset_bit(n : u8, reg : u8) -> u8 {
        let mut value : u8 = 0;
        let mask : u8 = 1 << n;
        value = reg & (0xFF - mask);
        value
    }

    pub fn get_bit(n : u8, reg : u8) -> bool {
        let mask : u8 = 1 << n;

        if reg & mask == 0 {
            false
        } else {
            true
        }
    }

}
