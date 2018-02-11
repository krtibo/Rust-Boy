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
use std::{thread, time};
use ppu::PPU;
use timer::Timer;
use joypad::Joypad;
extern crate minifb;
use self::minifb::{Key, KeyRepeat, Window, WindowOptions, Scale, MouseMode, MouseButton};

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
    pub FLAG : u8, // ZNHC0000
    // C (carry), H (half carry), N (substract), Z (zero)
    pub IR : bool,
    pub RAM : [u8; 65536],
    pub STACK : LinkedList<u8>,
    freq_change : bool,
    boot_rom : bool,
    pub path : String,
}

impl CPU {
    pub fn new() -> CPU {
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
            PC : 0x100,
            FLAG : 0,
            IR : false,
            RAM : [0; 65536],
            STACK : LinkedList::new(),
            freq_change : false,
            boot_rom : false,
            path : String::from(""),
        }
    }



    pub fn cycle(&mut self) {
        let mut cycle : u32 = 0;
        let mut opcode : Opcode = Opcode::new();
        let mut debugger : Debugger = Debugger::new();
        let mut debug_data : DebugData = DebugData::new();
        let mut vram : VRAM = VRAM::new();
        let mut ppu : PPU = PPU::new();
        let mut timer : Timer = Timer::new();
        let mut joypad : Joypad = Joypad::new();
        let mut speed = 100;
        opcode.init();

        loop {
            while cycle < CYCLES && self.PC <= 65534 {

                if self.PC == 0x100 && self.boot_rom == false {
                    self.boot_rom = true;
                    //self.load_rom_header();
                }


                // fetch and decode opcode
                joypad.scan_window_button_pressed(&ppu.window, self);
                self.RAM[0xFF00] = joypad.update_state(self);

                //println!("{:08b}", self.RAM[0xFF00]);

/*

                if ppu.window.is_key_pressed(Key::Space, KeyRepeat::Yes) {
                    println!("space");
                    self.RAM[0xFF00] |= 0x0F;
                    self.IRQ(4);
                }
                if ppu.window.is_key_pressed(Key::RightShift, KeyRepeat::Yes) {
                    println!("shift");
                    self.RAM[0xFF00] = 215; // select
                    self.IRQ(4);
                }
*/


                let instr_time : u8 = opcode.execute(self);
                cycle += instr_time as u32;
                ppu.update_n_sync(self, instr_time);



                let data = self.assemble_debug_data(opcode.last_instruction.to_string(),
                                                    opcode.last_opcode,
                                                    opcode.lhs,
                                                    opcode.rhs,
                                                    opcode.operand_mode);

                if debugger.window.is_key_pressed(Key::H, KeyRepeat::Yes) {
                    speed = 1;
                }
                if debugger.window.is_key_pressed(Key::D, KeyRepeat::Yes) {
                    speed = 10;
                }
                if debugger.window.is_key_pressed(Key::F, KeyRepeat::Yes) {
                    speed = 50;
                }
                if debugger.window.is_key_pressed(Key::G, KeyRepeat::Yes) {
                    speed = 100;
                }

                debug_data.parse_data_from_cpu(data);
                //thread::sleep(time::Duration::from_millis(speed));

                if debugger.window.is_key_pressed(Key::Space, KeyRepeat::No){
                    loop {
                        debugger.update_window(&debug_data);
                        vram.print_vram(self);
                        if debugger.window.is_key_pressed(Key::LeftCtrl, KeyRepeat::No) {
                            break
                        }
                    }
                }

                if self.freq_change {
                    timer.update_freq(self);
                    self.freq_change = false;
                }
                self.interrupt_checker();
                timer.update(self, instr_time);
            }


            debugger.update_window(&debug_data);
            vram.print_vram(self);
            ppu.render();



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

        for i in 0..rom_buffer.len() {
            self.RAM[i] = rom_buffer[i];
        }
        println!("ROM copying done! {:x}", self.RAM[0x65]);

        self.RAM[0xFF05] = 0;
        self.RAM[0xFF06] = 0;
        self.RAM[0xFF07] = 0;
        self.RAM[0xFF10] = 0x80;
        self.RAM[0xFF11] = 0xBF;
        self.RAM[0xFF12] = 0xF3;
        self.RAM[0xFF14] = 0xBF;
        self.RAM[0xFF16] = 0x3F;
        self.RAM[0xFF17] = 0;
        self.RAM[0xFF19] = 0xBF;
        self.RAM[0xFF1A] = 0x7F;
        self.RAM[0xFF1B] = 0xFF;
        self.RAM[0xFF1C] = 0x9F;
        self.RAM[0xFF1E] = 0xFF;
        self.RAM[0xFF20] = 0xFF;
        self.RAM[0xFF21] = 0;
        self.RAM[0xFF22] = 0;
        self.RAM[0xFF23] = 0xBF;
        self.RAM[0xFF24] = 0x77;
        self.RAM[0xFF25] = 0xF3;
        self.RAM[0xFF26] = 0xF1;
        self.RAM[0xFF40] = 0x91;
        self.RAM[0xFF42] = 0;
        self.RAM[0xFF43] = 0;
        self.RAM[0xFF45] = 0;
        self.RAM[0xFF47] = 0xFC;
        self.RAM[0xFF48] = 0xFF;
        self.RAM[0xFF49] = 0xFF;
        self.RAM[0xFF4A] = 0;
        self.RAM[0xFF4B] = 0;
    }

    pub fn load_rom_header(&mut self) {
        let mut rom_buffer : Vec<u8> = Vec::new();
        let mut f = File::open(String::from("./rom/mario.gb"))
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


    pub fn IRQ(&mut self, t : u8) {
        let IR_flag : u8 = CPU::set_bit(t, self.RAM[0xFF0F]);
        self.RAM[0xFF0F] = IR_flag;
    }

    pub fn interrupt_checker(&mut self) {

        // check if the interrupts are enabled
        // and if there is any request or not
        if self.IR && self.RAM[0xFF0F] > 0 {
            for i in 0..5 {

                // if it has a request and its enabled
                if CPU::get_bit(i, self.RAM[0xFF0F]) &&
                CPU::get_bit(i, self.RAM[0xFFFF]) {

                    self.handler(i);
                }
            }
        }
    }

    pub fn handler(&mut self, t : u8) {
        // reset interrupt flags
        self.IR = false;
        let IR_flag : u8 = CPU::reset_bit(t, self.RAM[0xFF0F]);
        self.RAM[0xFF0F] = IR_flag;

        // push address onto stack
        let pc_h : u8 = ((self.PC) >> 8) as u8;
        let pc_l : u8 = ((self.PC) & 0x00FF) as u8;
        self.SP -= 2;
        let sp : u16 = self.SP;
        self.write_ram(sp, pc_l);
        self.write_ram(sp + 1, pc_h);

        // jump to interrupt function on RAM
        match t {
            0 => self.PC = 0x40,
            1 => self.PC = 0x48,
            2 => self.PC = 0x50,
            4 => self.PC = 0x60,
            _ => return,
        }
    }

    fn set_bit(n : u8, reg : u8) -> u8 {
        let mut value : u8 = 0;
        let mask : u8 = 1 << n;
        value = reg | mask;
        value
    }

    fn reset_bit(n : u8, reg : u8) -> u8 {
        let mut value : u8 = 0;
        let mask : u8 = 1 << n;
        value = reg & (0xFF - mask);
        value
    }

    fn get_bit(n : u8, reg : u8) -> bool {
        let mask : u8 = 1 << n;

        if reg & mask == 0 {
            false
        } else {
            true
        }
    }

}




























pub struct VRAM {
    pub vram : [u32; 350*200],
    pub window : Window,
}

impl VRAM {

    pub fn new() -> VRAM {

        VRAM {
            vram : [0; 350*200],
            window : Window::new("RAM Map",
                         350,
                         200,
                         WindowOptions {
                             resize: false,
                             scale: Scale::X4,
                             ..WindowOptions::default()})
                             .unwrap(),
        }
    }

    pub fn print_vram(& mut self, cpu : &CPU) {

        for i in 0..65535 {

            if cpu.RAM[i] > 0 {
                //println!("{:04X} : {:02X}", i, cpu.RAM[i]);
                self.vram[i] = 0xFF0F380F;
                //print!("{:X} ", cpu.RAM[i]);
                if i >= 0xFE00 && i <= 0xFE9F { self.vram[i] = 0xFFF20056; } // sprite attribute table
                if i >= 0x8000 && i <= 0x8FFF { self.vram[i] = 0xFF003399; } // tile 1
                if i >= 0x8800 && i <= 0x97FF { self.vram[i] = 0xFF003399; } // tile 2
                if i >= 0x9800 && i <= 0x9BFF { self.vram[i] = 0xFF453823; } // window 1
                if i >= 0x9C00 && i <= 0x9FFF { self.vram[i] = 0xFF453823; } // window 2
            } else {
                self.vram[i] = 0xFF9BBC0F;
                if i >= 0xFE00 && i <= 0xFE9F { self.vram[i] = 0xFFFFBAD2; }
                if i >= 0x8000 && i <= 0x8FFF { self.vram[i] = 0xFF66CCFF; }
                if i >= 0x8800 && i <= 0x97FF { self.vram[i] = 0xFF66CCFF; }
                if i >= 0x9800 && i <= 0x9BFF { self.vram[i] = 0xFFBAA378; }
                if i >= 0x9C00 && i <= 0x9FFF { self.vram[i] = 0xFFBAA378; }
            }


        }
        self.window.update_with_buffer(&self.vram).unwrap();
        if self.window.get_mouse_down(MouseButton::Left) {
            self.window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                if mouse.0 as u32 + mouse.1 as u32 * 350 <= 65535 {
                    let pos : u16 = mouse.0 as u16 + mouse.1 as u16 * 350;
                    println!("BYTE: 0x{:02X} 0b{:08b} -- POSITION: 0x{:02X}",
                    cpu.RAM[pos as usize],
                    cpu.RAM[pos as usize],
                    pos);
                }

            });
        }

    }
}
