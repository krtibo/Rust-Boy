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
        let IR_flag : u8 = Interrupt::set_bit(t, cpu.RAM[0xFF0F]);
        cpu.RAM[0xFF0F] = IR_flag;
        println!("IRQ number {}", t);
    }

    pub fn interrupt_checker(&mut self, cpu : &mut CPU) {

        // check if the interrupts are enabled
        // and if there is any request or not
        if cpu.IR && cpu.RAM[0xFF0F] > 0 {
            for i in 0..5 {

                // if it has a request and its enabled
                if Interrupt::get_bit(i, cpu.RAM[0xFF0F]) &&
                Interrupt::get_bit(i, cpu.RAM[0xFFFF]) {

                    self.handler(cpu, i);
                }
            }
        }
    }

    pub fn handler(&self, cpu : &mut CPU, t : u8) {

        // reset interrupt flags
        cpu.IR = false;
        let IR_flag : u8 = Interrupt::reset_bit(t, cpu.RAM[0xFF0F]);
        cpu.RAM[0xFF0F] = IR_flag;

        // push address onto stack
        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;
        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);

        // jump to interrupt function on RAM
        match t {
            0 => cpu.PC = 0x40,
            1 => cpu.PC = 0x48,
            2 => cpu.PC = 0x50,
            4 => cpu.PC = 0x60,
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