#![allow(dead_code)]
#![allow(unused_variables)]

use cpu::CPU;

pub struct Opcode {
    pub lhs : u8,
    pub rhs : u8,
    pub opc : [fn(&mut Opcode, &mut CPU) -> u8; 256],
    pub cb_opc : [fn(&mut Opcode, &mut CPU) -> u8; 256],
    pub last_instruction :  &'static str,

}

impl Opcode {
    pub fn new() -> Opcode {
        Opcode {
            lhs : 0,
            rhs : 0,
            opc : [Opcode::default; 256],
            cb_opc : [Opcode::default; 256],
            last_instruction : "",

        }
    }


    pub fn init(&mut self) {
        // LD nnn
        self.opc[0x06] = Opcode::ld_nnn_06;
        self.opc[0x0e] = Opcode::ld_nnn_0e;
        self.opc[0x16] = Opcode::ld_nnn_16;
        self.opc[0x1e] = Opcode::ld_nnn_1e;
        self.opc[0x26] = Opcode::ld_nnn_26;
        self.opc[0x2e] = Opcode::ld_nnn_2e;
        // LD r1,r2
        self.opc[0x7f] = Opcode::ld_r1r2_7f;
        self.opc[0x78] = Opcode::ld_r1r2_78;
        self.opc[0x79] = Opcode::ld_r1r2_79;
        self.opc[0x7a] = Opcode::ld_r1r2_7a;
        self.opc[0x7b] = Opcode::ld_r1r2_7b;
        self.opc[0x7c] = Opcode::ld_r1r2_7c;
        self.opc[0x7d] = Opcode::ld_r1r2_7d;
        self.opc[0x7e] = Opcode::ld_r1r2_7e;
        self.opc[0x40] = Opcode::ld_r1r2_40;
        self.opc[0x41] = Opcode::ld_r1r2_41;
        self.opc[0x42] = Opcode::ld_r1r2_42;
        self.opc[0x43] = Opcode::ld_r1r2_43;
        self.opc[0x44] = Opcode::ld_r1r2_44;
        self.opc[0x45] = Opcode::ld_r1r2_45;
        self.opc[0x46] = Opcode::ld_r1r2_46;
        self.opc[0x48] = Opcode::ld_r1r2_48;
        self.opc[0x49] = Opcode::ld_r1r2_49;
        self.opc[0x4a] = Opcode::ld_r1r2_4a;
        self.opc[0x4b] = Opcode::ld_r1r2_4b;
        self.opc[0x4c] = Opcode::ld_r1r2_4c;
        self.opc[0x4d] = Opcode::ld_r1r2_4d;
        self.opc[0x4e] = Opcode::ld_r1r2_4e;
        self.opc[0x50] = Opcode::ld_r1r2_50;
        self.opc[0x51] = Opcode::ld_r1r2_51;
        self.opc[0x52] = Opcode::ld_r1r2_52;
        self.opc[0x53] = Opcode::ld_r1r2_53;
        self.opc[0x54] = Opcode::ld_r1r2_54;
        self.opc[0x55] = Opcode::ld_r1r2_55;
        self.opc[0x56] = Opcode::ld_r1r2_56;
        self.opc[0x58] = Opcode::ld_r1r2_58;
        self.opc[0x59] = Opcode::ld_r1r2_59;
        self.opc[0x5a] = Opcode::ld_r1r2_5a;
        self.opc[0x5b] = Opcode::ld_r1r2_5b;
        self.opc[0x5c] = Opcode::ld_r1r2_5c;
        self.opc[0x5d] = Opcode::ld_r1r2_5d;
        self.opc[0x5e] = Opcode::ld_r1r2_5e;
    }


    // fetch the opcode and the byte after
    pub fn fetch(&mut self, cpu : &mut CPU) -> u8 {
        // increase the Program Counter
        let ret = cpu.RAM[(cpu.PC) as usize];
        cpu.PC += 1;
        ret
    }


    pub fn execute(&mut self, cpu : &mut CPU) -> u8 {
        self.opc[self.fetch(cpu) as usize](self, cpu)
    }

    fn byte_cat(h : u8, l : u8) -> u16 {
        let high : u16 = (h as u16) << 8;
        let low : u16 = l as u16;
        let hl : u16 = high | low;
        hl
    }

    //////  O P C O D E S  //////


    fn default(&mut self, cpu : &mut CPU) -> u8 {
        //println!("DEFAULT");
        self.last_instruction = "Default";
        1
    }


    fn ld_nnn_06(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = self.fetch(cpu);
        println!("LD B, nnn");
        self.last_instruction = "LD B, nnn";
        8
    }


    fn ld_nnn_0e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = self.fetch(cpu);
        println!("LD C, nnn");
        self.last_instruction = "LD C, nnn";
        8
    }


    fn ld_nnn_16(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = self.fetch(cpu);
        println!("LD D, nnn");
        self.last_instruction = "LD D, nnn";
        8
    }


    fn ld_nnn_1e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = self.fetch(cpu);
        println!("LD E, nnn");
        self.last_instruction = "LD E, nnn";
        8
    }


    fn ld_nnn_26(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = self.fetch(cpu);
        println!("LD H, nnn");
        self.last_instruction = "LD H, nnn";
        8
    }


    fn ld_nnn_2e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = self.fetch(cpu);
        println!("LD L, nnn");
        self.last_instruction = "LD L, nnn";
        8
    }








    fn ld_r1r2_7f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_78(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.B;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_79(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.C;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_7a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.D;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_7b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.E;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_7c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.H;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_7d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.L;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_7e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];
        println!("LD r1,r2");
        8
    }


    fn ld_r1r2_40(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.B;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_41(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.C;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_42(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.D;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_43(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.E;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_44(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.H;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_45(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.L;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_46(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];
        println!("LD r1,r2");
        8
    }


    fn ld_r1r2_48(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.B;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_49(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.C;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_4a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.D;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_4b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.E;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_4c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.H;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_4d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.L;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_4e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];
        println!("LD r1,r2");
        8
    }


    fn ld_r1r2_50(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.B;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_51(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.C;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_52(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.D;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_53(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.E;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_54(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.H;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_55(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.L;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_56(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];
        println!("LD r1,r2");
        8
    }


    fn ld_r1r2_58(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.B;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_59(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.C;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_5a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.D;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_5b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.E;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_5c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.H;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_5d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.L;
        println!("LD r1,r2");
        4
    }


    fn ld_r1r2_5e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];
        println!("LD r1,r2");
        8
    }


}
