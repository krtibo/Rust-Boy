use cpu::CPU;

pub struct Timer {
    timer_counter : u32,
    divider_register : u16,
    freq : u32,
    clockspeed : u32,
    pub TMC : u16,
    TMA : u16,
    TIMA : u16,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            clockspeed : 4194304,
            freq : 4096,
            timer_counter : 0,
            divider_register : 0,
            TMC : 0xFF07,
            TMA : 0xFF06,
            TIMA : 0xFF05,
        }
    }

    pub fn update(&mut self, cpu : &mut CPU, cycle : u8) {

        self.div_reg(cpu, cycle);

        if Timer::get_bit(2, cpu.RAM[self.TMC as usize]) {
            self.timer_counter -= cycle as u32;

            if self.timer_counter <= 0 {
                self.update_freq(cpu);

                if cpu.RAM[self.TIMA as usize] == 255 {
                    let TMA = cpu.RAM[self.TMA as usize];
                    cpu.write_ram(self.TIMA, TMA);
                    cpu.IRQ(2);
                } else {
                    let TIMA = cpu.RAM[self.TIMA as usize];
                    cpu.write_ram(self.TIMA, TIMA + 1);
                }
            }
        }
    }

    pub fn update_freq(&mut self, cpu : &mut CPU) {
        let new_freq = cpu.RAM[self.TMC as usize] & 0x3;

        match new_freq {
            0 => self.timer_counter = 1024,
            1 => self.timer_counter = 16,
            2 => self.timer_counter = 64,
            3 => self.timer_counter = 256,
            _ => return            
        }
    }

    fn div_reg(&mut self, cpu : &mut CPU, cycle : u8) {
        self.divider_register += cycle as u16;
        if self.divider_register >= 255 && cpu.RAM[0xFF04] < 255 {
            self.divider_register = 0;
            cpu.RAM[0xFF04] += 1;
            println!("{}", cpu.RAM[0xFF04]);
        }

        if self.divider_register >= 255 && cpu.RAM[0xFF04] == 255 {
            self.divider_register = 0;
            cpu.RAM[0xFF04] = 0;
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