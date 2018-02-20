#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use cpu::CPU;

pub struct Interrupt {
}

impl Interrupt {
    pub fn new() -> Interrupt {
        Interrupt {}
    }

    #[allow(non_snake_case)]
    pub fn IRQ(&self, cpu : &mut CPU, t : u8) {
        let IR_flag : u8 = CPU::set_bit(t, cpu.RAM[0xFF0F]);
        cpu.RAM[0xFF0F] = IR_flag;
    }

    pub fn interrupt_checker(&mut self, cpu : &mut CPU) {

        // check if the interrupts are enabled
        // and if there is any request or not
        if cpu.IR && cpu.RAM[0xFF0F] > 0 {
            for i in 0..5 {

                // if it has a request and its enabled
                if CPU::get_bit(i, cpu.RAM[0xFF0F]) &&
                CPU::get_bit(i, cpu.RAM[0xFFFF]) {

                    self.handler(cpu, i);
                }
            }
        }
    }

    pub fn handler(&self, cpu : &mut CPU, t : u8) {

        // reset interrupt flags
        cpu.IR = false;
        let IR_flag : u8 = CPU::reset_bit(t, cpu.RAM[0xFF0F]);
        cpu.RAM[0xFF0F] = IR_flag;

        // push address onto stack
        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;
        cpu.SP -= 2;
        let sp : u16 = cpu.SP;
        cpu.write_ram(sp, pc_l);
        cpu.write_ram(sp + 1, pc_h);

        // jump to interrupt function on RAM
        match t {
            0 => cpu.PC = 0x40,
            1 => cpu.PC = 0x48,
            2 => cpu.PC = 0x50,
            4 => cpu.PC = 0x60,
            _ => return,
        }
    }
}
