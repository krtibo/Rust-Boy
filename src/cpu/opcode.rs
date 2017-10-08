#![allow(dead_code)]
#![allow(unused_variables)]

use cpu::CPU;

pub struct Opcode {
    pub lhs : u16,
    pub rhs : u16,
    pub operand_mode : u8,
    pub opc : [fn(&mut Opcode, &mut CPU) -> u8; 256],
    pub cb_opc : [fn(&mut Opcode, &mut CPU) -> u8; 256],
    pub last_instruction :  &'static str,
}

impl Opcode {
    pub fn new() -> Opcode {
        Opcode {
            lhs : 0,
            rhs : 0,
            operand_mode : 0,
            opc : [Opcode::default; 256],
            cb_opc : [Opcode::default; 256],
            last_instruction : "",
        }
    }


    pub fn init(&mut self) {
        //self.opc[0x00] = Opcode::nop_00;
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
        self.opc[0x60] = Opcode::ld_r1r2_60;
        self.opc[0x61] = Opcode::ld_r1r2_61;
        self.opc[0x62] = Opcode::ld_r1r2_62;
        self.opc[0x63] = Opcode::ld_r1r2_63;
        self.opc[0x64] = Opcode::ld_r1r2_64;
        self.opc[0x65] = Opcode::ld_r1r2_65;
        self.opc[0x66] = Opcode::ld_r1r2_66;
        self.opc[0x68] = Opcode::ld_r1r2_68;
        self.opc[0x69] = Opcode::ld_r1r2_69;
        self.opc[0x6a] = Opcode::ld_r1r2_6a;
        self.opc[0x6b] = Opcode::ld_r1r2_6b;
        self.opc[0x6c] = Opcode::ld_r1r2_6c;
        self.opc[0x6d] = Opcode::ld_r1r2_6d;
        self.opc[0x6e] = Opcode::ld_r1r2_6e;
        // LD A,n
        self.opc[0x0a] = Opcode::ld_an_0a;
        self.opc[0x1a] = Opcode::ld_an_1a;
        self.opc[0xfa] = Opcode::ld_an_fa;
        self.opc[0x3e] = Opcode::ld_an_3e;
        // LD n,A
        self.opc[0x47] = Opcode::ld_na_47;
        self.opc[0x4f] = Opcode::ld_na_4f;
        self.opc[0x57] = Opcode::ld_na_57;
        self.opc[0x5f] = Opcode::ld_na_5f;
        self.opc[0x67] = Opcode::ld_na_67;
        self.opc[0x6f] = Opcode::ld_na_6f;
        self.opc[0x02] = Opcode::ld_na_02;
        self.opc[0x12] = Opcode::ld_na_12;
        self.opc[0x77] = Opcode::ld_na_77;
        self.opc[0xea] = Opcode::ld_na_ea;

        self.opc[0xf2] = Opcode::ld_ac_f2;
        self.opc[0xe2] = Opcode::ld_ca_e2;
        self.opc[0x3a] = Opcode::ldd_ahl_3a;
        self.opc[0x32] = Opcode::ldd_hla_32;
        self.opc[0x2a] = Opcode::ldi_ahl_2a;
        self.opc[0x22] = Opcode::ldi_hla_22;
        self.opc[0xe0] = Opcode::ldh_na_e0;
        self.opc[0xf0] = Opcode::ldh_an_f0;
        // 16-bit loads
        self.opc[0x01] = Opcode::ld_nnn_01;
        self.opc[0x11] = Opcode::ld_nnn_11;
        self.opc[0x21] = Opcode::ld_nnn_21;
        self.opc[0x31] = Opcode::ld_nnn_31;
        self.opc[0xf9] = Opcode::ld_sphl_f9;
        self.opc[0x08] = Opcode::ld_nnsp_08;
        //push
        // TODO check these
        self.opc[0xf5] = Opcode::push_nn_f5;
        self.opc[0xc5] = Opcode::push_nn_c5;
        self.opc[0xd5] = Opcode::push_nn_d5;
        self.opc[0xe5] = Opcode::push_nn_e5;
        // pop
        // TODO check these as well
        self.opc[0xf1] = Opcode::pop_nn_f1;
        self.opc[0xc1] = Opcode::pop_nn_c1;
        self.opc[0xd1] = Opcode::pop_nn_d1;
        self.opc[0xe1] = Opcode::pop_nn_e1;
        // inc, dec
        self.opc[0x03] = Opcode::inc_nn_03;
        self.opc[0x13] = Opcode::inc_nn_13;
        self.opc[0x23] = Opcode::inc_nn_23;
        self.opc[0x33] = Opcode::inc_nn_33;
        self.opc[0x04] = Opcode::inc_b_04;
        self.opc[0x0c] = Opcode::inc_c_0c;
        self.opc[0x14] = Opcode::inc_d_14;
        self.opc[0x1c] = Opcode::inc_e_1c;
        self.opc[0x24] = Opcode::inc_h_24;
        self.opc[0x2c] = Opcode::inc_l_2c;
        self.opc[0x34] = Opcode::inc_hl_34;
        self.opc[0x3c] = Opcode::inc_a_3c;
        self.opc[0x0b] = Opcode::dec_nn_0b;
        self.opc[0x1b] = Opcode::dec_nn_1b;
        self.opc[0x2b] = Opcode::dec_nn_2b;
        self.opc[0x3b] = Opcode::dec_nn_3b;
        self.opc[0x05] = Opcode::dec_b_05;
        self.opc[0x0d] = Opcode::dec_c_0d;
        self.opc[0x15] = Opcode::dec_d_15;
        self.opc[0x1d] = Opcode::dec_e_1d;
        self.opc[0x25] = Opcode::dec_h_25;
        self.opc[0x2d] = Opcode::dec_l_2d;
        self.opc[0x35] = Opcode::dec_hl_35;
        self.opc[0x3d] = Opcode::dec_a_3d;

        // and
        self.opc[0xa7] = Opcode::and_n_a7;
        self.opc[0xa0] = Opcode::and_n_a0;
        self.opc[0xa1] = Opcode::and_n_a1;
        self.opc[0xa2] = Opcode::and_n_a2;
        self.opc[0xa3] = Opcode::and_n_a3;
        self.opc[0xa4] = Opcode::and_n_a4;
        self.opc[0xa5] = Opcode::and_n_a5;
        self.opc[0xa6] = Opcode::and_n_a6;
        self.opc[0xe6] = Opcode::and_n_e6;
        // or
        self.opc[0xb7] = Opcode::or_n_b7;
        self.opc[0xb0] = Opcode::or_n_b0;
        self.opc[0xb1] = Opcode::or_n_b1;
        self.opc[0xb2] = Opcode::or_n_b2;
        self.opc[0xb3] = Opcode::or_n_b3;
        self.opc[0xb4] = Opcode::or_n_b4;
        self.opc[0xb5] = Opcode::or_n_b5;
        self.opc[0xb6] = Opcode::or_n_b6;
        self.opc[0xf6] = Opcode::or_n_f6;
        // xor
        self.opc[0xaf] = Opcode::xor_n_af;
        self.opc[0xa8] = Opcode::xor_n_a8;
        self.opc[0xa9] = Opcode::xor_n_a9;
        self.opc[0xaa] = Opcode::xor_n_aa;
        self.opc[0xab] = Opcode::xor_n_ab;
        self.opc[0xac] = Opcode::xor_n_ac;
        self.opc[0xad] = Opcode::xor_n_ad;
        self.opc[0xae] = Opcode::xor_n_ae;
        self.opc[0xee] = Opcode::xor_n_ee;
        // add
        self.opc[0x87] = Opcode::add_an_87;
        self.opc[0x80] = Opcode::add_an_80;
        self.opc[0x81] = Opcode::add_an_81;
        self.opc[0x82] = Opcode::add_an_82;
        self.opc[0x83] = Opcode::add_an_83;
        self.opc[0x84] = Opcode::add_an_84;
        self.opc[0x85] = Opcode::add_an_85;
        self.opc[0x86] = Opcode::add_an_86;
        self.opc[0xc6] = Opcode::add_an_c6;
        self.opc[0x09] = Opcode::add_hl_n_09;
        self.opc[0x19] = Opcode::add_hl_n_19;
        self.opc[0x29] = Opcode::add_hl_n_29;
        self.opc[0x39] = Opcode::add_hl_n_39;
        // adc
        self.opc[0x8f] = Opcode::adc_an_8f;
        self.opc[0x88] = Opcode::adc_an_88;
        self.opc[0x89] = Opcode::adc_an_89;
        self.opc[0x8a] = Opcode::adc_an_8a;
        self.opc[0x8b] = Opcode::adc_an_8b;
        self.opc[0x8c] = Opcode::adc_an_8c;
        self.opc[0x8d] = Opcode::adc_an_8d;
        self.opc[0x8e] = Opcode::adc_an_8e;
        self.opc[0xce] = Opcode::adc_an_ce;

        self.opc[0xcd] = Opcode::call_nn_cd;
        self.opc[0x20] = Opcode::jr_cc_n_20;


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
        self.rhs = cpu.RAM[(cpu.PC - 1) as usize] as u16;
        self.lhs = 0;
        self.last_instruction = "NOP or Default ---";
        self.operand_mode = 1;
        1
    }

    fn nop_00(&mut self, cpu : &mut CPU) -> u8 {
        4
    }


    fn ld_nnn_06(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.B = data;
        self.rhs = data as u16;
        self.last_instruction = "LD B,";
        self.operand_mode = 1;
        8
    }


    fn ld_nnn_0e(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.C = data;
        self.rhs = data as u16;
        self.last_instruction = "LD C,";
        self.operand_mode = 1;
        8
    }


    fn ld_nnn_16(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.D = data;
        self.rhs = data as u16;
        self.last_instruction = "LD D,";
        self.operand_mode = 1;
        8
    }


    fn ld_nnn_1e(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.E = data;
        self.rhs = data as u16;
        self.last_instruction = "LD E,";
        self.operand_mode = 1;
        8
    }


    fn ld_nnn_26(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.H = data;
        self.rhs = data as u16;
        self.last_instruction = "LD H,";
        self.operand_mode = 1;
        8
    }


    fn ld_nnn_2e(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.L = data;
        self.rhs = data as u16;
        self.last_instruction = "LD L,";
        self.operand_mode = 1;
        8
    }








    fn ld_r1r2_7f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A;

        self.last_instruction = "LD A, A";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_78(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.B;

        self.last_instruction = "LD A, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_79(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.C;

        self.last_instruction = "LD A, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_7a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.D;

        self.last_instruction = "LD A, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_7b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.E;

        self.last_instruction = "LD A, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_7c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.H;

        self.last_instruction = "LD A, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_7d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.L;

        self.last_instruction = "LD A, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_7e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        self.last_instruction = "LD A, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_40(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.B;

        self.last_instruction = "LD B, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_41(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.C;

        self.last_instruction = "LD B, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_42(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.D;

        self.last_instruction = "LD B, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_43(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.E;

        self.last_instruction = "LD B, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_44(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.H;

        self.last_instruction = "LD B, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_45(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.L;

        self.last_instruction = "LD B, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_46(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];

        self.last_instruction = "LD B, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_48(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.B;

        self.last_instruction = "LD C, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_49(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.C;

        self.last_instruction = "LD C, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_4a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.D;

        self.last_instruction = "LD C, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_4b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.E;

        self.last_instruction = "LD C, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_4c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.H;

        self.last_instruction = "LD C, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_4d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.L;

        self.last_instruction = "LD C, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_4e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        self.last_instruction = "LD C, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_50(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.B;

        self.last_instruction = "LD D, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_51(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.C;

        self.last_instruction = "LD D, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_52(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.D;

        self.last_instruction = "LD D, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_53(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.E;

        self.last_instruction = "LD D, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_54(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.H;

        self.last_instruction = "LD D, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_55(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.L;

        self.last_instruction = "LD D, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_56(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        self.last_instruction = "LD D, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_58(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.B;

        self.last_instruction = "LD E, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_59(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.C;

        self.last_instruction = "LD E, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_5a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.D;

        self.last_instruction = "LD E, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_5b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.E;

        self.last_instruction = "LD E, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_5c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.H;

        self.last_instruction = "LD E, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_5d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.L;

        self.last_instruction = "LD E, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_5e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        self.last_instruction = "LD E, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_60(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.B;

        self.last_instruction = "LD H, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_61(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.C;

        self.last_instruction = "LD H, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_62(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.D;

        self.last_instruction = "LD H, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_63(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.E;

        self.last_instruction = "LD H, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_64(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.H;

        self.last_instruction = "LD H, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_65(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.L;

        self.last_instruction = "LD H, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_66(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        self.last_instruction = "LD H, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_68(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.B;

        self.last_instruction = "LD L, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_69(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.C;

        self.last_instruction = "LD L, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_6a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.D;

        self.last_instruction = "LD L, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_6b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.E;

        self.last_instruction = "LD L, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_6c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.H;

        self.last_instruction = "LD L, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_6d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.L;

        self.last_instruction = "LD L, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_6e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        self.last_instruction = "LD L, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_70(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.B;

        self.last_instruction = "LD (HL), B";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_71(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.C;

        self.last_instruction = "LD (HL), C";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_72(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.D;

        self.last_instruction = "LD (HL), D";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_73(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.E;

        self.last_instruction = "LD (HL), E";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_74(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.H;

        self.last_instruction = "LD (HL), H";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_75(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.L;

        self.last_instruction = "LD (HL), L";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_36(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = data;

        self.rhs = data as u16;
        self.last_instruction = "LD (HL),";
        self.operand_mode = 1;
        12
    }


    fn ld_an_0a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.RAM[Opcode::byte_cat(cpu.B, cpu.C) as usize];

        self.last_instruction = "LD A, (BC)";
        self.operand_mode = 0;
        8
    }


    fn ld_an_1a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.RAM[Opcode::byte_cat(cpu.D, cpu.E) as usize];

        self.last_instruction = "LD A, (DE)";
        self.operand_mode = 0;
        8
    }


    fn ld_an_fa(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);

        cpu.A = cpu.RAM[Opcode::byte_cat(data_h, data_l) as usize];

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD A,";
        self.operand_mode = 2;
        16
    }


    fn ld_an_3e(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.A = data;

        self.rhs = data as u16;
        self.last_instruction = "LD A,";
        self.operand_mode = 1;
        8
    }


    fn ld_na_47(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.A;

        self.last_instruction = "LD B,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_4f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.A;

        self.last_instruction = "LD C,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_57(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.A;

        self.last_instruction = "LD D,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_5f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.A;

        self.last_instruction = "LD E,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_67(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.A;

        self.last_instruction = "LD H,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_6f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.A;

        self.last_instruction = "LD L,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_02(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.B, cpu.C) as usize] = cpu.A;

        self.last_instruction = "LD (BC),A";
        self.operand_mode = 0;
        8
    }


    fn ld_na_12(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.D, cpu.E) as usize] = cpu.A;

        self.last_instruction = "LD (DE),A";
        self.operand_mode = 0;
        8
    }


    fn ld_na_77(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.A;

        self.last_instruction = "LD (HL),A";
        self.operand_mode = 0;
        8
    }


    fn ld_na_ea(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);

        cpu.RAM[Opcode::byte_cat(data_h, data_l) as usize] = cpu.A;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD (nn),A";
        self.operand_mode = 2;
        16
    }


    fn ld_ac_f2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.RAM[0xFF00 + cpu.C as usize];

        self.last_instruction = "LD A,(C)";
        self.operand_mode = 0;
        8
    }


    fn ld_ca_e2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[0xFF00 + cpu.C as usize] = cpu.A;

        self.last_instruction = "LD (C),A";
        self.operand_mode = 0;
        8
    }


    fn ldd_ahl_3a(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.A = cpu.RAM[hl as usize];
        hl -= 1;
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDD A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn ldd_hla_32(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.RAM[hl as usize] = cpu.A;
        hl -= 1;
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDD (HL),A";
        self.operand_mode = 0;
        8
    }


    fn ldi_ahl_2a(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.A = cpu.RAM[hl as usize];
        hl += 1;
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDI A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn ldi_hla_22(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.RAM[hl as usize] = cpu.A;
        hl += 1;
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDD (HL),A";
        self.operand_mode = 0;
        8
    }


    fn ldh_na_e0(&mut self, cpu : &mut CPU) -> u8 {
        let n : u8 = self.fetch(cpu);
        cpu.RAM[0xFF00 + n as usize] = cpu.A;

        self.rhs = n as u16;
        self.last_instruction = "LDH (n),A";
        self.operand_mode = 1;
        12
    }


    fn ldh_an_f0(&mut self, cpu : &mut CPU) -> u8 {
        let n : u8 = self.fetch(cpu);
        cpu.A = cpu.RAM[0xFF00 + n as usize];

        self.rhs = n as u16;
        self.last_instruction = "LDH A, (n)";
        self.operand_mode = 1;
        12
    }


    fn ld_nnn_01(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);
        let bc : u16 = Opcode::byte_cat(data_h, data_l);
        cpu.B = (bc >> 8) as u8;
        cpu.C = (bc & 0x00FF) as u8;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD BC,nn";
        self.operand_mode = 2;
        12
    }


    fn ld_nnn_11(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);
        let de : u16 = Opcode::byte_cat(data_h, data_l);
        cpu.D = (de >> 8) as u8;
        cpu.E = (de & 0x00FF) as u8;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD DE,nn";
        self.operand_mode = 2;
        12
    }


    fn ld_nnn_21(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);
        let hl : u16 = Opcode::byte_cat(data_h, data_l);
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD HL,nn";
        self.operand_mode = 2;
        12
    }


    fn ld_nnn_31(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);
        let sp : u16 = Opcode::byte_cat(data_h, data_l);
        cpu.SP = sp;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD SP,nn";
        self.operand_mode = 2;
        12
    }


    fn ld_sphl_f9(&mut self, cpu : &mut CPU) -> u8 {
        cpu.SP = Opcode::byte_cat(cpu.H, cpu.L);

        self.last_instruction = "LD SP,HL";
        self.operand_mode = 0;
        8
    }


    fn ld_nnsp_08(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);
        cpu.RAM[Opcode::byte_cat(data_h, data_l) as usize] =
        (cpu.SP >> 8) as u8;

        cpu.RAM[(Opcode::byte_cat(data_h, data_l) + (1 as u16)) as usize] =
        (cpu.SP & 0x00FF) as u8;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD (nn),SP";
        self.operand_mode = 2;
        20
    }


    fn push_nn_f5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.STACK.push_front(cpu.A);
        cpu.STACK.push_front(cpu.F);
        cpu.SP -= 2;

        self.last_instruction = "PUSH AF";
        self.operand_mode = 0;
        16
    }


    fn push_nn_c5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.STACK.push_front(cpu.B);
        cpu.STACK.push_front(cpu.C);
        cpu.SP -= 2;

        self.last_instruction = "PUSH BC";
        self.operand_mode = 0;
        16
    }


    fn push_nn_d5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.STACK.push_front(cpu.D);
        cpu.STACK.push_front(cpu.E);
        cpu.SP -= 2;

        self.last_instruction = "PUSH DE";
        self.operand_mode = 0;
        16
    }


    fn push_nn_e5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.STACK.push_front(cpu.H);
        cpu.STACK.push_front(cpu.L);
        cpu.SP -= 2;

        self.last_instruction = "PUSH HL";
        self.operand_mode = 0;
        16
    }


    fn pop_nn_f1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.F = cpu.STACK.pop_front().unwrap();
        cpu.A = cpu.STACK.pop_front().unwrap();
        cpu.SP += 2;

        self.last_instruction = "POP AF";
        self.operand_mode = 0;
        12
    }


    fn pop_nn_c1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.STACK.pop_front().unwrap();
        cpu.B = cpu.STACK.pop_front().unwrap();
        cpu.SP += 2;

        self.last_instruction = "POP BC";
        self.operand_mode = 0;
        12
    }


    fn pop_nn_d1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.STACK.pop_front().unwrap();
        cpu.D = cpu.STACK.pop_front().unwrap();
        cpu.SP += 2;

        self.last_instruction = "POP DE";
        self.operand_mode = 0;
        12
    }


    fn pop_nn_e1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.STACK.pop_front().unwrap();
        cpu.H = cpu.STACK.pop_front().unwrap();
        cpu.SP += 2;

        self.last_instruction = "POP HL";
        self.operand_mode = 0;
        12
    }


    fn inc_b_04(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.B + 1 == 0 { 
            cpu.set_flag("Z"); 
        } else { 
            cpu.reset_flag("Z"); 
        }
 

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.B & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.B += 1;

        self.last_instruction = "INC B";
        self.operand_mode = 0;
        4
    }


    fn inc_c_0c(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.C + 1 == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.C & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.C += 1;

        self.last_instruction = "INC C";
        self.operand_mode = 0;
        4
    }


    fn inc_d_14(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.D + 1 == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.D & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.D += 1;

        self.last_instruction = "INC D";
        self.operand_mode = 0;
        4
    }


    fn inc_e_1c(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.E + 1 == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.E & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.E += 1;

        self.last_instruction = "INC E";
        self.operand_mode = 0;
        4
    }


    fn inc_h_24(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.H + 1 == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.H & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.H += 1;

        self.last_instruction = "INC H";
        self.operand_mode = 0;
        4
    }


    fn inc_l_2c(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.L + 1 == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.L & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.L += 1;

        self.last_instruction = "INC L";
        self.operand_mode = 0;
        4
    }


    fn inc_hl_34(&mut self, cpu : &mut CPU) -> u8 {

        let hl : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        if hl + 1 == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");    // reset N flag

        if (((hl & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] += 1;

        self.last_instruction = "INC (HL)";
        self.operand_mode = 0;
        12
    }


    fn inc_a_3c(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.A + 1 == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.A & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.A += 1;

        self.last_instruction = "INC A";
        self.operand_mode = 0;
        4
    }


    fn inc_nn_03(&mut self, cpu : &mut CPU) -> u8 {
        let mut bc : u16 = Opcode::byte_cat(cpu.B, cpu.C);
        bc += 1;
        cpu.B = (bc >> 8) as u8;
        cpu.C = (bc & 0x00FF) as u8;

        self.last_instruction = "INC BC";
        self.operand_mode = 0;
        8
    }


    fn inc_nn_13(&mut self, cpu : &mut CPU) -> u8 {
        let mut de : u16 = Opcode::byte_cat(cpu.D, cpu.E);
        de += 1;
        cpu.D = (de >> 8) as u8;
        cpu.E = (de & 0x00FF) as u8;

        self.last_instruction = "INC DE";
        self.operand_mode = 0;
        8
    }


    fn inc_nn_23(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        hl += 1;
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "INC HL";
        self.operand_mode = 0;
        8
    }


    fn inc_nn_33(&mut self, cpu : &mut CPU) -> u8 {
        cpu.SP += 1;

        self.last_instruction = "INC SP";
        self.operand_mode = 0;
        8
    }


    fn dec_nn_0b(&mut self, cpu : &mut CPU) -> u8 {
        let mut bc : u16 = Opcode::byte_cat(cpu.B, cpu.C);
        bc -= 1;
        cpu.B = (bc >> 8) as u8;
        cpu.C = (bc & 0x00FF) as u8;

        self.last_instruction = "DEC BC";
        self.operand_mode = 0;
        8
    }


    fn dec_nn_1b(&mut self, cpu : &mut CPU) -> u8 {
        let mut de : u16 = Opcode::byte_cat(cpu.D, cpu.E);
        de -= 1;
        cpu.D = (de >> 8) as u8;
        cpu.E = (de & 0x00FF) as u8;

        self.last_instruction = "DEC DE";
        self.operand_mode = 0;
        8
    }


    fn dec_nn_2b(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        hl -= 1;
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "DEC HL";
        self.operand_mode = 0;
        8
    }


    fn dec_nn_3b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.SP -= 1;

        self.last_instruction = "DEC SP";
        self.operand_mode = 0;
        8
    }


    fn dec_b_05(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.B - 1  == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");    // set N flag

        if (cpu.B & 0x0F) < (1 & 0x0F) {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.B -= 1;

        self.last_instruction = "DEC B";
        self.operand_mode = 0;
        4
    }


    fn dec_c_0d(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.C - 1  == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");    // set N flag

        if (cpu.C & 0x0F) < (1 & 0x0F) {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.C -= 1;

        self.last_instruction = "DEC C";
        self.operand_mode = 0;
        4
    }


    fn dec_d_15(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.D - 1  == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");    // set N flag

        if (cpu.D & 0x0F) < (1 & 0x0F) {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.D -= 1;

        self.last_instruction = "DEC D";
        self.operand_mode = 0;
        4
    }


    fn dec_e_1d(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.E - 1  == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");    // set N flag

        if (cpu.E & 0x0F) < (1 & 0x0F) {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.E -= 1;

        self.last_instruction = "DEC E";
        self.operand_mode = 0;
        4
    }


    fn dec_h_25(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.H - 1  == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");    // set N flag

        if (cpu.H & 0x0F) < (1 & 0x0F) {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.H -= 1;

        self.last_instruction = "DEC H";
        self.operand_mode = 0;
        4
    }


    fn dec_l_2d(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.L - 1  == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");    // set N flag

        if (cpu.L & 0x0F) < (1 & 0x0F) {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.L -= 1;

        self.last_instruction = "DEC L";
        self.operand_mode = 0;
        4
    }


    fn dec_hl_35(&mut self, cpu : &mut CPU) -> u8 {

        let hl : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        if hl - 1  == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");    // set N flag

        if (hl & 0x0F) < (1 & 0x0F) {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] -= 1;

        self.last_instruction = "DEC (HL)";
        self.operand_mode = 0;
        12
    }


    fn dec_a_3d(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.A - 1  == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");    // set N flag

        if (cpu.A & 0x0F) < (1 & 0x0F) {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.A -= 1;

        self.last_instruction = "DEC A";
        self.operand_mode = 0;
        4
    }


    fn and_n_a7(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A & cpu.A;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "AND A,A";
        self.operand_mode = 0;
        4
    }


    fn and_n_a0(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A & cpu.B;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "AND A,B";
        self.operand_mode = 0;
        4
    }


    fn and_n_a1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A & cpu.C;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "AND A,C";
        self.operand_mode = 0;
        4
    }


    fn and_n_a2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A & cpu.D;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "AND A,D";
        self.operand_mode = 0;
        4
    }


    fn and_n_a3(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A & cpu.E;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "AND A,E";
        self.operand_mode = 0;
        4
    }


    fn and_n_a4(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A & cpu.H;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "AND A,H";
        self.operand_mode = 0;
        4
    }


    fn and_n_a5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A & cpu.L;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "AND A,L";
        self.operand_mode = 0;
        4
    }


    fn and_n_a6(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A & cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "AND A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn and_n_e6(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.A = cpu.A & data;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.set_flag("H");
            cpu.reset_flag("C");
        }

        self.rhs = data as u16;
        self.last_instruction = "AND A,";
        self.operand_mode = 1;
        8
    }


    fn or_n_b7(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A | cpu.A;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "OR A,A";
        self.operand_mode = 0;
        4
    }


    fn or_n_b0(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A | cpu.B;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "OR A,B";
        self.operand_mode = 0;
        4
    }


    fn or_n_b1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A | cpu.C;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "OR A,C";
        self.operand_mode = 0;
        4
    }


    fn or_n_b2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A | cpu.D;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "OR A,D";
        self.operand_mode = 0;
        4
    }


    fn or_n_b3(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A | cpu.E;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "OR A,E";
        self.operand_mode = 0;
        4
    }


    fn or_n_b4(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A | cpu.H;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "OR A,H";
        self.operand_mode = 0;
        4
    }


    fn or_n_b5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A | cpu.L;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "OR A,L";
        self.operand_mode = 0;
        4
    }


    fn or_n_b6(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A | cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "OR A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn or_n_f6(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.A = cpu.A | data;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.rhs = data as u16;
        self.last_instruction = "OR A,";
        self.operand_mode = 1;
        8
    }


    fn xor_n_af(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A ^ cpu.A;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "XOR A,A";
        self.operand_mode = 0;
        4
    }


    fn xor_n_a8(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A ^ cpu.B;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "XOR A,B";
        self.operand_mode = 0;
        4
    }


    fn xor_n_a9(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A ^ cpu.C;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "XOR A,C";
        self.operand_mode = 0;
        4
    }


    fn xor_n_aa(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A ^ cpu.D;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "XOR A,D";
        self.operand_mode = 0;
        4
    }


    fn xor_n_ab(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A ^ cpu.E;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "XOR A,E";
        self.operand_mode = 0;
        4
    }


    fn xor_n_ac(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A ^ cpu.H;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "XOR A,H";
        self.operand_mode = 0;
        4
    }


    fn xor_n_ad(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A ^ cpu.L;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "XOR A,L";
        self.operand_mode = 0;
        4
    }


    fn xor_n_ae(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A ^ cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.last_instruction = "XOR A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn xor_n_ee(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.A = cpu.A ^ data;
        
        if cpu.A == 0 {
            cpu.set_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        } else {
            cpu.reset_flag("Z");
            cpu.reset_flag("N");
            cpu.reset_flag("H");
            cpu.reset_flag("C");
        }

        self.rhs = data as u16;
        self.last_instruction = "XOR A,";
        self.operand_mode = 1;
        8
    }


    fn add_an_87(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let b : u16 = cpu.A as u16;

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.A;
        }

        self.last_instruction = "ADD A,A";
        self.operand_mode = 0;
        4
    }


    fn add_an_80(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let b : u16 = cpu.B as u16;

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.B;
        }

        self.last_instruction = "ADD A,B";
        self.operand_mode = 0;
        4
    }


    fn add_an_81(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let b : u16 = cpu.C as u16;

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.C;
        }

        self.last_instruction = "ADD A,C";
        self.operand_mode = 0;
        4
    }


    fn add_an_82(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let b : u16 = cpu.D as u16;

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.D;
        }

        self.last_instruction = "ADD A,D";
        self.operand_mode = 0;
        4
    }


    fn add_an_83(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let b : u16 = cpu.E as u16;

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.E;
        }

        self.last_instruction = "ADD A,E";
        self.operand_mode = 0;
        4
    }


    fn add_an_84(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let b : u16 = cpu.H as u16;

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.H;
        }

        self.last_instruction = "ADD A,H";
        self.operand_mode = 0;
        4
    }


    fn add_an_85(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let b : u16 = cpu.L as u16;

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.L;
        }

        self.last_instruction = "ADD A,L";
        self.operand_mode = 0;
        4
    }


    fn add_an_86(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let b : u16 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] as u16;

if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];
        }

        self.last_instruction = "ADD A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn add_an_c6(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let b : u16 = self.fetch(cpu) as u16;

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += b as u8;
        }

        self.rhs = b;
        self.last_instruction = "ADD A,";
        self.operand_mode = 1;
        8
    }


    fn adc_an_8f(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let mut b : u16 = cpu.A as u16;
        let mut carry : u8 = cpu.FLAG & 0b0001_0000;

        if carry > 0 {
            carry = 1;
            b += carry as u16;
        }

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.A + carry;
        }

        self.last_instruction = "ADC A,A";
        self.operand_mode = 0;
        4
    }


    fn adc_an_88(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let mut b : u16 = cpu.B as u16;
        let mut carry : u8 = cpu.FLAG & 0b0001_0000;

        if carry > 0 {
            carry = 1;
            b += carry as u16;
        }

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.B + carry;
        }

        self.last_instruction = "ADC A,B";
        self.operand_mode = 0;
        4
    }


    fn adc_an_89(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let mut b : u16 = cpu.C as u16;
        let mut carry : u8 = cpu.FLAG & 0b0001_0000;

        if carry > 0 {
            carry = 1;
            b += carry as u16;
        }

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.C + carry;
        }

        self.last_instruction = "ADC A,C";
        self.operand_mode = 0;
        4
    }


    fn adc_an_8a(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let mut b : u16 = cpu.D as u16;
        let mut carry : u8 = cpu.FLAG & 0b0001_0000;

        if carry > 0 {
            carry = 1;
            b += carry as u16;
        }

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.D + carry;
        }

        self.last_instruction = "ADC A,D";
        self.operand_mode = 0;
        4
    }


    fn adc_an_8b(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let mut b : u16 = cpu.E as u16;
        let mut carry : u8 = cpu.FLAG & 0b0001_0000;

        if carry > 0 {
            carry = 1;
            b += carry as u16;
        }

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.E + carry;
        }

        self.last_instruction = "ADC A,E";
        self.operand_mode = 0;
        4
    }


    fn adc_an_8c(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let mut b : u16 = cpu.H as u16;
        let mut carry : u8 = cpu.FLAG & 0b0001_0000;

        if carry > 0 {
            carry = 1;
            b += carry as u16;
        }

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.H + carry;
        }

        self.last_instruction = "ADC A,H";
        self.operand_mode = 0;
        4
    }


    fn adc_an_8d(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let mut b : u16 = cpu.L as u16;
        let mut carry : u8 = cpu.FLAG & 0b0001_0000;

        if carry > 0 {
            carry = 1;
            b += carry as u16;
        }

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.L + carry;
        }

        self.last_instruction = "ADC A,L";
        self.operand_mode = 0;
        4
    }


    fn adc_an_8e(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let mut b : u16 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] as u16;
        let mut carry : u8 = cpu.FLAG & 0b0001_0000;

        if carry > 0 {
            carry = 1;
            b += carry as u16;
        }

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] + carry;
        }

        self.last_instruction = "ADC A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn adc_an_ce(&mut self, cpu : &mut CPU) -> u8 {
        let a : u16 = cpu.A as u16;
        let mut b : u16 = self.fetch(cpu) as u16;
        let mut carry : u8 = cpu.FLAG & 0b0001_0000;

        if carry > 0 {
            carry = 1;
            b += carry as u16;
        }

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a + b == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a + b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A += b as u8 + carry;
        }

        self.rhs = b;
        self.last_instruction = "ADC A,";
        self.operand_mode = 1;
        8
    }


    fn add_hl_n_09(&mut self, cpu : &mut CPU) -> u8 {
        let mut a : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let b : u16 = Opcode::byte_cat(cpu.B, cpu.C);

        if (((a & 0x0fff) + (b & 0x0fff)) & 0x1000) == 0x1000 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.reset_flag("N");    // reset N flag

        if (a + b) as u32 > 65535 {
            cpu.set_flag("C");
            a = 65535;
        } else {
            cpu.reset_flag("C");
            a += b;
        }

        cpu.H = (a >> 8) as u8;
        cpu.L = (a & 0x00FF) as u8;

        self.last_instruction = "ADD HL,BC";
        self.operand_mode = 0;
        8
    }


    fn add_hl_n_19(&mut self, cpu : &mut CPU) -> u8 {
        let mut a : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let b : u16 = Opcode::byte_cat(cpu.D, cpu.E);

        if (((a & 0x0fff) + (b & 0x0fff)) & 0x1000) == 0x1000 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.reset_flag("N");    // reset N flag

        if (a + b) as u32 > 65535 {
            cpu.set_flag("C");
            a = 65535;
        } else {
            cpu.reset_flag("C");
            a += b;
        }

        cpu.H = (a >> 8) as u8;
        cpu.L = (a & 0x00FF) as u8;

        self.last_instruction = "ADD HL,DE";
        self.operand_mode = 0;
        8
    }


    fn add_hl_n_29(&mut self, cpu : &mut CPU) -> u8 {
        let mut a : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let b : u16 = Opcode::byte_cat(cpu.H, cpu.L);

        if (((a & 0x0fff) + (b & 0x0fff)) & 0x1000) == 0x1000 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.reset_flag("N");    // reset N flag

        if (a + b) as u32 > 65535 {
            cpu.set_flag("C");
            a = 65535;
        } else {
            cpu.reset_flag("C");
            a += b;
        }

        cpu.H = (a >> 8) as u8;
        cpu.L = (a & 0x00FF) as u8;

        self.last_instruction = "ADD HL,HL";
        self.operand_mode = 0;
        8
    }


    fn add_hl_n_39(&mut self, cpu : &mut CPU) -> u8 {
        let mut a : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let b : u16 = cpu.SP;

        if (((a & 0x0fff) + (b & 0x0fff)) & 0x1000) == 0x1000 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.reset_flag("N");    // reset N flag

        if (a + b) as u32 > 65535 {
            cpu.set_flag("C");
            a = 65535;
        } else {
            cpu.reset_flag("C");
            a += b;
        }

        cpu.H = (a >> 8) as u8;
        cpu.L = (a & 0x00FF) as u8;

        self.last_instruction = "ADD HL,SP";
        self.operand_mode = 0;
        8
    }


    fn call_nn_cd(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        let pc_h : u8 = (cpu.PC >> 8) as u8;
        let pc_l : u8 = (cpu.PC & 0x00FF) as u8;

        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);

        cpu.PC = Opcode::byte_cat(h,l);
        
        self.rhs = Opcode::byte_cat(h,l);
        self.last_instruction = "CALL,";
        self.operand_mode = 1;
        12
    }


    fn jr_cc_n_20(&mut self, cpu : &mut CPU) -> u8 {
        let n : i8 = self.fetch(cpu) as i8;

        if cpu.get_flag("Z") == 0 && n <= 0 {
            cpu.PC = cpu.PC - (-n) as u16;
            self.rhs = (-n) as u16;
            self.last_instruction = "JR, -";
        }

        if cpu.get_flag("Z") == 0 && n > 0 {
            cpu.PC = cpu.PC + (n as u16);
            self.rhs = n as u16;
            self.last_instruction = "JR,";
        }

        
        self.operand_mode = 1;
        8
    }








}
