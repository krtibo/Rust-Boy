#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use cpu::CPU;

pub struct Opcode {
    pub lhs : u16,
    pub rhs : u16,
    pub operand_mode : u8,
    pub opc : [fn(&mut Opcode, &mut CPU) -> u8; 256],
    pub cb_opc : [fn(&mut Opcode, &mut CPU) -> u8; 256],
    pub last_instruction :  &'static str,
    pub last_opcode : u8,
    pub disable_int : bool,
    pub enable_int : bool,
    pub count_disable_int : u8,
    pub count_enable_int : u8,
    pub last_opcode_cb : bool,
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
            last_opcode : 0,
            disable_int : false,
            enable_int : false,
            count_disable_int : 0,
            count_enable_int : 0,
            last_opcode_cb : false,
        }
    }


    pub fn init(&mut self) {
        self.opc[0x00] = Opcode::nop_00;
        self.opc[0xcb] = Opcode::prefix_cb;
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
        self.opc[0x36] = Opcode::ld_r1r2_36;
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
        self.opc[0xe8] = Opcode::add_sp_n_e8;
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

        self.opc[0x20] = Opcode::jr_cc_n_20;
        self.opc[0x28] = Opcode::jr_cc_n_28;
        self.opc[0x30] = Opcode::jr_cc_n_30;
        self.opc[0x38] = Opcode::jr_cc_n_38;
        self.opc[0x18] = Opcode::jr_n_18;
        self.opc[0xe9] = Opcode::jp_hl_e9;
        self.opc[0xc2] = Opcode::jp_cc_nn_c2;
        self.opc[0xca] = Opcode::jp_cc_nn_ca;
        self.opc[0xd2] = Opcode::jp_cc_nn_d2;
        self.opc[0xda] = Opcode::jp_cc_nn_da;
        self.opc[0xc3] = Opcode::jp_nn_c3;

        self.opc[0xcd] = Opcode::call_nn_cd;
        self.opc[0xc4] = Opcode::call_cc_nn_c4;
        self.opc[0xcc] = Opcode::call_cc_nn_cc;
        self.opc[0xd4] = Opcode::call_cc_nn_d4;
        self.opc[0xdc] = Opcode::call_cc_nn_dc;

        self.opc[0xc7] = Opcode::rst_c7;
        self.opc[0xcf] = Opcode::rst_cf;
        self.opc[0xd7] = Opcode::rst_d7;
        self.opc[0xdf] = Opcode::rst_df;
        self.opc[0xe7] = Opcode::rst_e7;
        self.opc[0xef] = Opcode::rst_ef;
        self.opc[0xf7] = Opcode::rst_f7;
        self.opc[0xff] = Opcode::rst_ff;

        self.opc[0xc9] = Opcode::ret_c9;
        self.opc[0xd9] = Opcode::reti_d9;
        self.opc[0xc0] = Opcode::ret_cc_c0;
        self.opc[0xc8] = Opcode::ret_cc_c8;
        self.opc[0xd0] = Opcode::ret_cc_d0;
        self.opc[0xd8] = Opcode::ret_cc_d8;
        self.opc[0x76] = Opcode::halt_76;


        self.opc[0x3f] = Opcode::ccf_3f;
        self.opc[0x37] = Opcode::scf_37;
        self.opc[0xf3] = Opcode::di_f3;
        self.opc[0xfb] = Opcode::ei_fb;

        self.opc[0x97] = Opcode::sub_a_97;
        self.opc[0x90] = Opcode::sub_a_90;
        self.opc[0x91] = Opcode::sub_a_91;
        self.opc[0x92] = Opcode::sub_a_92;
        self.opc[0x93] = Opcode::sub_a_93;
        self.opc[0x94] = Opcode::sub_a_94;
        self.opc[0x95] = Opcode::sub_a_95;
        self.opc[0x96] = Opcode::sub_a_96;
        self.opc[0xd6] = Opcode::sub_a_d6;

        self.opc[0x9f] = Opcode::sbc_an_9f;
        self.opc[0x98] = Opcode::sbc_an_98;
        self.opc[0x99] = Opcode::sbc_an_99;
        self.opc[0x9a] = Opcode::sbc_an_9a;
        self.opc[0x9b] = Opcode::sbc_an_9b;
        self.opc[0x9c] = Opcode::sbc_an_9c;
        self.opc[0x9d] = Opcode::sbc_an_9d;
        self.opc[0x9e] = Opcode::sbc_an_9e;
        self.opc[0xde] = Opcode::sbc_an_de;

        self.opc[0xbf] = Opcode::cp_n_bf;
        self.opc[0xb8] = Opcode::cp_n_b8;
        self.opc[0xb9] = Opcode::cp_n_b9;
        self.opc[0xba] = Opcode::cp_n_ba;
        self.opc[0xbb] = Opcode::cp_n_bb;
        self.opc[0xbc] = Opcode::cp_n_bc;
        self.opc[0xbd] = Opcode::cp_n_bd;
        self.opc[0xbe] = Opcode::cp_n_be;
        self.opc[0xfe] = Opcode::cp_n_fe;

        self.opc[0x07] = Opcode::rlca_07;
        self.opc[0x17] = Opcode::rla_17;
        self.opc[0x0f] = Opcode::rrca_0f;
        self.opc[0x1f] = Opcode::rra_1f;
        self.opc[0x2f] = Opcode::cpl_2f;
        self.opc[0x27] = Opcode::daa_27;










        self.cb_opc[0x11] = Opcode::cb_rl_11;
        self.cb_opc[0x40] = Opcode::cb_bit_40;
        self.cb_opc[0x50] = Opcode::cb_bit_50;
        self.cb_opc[0x60] = Opcode::cb_bit_60;
        self.cb_opc[0x70] = Opcode::cb_bit_70;
        self.cb_opc[0x41] = Opcode::cb_bit_41;
        self.cb_opc[0x51] = Opcode::cb_bit_51;
        self.cb_opc[0x61] = Opcode::cb_bit_61;
        self.cb_opc[0x71] = Opcode::cb_bit_71;
        self.cb_opc[0x42] = Opcode::cb_bit_42;
        self.cb_opc[0x52] = Opcode::cb_bit_52;
        self.cb_opc[0x62] = Opcode::cb_bit_62;
        self.cb_opc[0x72] = Opcode::cb_bit_72;
        self.cb_opc[0x43] = Opcode::cb_bit_43;
        self.cb_opc[0x53] = Opcode::cb_bit_53;
        self.cb_opc[0x63] = Opcode::cb_bit_63;
        self.cb_opc[0x73] = Opcode::cb_bit_73;
        self.cb_opc[0x44] = Opcode::cb_bit_44;
        self.cb_opc[0x54] = Opcode::cb_bit_54;
        self.cb_opc[0x64] = Opcode::cb_bit_64;
        self.cb_opc[0x74] = Opcode::cb_bit_74;
        self.cb_opc[0x45] = Opcode::cb_bit_45;
        self.cb_opc[0x55] = Opcode::cb_bit_55;
        self.cb_opc[0x65] = Opcode::cb_bit_65;
        self.cb_opc[0x75] = Opcode::cb_bit_75;
        self.cb_opc[0x46] = Opcode::cb_bit_46;
        self.cb_opc[0x56] = Opcode::cb_bit_56;
        self.cb_opc[0x66] = Opcode::cb_bit_66;
        self.cb_opc[0x76] = Opcode::cb_bit_76;
        self.cb_opc[0x47] = Opcode::cb_bit_47;
        self.cb_opc[0x57] = Opcode::cb_bit_57;
        self.cb_opc[0x67] = Opcode::cb_bit_67;
        self.cb_opc[0x77] = Opcode::cb_bit_77;
        self.cb_opc[0x48] = Opcode::cb_bit_48;
        self.cb_opc[0x58] = Opcode::cb_bit_58;
        self.cb_opc[0x68] = Opcode::cb_bit_68;
        self.cb_opc[0x78] = Opcode::cb_bit_78;
        self.cb_opc[0x49] = Opcode::cb_bit_49;
        self.cb_opc[0x59] = Opcode::cb_bit_59;
        self.cb_opc[0x69] = Opcode::cb_bit_69;
        self.cb_opc[0x79] = Opcode::cb_bit_79;
        self.cb_opc[0x4a] = Opcode::cb_bit_4a;
        self.cb_opc[0x5a] = Opcode::cb_bit_5a;
        self.cb_opc[0x6a] = Opcode::cb_bit_6a;
        self.cb_opc[0x7a] = Opcode::cb_bit_7a;
        self.cb_opc[0x4b] = Opcode::cb_bit_4b;
        self.cb_opc[0x5b] = Opcode::cb_bit_5b;
        self.cb_opc[0x6b] = Opcode::cb_bit_6b;
        self.cb_opc[0x7b] = Opcode::cb_bit_7b;
        self.cb_opc[0x4c] = Opcode::cb_bit_4c;
        self.cb_opc[0x5c] = Opcode::cb_bit_5c;
        self.cb_opc[0x6c] = Opcode::cb_bit_6c;
        self.cb_opc[0x7c] = Opcode::cb_bit_7c;
        self.cb_opc[0x4d] = Opcode::cb_bit_4d;
        self.cb_opc[0x5d] = Opcode::cb_bit_5d;
        self.cb_opc[0x6d] = Opcode::cb_bit_6d;
        self.cb_opc[0x7d] = Opcode::cb_bit_7d;
        self.cb_opc[0x4e] = Opcode::cb_bit_4e;
        self.cb_opc[0x5e] = Opcode::cb_bit_5e;
        self.cb_opc[0x6e] = Opcode::cb_bit_6e;
        self.cb_opc[0x7e] = Opcode::cb_bit_7e;
        self.cb_opc[0x4f] = Opcode::cb_bit_4f;
        self.cb_opc[0x5f] = Opcode::cb_bit_5f;
        self.cb_opc[0x6f] = Opcode::cb_bit_6f;
        self.cb_opc[0x7f] = Opcode::cb_bit_7f;

        self.cb_opc[0x80] = Opcode::cb_res_80;
        self.cb_opc[0x90] = Opcode::cb_res_90;
        self.cb_opc[0xa0] = Opcode::cb_res_a0;
        self.cb_opc[0xb0] = Opcode::cb_res_b0;
        self.cb_opc[0x81] = Opcode::cb_res_81;
        self.cb_opc[0x91] = Opcode::cb_res_91;
        self.cb_opc[0xa1] = Opcode::cb_res_a1;
        self.cb_opc[0xb1] = Opcode::cb_res_b1;
        self.cb_opc[0x82] = Opcode::cb_res_82;
        self.cb_opc[0x92] = Opcode::cb_res_92;
        self.cb_opc[0xa2] = Opcode::cb_res_a2;
        self.cb_opc[0xb2] = Opcode::cb_res_b2;
        self.cb_opc[0x83] = Opcode::cb_res_83;
        self.cb_opc[0x93] = Opcode::cb_res_93;
        self.cb_opc[0xa3] = Opcode::cb_res_a3;
        self.cb_opc[0xb3] = Opcode::cb_res_b3;
        self.cb_opc[0x84] = Opcode::cb_res_84;
        self.cb_opc[0x94] = Opcode::cb_res_94;
        self.cb_opc[0xa4] = Opcode::cb_res_a4;
        self.cb_opc[0xb4] = Opcode::cb_res_b4;
        self.cb_opc[0x85] = Opcode::cb_res_85;
        self.cb_opc[0x95] = Opcode::cb_res_95;
        self.cb_opc[0xa5] = Opcode::cb_res_a5;
        self.cb_opc[0xb5] = Opcode::cb_res_b5;
        self.cb_opc[0x86] = Opcode::cb_res_86;
        self.cb_opc[0x96] = Opcode::cb_res_96;
        self.cb_opc[0xa6] = Opcode::cb_res_a6;
        self.cb_opc[0xb6] = Opcode::cb_res_b6;
        self.cb_opc[0x87] = Opcode::cb_res_87;
        self.cb_opc[0x97] = Opcode::cb_res_97;
        self.cb_opc[0xa7] = Opcode::cb_res_a7;
        self.cb_opc[0xb7] = Opcode::cb_res_b7;
        self.cb_opc[0x88] = Opcode::cb_res_88;
        self.cb_opc[0x98] = Opcode::cb_res_98;
        self.cb_opc[0xa8] = Opcode::cb_res_a8;
        self.cb_opc[0xb8] = Opcode::cb_res_b8;
        self.cb_opc[0x89] = Opcode::cb_res_89;
        self.cb_opc[0x99] = Opcode::cb_res_99;
        self.cb_opc[0xa9] = Opcode::cb_res_a9;
        self.cb_opc[0xb9] = Opcode::cb_res_b9;
        self.cb_opc[0x8a] = Opcode::cb_res_8a;
        self.cb_opc[0x9a] = Opcode::cb_res_9a;
        self.cb_opc[0xaa] = Opcode::cb_res_aa;
        self.cb_opc[0xba] = Opcode::cb_res_ba;
        self.cb_opc[0x8b] = Opcode::cb_res_8b;
        self.cb_opc[0x9b] = Opcode::cb_res_9b;
        self.cb_opc[0xab] = Opcode::cb_res_ab;
        self.cb_opc[0xbb] = Opcode::cb_res_bb;
        self.cb_opc[0x8c] = Opcode::cb_res_8c;
        self.cb_opc[0x9c] = Opcode::cb_res_9c;
        self.cb_opc[0xac] = Opcode::cb_res_ac;
        self.cb_opc[0xbc] = Opcode::cb_res_bc;
        self.cb_opc[0x8d] = Opcode::cb_res_8d;
        self.cb_opc[0x9d] = Opcode::cb_res_9d;
        self.cb_opc[0xad] = Opcode::cb_res_ad;
        self.cb_opc[0xbd] = Opcode::cb_res_bd;
        self.cb_opc[0x8e] = Opcode::cb_res_8e;
        self.cb_opc[0x9e] = Opcode::cb_res_9e;
        self.cb_opc[0xae] = Opcode::cb_res_ae;
        self.cb_opc[0xbe] = Opcode::cb_res_be;
        self.cb_opc[0x8f] = Opcode::cb_res_8f;
        self.cb_opc[0x9f] = Opcode::cb_res_9f;
        self.cb_opc[0xaf] = Opcode::cb_res_af;
        self.cb_opc[0xbf] = Opcode::cb_res_bf;

        self.cb_opc[0xc0] = Opcode::cb_set_c0;
        self.cb_opc[0xd0] = Opcode::cb_set_d0;
        self.cb_opc[0xe0] = Opcode::cb_set_e0;
        self.cb_opc[0xf0] = Opcode::cb_set_f0;
        self.cb_opc[0xc1] = Opcode::cb_set_c1;
        self.cb_opc[0xd1] = Opcode::cb_set_d1;
        self.cb_opc[0xe1] = Opcode::cb_set_e1;
        self.cb_opc[0xf1] = Opcode::cb_set_f1;
        self.cb_opc[0xc2] = Opcode::cb_set_c2;
        self.cb_opc[0xd2] = Opcode::cb_set_d2;
        self.cb_opc[0xe2] = Opcode::cb_set_e2;
        self.cb_opc[0xf2] = Opcode::cb_set_f2;
        self.cb_opc[0xc3] = Opcode::cb_set_c3;
        self.cb_opc[0xd3] = Opcode::cb_set_d3;
        self.cb_opc[0xe3] = Opcode::cb_set_e3;
        self.cb_opc[0xf3] = Opcode::cb_set_f3;
        self.cb_opc[0xc4] = Opcode::cb_set_c4;
        self.cb_opc[0xd4] = Opcode::cb_set_d4;
        self.cb_opc[0xe4] = Opcode::cb_set_e4;
        self.cb_opc[0xf4] = Opcode::cb_set_f4;
        self.cb_opc[0xc5] = Opcode::cb_set_c5;
        self.cb_opc[0xd5] = Opcode::cb_set_d5;
        self.cb_opc[0xe5] = Opcode::cb_set_e5;
        self.cb_opc[0xf5] = Opcode::cb_set_f5;
        self.cb_opc[0xc6] = Opcode::cb_set_c6;
        self.cb_opc[0xd6] = Opcode::cb_set_d6;
        self.cb_opc[0xe6] = Opcode::cb_set_e6;
        self.cb_opc[0xf6] = Opcode::cb_set_f6;
        self.cb_opc[0xc7] = Opcode::cb_set_c7;
        self.cb_opc[0xd7] = Opcode::cb_set_d7;
        self.cb_opc[0xe7] = Opcode::cb_set_e7;
        self.cb_opc[0xf7] = Opcode::cb_set_f7;
        self.cb_opc[0xc8] = Opcode::cb_set_c8;
        self.cb_opc[0xd8] = Opcode::cb_set_d8;
        self.cb_opc[0xe8] = Opcode::cb_set_e8;
        self.cb_opc[0xf8] = Opcode::cb_set_f8;
        self.cb_opc[0xc9] = Opcode::cb_set_c9;
        self.cb_opc[0xd9] = Opcode::cb_set_d9;
        self.cb_opc[0xe9] = Opcode::cb_set_e9;
        self.cb_opc[0xf9] = Opcode::cb_set_f9;
        self.cb_opc[0xca] = Opcode::cb_set_ca;
        self.cb_opc[0xda] = Opcode::cb_set_da;
        self.cb_opc[0xea] = Opcode::cb_set_ea;
        self.cb_opc[0xfa] = Opcode::cb_set_fa;
        self.cb_opc[0xcb] = Opcode::cb_set_cb;
        self.cb_opc[0xdb] = Opcode::cb_set_db;
        self.cb_opc[0xeb] = Opcode::cb_set_eb;
        self.cb_opc[0xfb] = Opcode::cb_set_fb;
        self.cb_opc[0xcc] = Opcode::cb_set_cc;
        self.cb_opc[0xdc] = Opcode::cb_set_dc;
        self.cb_opc[0xec] = Opcode::cb_set_ec;
        self.cb_opc[0xfc] = Opcode::cb_set_fc;
        self.cb_opc[0xcd] = Opcode::cb_set_cd;
        self.cb_opc[0xdd] = Opcode::cb_set_dd;
        self.cb_opc[0xed] = Opcode::cb_set_ed;
        self.cb_opc[0xfd] = Opcode::cb_set_fd;
        self.cb_opc[0xce] = Opcode::cb_set_ce;
        self.cb_opc[0xde] = Opcode::cb_set_de;
        self.cb_opc[0xee] = Opcode::cb_set_ee;
        self.cb_opc[0xfe] = Opcode::cb_set_fe;
        self.cb_opc[0xcf] = Opcode::cb_set_cf;
        self.cb_opc[0xdf] = Opcode::cb_set_df;
        self.cb_opc[0xef] = Opcode::cb_set_ef;
        self.cb_opc[0xff] = Opcode::cb_set_ff;

        self.cb_opc[0x30] = Opcode::cb_swap_30;
        self.cb_opc[0x31] = Opcode::cb_swap_31;
        self.cb_opc[0x32] = Opcode::cb_swap_32;
        self.cb_opc[0x33] = Opcode::cb_swap_33;
        self.cb_opc[0x34] = Opcode::cb_swap_34;
        self.cb_opc[0x35] = Opcode::cb_swap_35;
        self.cb_opc[0x36] = Opcode::cb_swap_36;
        self.cb_opc[0x37] = Opcode::cb_swap_37;

        self.cb_opc[0x20] = Opcode::cb_sla_20;
        self.cb_opc[0x21] = Opcode::cb_sla_21;
        self.cb_opc[0x22] = Opcode::cb_sla_22;
        self.cb_opc[0x23] = Opcode::cb_sla_23;
        self.cb_opc[0x24] = Opcode::cb_sla_24;
        self.cb_opc[0x25] = Opcode::cb_sla_25;
        self.cb_opc[0x26] = Opcode::cb_sla_26;
        self.cb_opc[0x27] = Opcode::cb_sla_27;

        self.cb_opc[0x38] = Opcode::cb_srl_38;
        self.cb_opc[0x39] = Opcode::cb_srl_39;
        self.cb_opc[0x3a] = Opcode::cb_srl_3a;
        self.cb_opc[0x3b] = Opcode::cb_srl_3b;
        self.cb_opc[0x3c] = Opcode::cb_srl_3c;
        self.cb_opc[0x3d] = Opcode::cb_srl_3d;
        self.cb_opc[0x3e] = Opcode::cb_srl_3e;
        self.cb_opc[0x3f] = Opcode::cb_srl_3f;
        self.cb_opc[0x28] = Opcode::cb_sra_28;
        self.cb_opc[0x29] = Opcode::cb_sra_29;
        self.cb_opc[0x2a] = Opcode::cb_sra_2a;
        self.cb_opc[0x2b] = Opcode::cb_sra_2b;
        self.cb_opc[0x2c] = Opcode::cb_sra_2c;
        self.cb_opc[0x2d] = Opcode::cb_sra_2d;
        self.cb_opc[0x2e] = Opcode::cb_sra_2e;
        self.cb_opc[0x2f] = Opcode::cb_sra_2f;

        self.cb_opc[0x00] = Opcode::cb_rlc_00;
        self.cb_opc[0x01] = Opcode::cb_rlc_01;
        self.cb_opc[0x02] = Opcode::cb_rlc_02;
        self.cb_opc[0x03] = Opcode::cb_rlc_03;
        self.cb_opc[0x04] = Opcode::cb_rlc_04;
        self.cb_opc[0x05] = Opcode::cb_rlc_05;
        self.cb_opc[0x06] = Opcode::cb_rlc_06;
        self.cb_opc[0x07] = Opcode::cb_rlc_07;
        self.cb_opc[0x08] = Opcode::cb_rrc_08;
        self.cb_opc[0x09] = Opcode::cb_rrc_09;
        self.cb_opc[0x0a] = Opcode::cb_rrc_0a;
        self.cb_opc[0x0b] = Opcode::cb_rrc_0b;
        self.cb_opc[0x0c] = Opcode::cb_rrc_0c;
        self.cb_opc[0x0d] = Opcode::cb_rrc_0d;
        self.cb_opc[0x0e] = Opcode::cb_rrc_0e;
        self.cb_opc[0x0f] = Opcode::cb_rrc_0f;

        self.cb_opc[0x10] = Opcode::cb_rl_10;
        self.cb_opc[0x11] = Opcode::cb_rl_11;
        self.cb_opc[0x12] = Opcode::cb_rl_12;
        self.cb_opc[0x13] = Opcode::cb_rl_13;
        self.cb_opc[0x14] = Opcode::cb_rl_14;
        self.cb_opc[0x15] = Opcode::cb_rl_15;
        self.cb_opc[0x16] = Opcode::cb_rl_16;
        self.cb_opc[0x17] = Opcode::cb_rl_17;
        self.cb_opc[0x18] = Opcode::cb_rr_18;
        self.cb_opc[0x19] = Opcode::cb_rr_19;
        self.cb_opc[0x1a] = Opcode::cb_rr_1a;
        self.cb_opc[0x1b] = Opcode::cb_rr_1b;
        self.cb_opc[0x1c] = Opcode::cb_rr_1c;
        self.cb_opc[0x1d] = Opcode::cb_rr_1d;
        self.cb_opc[0x1e] = Opcode::cb_rr_1e;
        self.cb_opc[0x1f] = Opcode::cb_rr_1f;



    }


    // fetch the opcode and the byte after
    pub fn fetch(&mut self, cpu : &mut CPU) -> u8 {
        // increase the Program Counter
        let ret = cpu.RAM[(cpu.PC) as usize];
        cpu.PC += 1;
        ret
    }


    pub fn execute(&mut self, cpu : &mut CPU) -> u8 {

        self.lhs = 0x0000;
        self.rhs = 0x0000;

        // EI, DI instruction delay handler

        if self.disable_int && self.count_disable_int < 2 {
            self.count_disable_int += 1;
        }

        if self.disable_int && self.count_disable_int == 2 {
            self.disable_int = false;
            self.count_disable_int = 0;
            //cpu.RAM[0xFFFF as usize] = 0b00011111; // enable interrupts
            cpu.IR = true;
        }

        if self.enable_int && self.count_enable_int < 2 {
            self.count_enable_int += 1;
        }

        if self.enable_int && self.count_enable_int == 2 {
            self.enable_int = false;
            self.count_enable_int = 0;
            //cpu.RAM[0xFFFF as usize] = 0b00000000; // disable interrupts
            cpu.IR = false;
        }

        // fetcher
        self.last_opcode_cb = false;
        self.last_opcode = self.fetch(cpu);
        self.opc[self.last_opcode as usize](self, cpu)
    }

    fn byte_cat(h : u8, l : u8) -> u16 {
        let high : u16 = (h as u16) << 8;
        let low : u16 = l as u16;
        let hl : u16 = high | low;
        hl
    }

    //////  O P C O D E S  //////


    fn default(&mut self, cpu : &mut CPU) -> u8 {
        if self.last_opcode_cb {
            self.last_instruction = "CB - UNKNOWN OPCODE";
        } else {
            self.last_instruction = "UNKNOWN OPCODE";
        }

        self.operand_mode = 0;
        0
    }

    fn nop_00(&mut self, cpu : &mut CPU) -> u8 {
        self.last_instruction = "NOP";
        self.operand_mode = 0;
        4
    }

    fn prefix_cb(&mut self, cpu : &mut CPU) -> u8 {


        let cb_opcode : u8 = self.fetch(cpu);
        self.last_opcode = cb_opcode;
        self.last_opcode_cb = true;
        //self.rhs = cb_opcode as u16;
        //self.last_instruction = "CB";
        //self.operand_mode = 1;

        self.cb_opc[cb_opcode as usize](self, cpu);
        //println!("CB OPC: {:x}", cb_opcode );

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
        let hl = Opcode::byte_cat(cpu.H, cpu.L);
        let b = cpu.B;
        cpu.write_ram(hl, b);

        self.last_instruction = "LD (HL), B";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_71(&mut self, cpu : &mut CPU) -> u8 {
        let hl = Opcode::byte_cat(cpu.H, cpu.L);
        let c = cpu.C;
        cpu.write_ram(hl, c);

        self.last_instruction = "LD (HL), C";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_72(&mut self, cpu : &mut CPU) -> u8 {
        let hl = Opcode::byte_cat(cpu.H, cpu.L);
        let d = cpu.D;
        cpu.write_ram(hl, d);

        self.last_instruction = "LD (HL), D";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_73(&mut self, cpu : &mut CPU) -> u8 {
        let hl = Opcode::byte_cat(cpu.H, cpu.L);
        let e = cpu.E;
        cpu.write_ram(hl, e);

        self.last_instruction = "LD (HL), E";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_74(&mut self, cpu : &mut CPU) -> u8 {
        let hl = Opcode::byte_cat(cpu.H, cpu.L);
        let h = cpu.H;
        cpu.write_ram(hl, h);

        self.last_instruction = "LD (HL), H";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_75(&mut self, cpu : &mut CPU) -> u8 {
        let hl = Opcode::byte_cat(cpu.H, cpu.L);
        let l = cpu.L;
        cpu.write_ram(hl, l);

        self.last_instruction = "LD (HL), L";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_36(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        let hl = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.write_ram(hl, data);

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
        let bc = Opcode::byte_cat(cpu.B, cpu.C);
        let a = cpu.A;
        cpu.write_ram(bc, a);

        self.last_instruction = "LD (BC),A";
        self.operand_mode = 0;
        8
    }


    fn ld_na_12(&mut self, cpu : &mut CPU) -> u8 {
        let de = Opcode::byte_cat(cpu.D, cpu.E);
        let a = cpu.A;
        cpu.write_ram(de, a);

        self.last_instruction = "LD (DE),A";
        self.operand_mode = 0;
        8
    }


    fn ld_na_77(&mut self, cpu : &mut CPU) -> u8 {
        let hl = Opcode::byte_cat(cpu.H, cpu.L);
        let a = cpu.A;
        cpu.write_ram(hl, a);

        self.last_instruction = "LD (HL),A";
        self.operand_mode = 0;
        8
    }


    fn ld_na_ea(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);

        let hl = Opcode::byte_cat(data_h, data_l);
        let a = cpu.A;
        cpu.write_ram(hl, a);

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
        let addr = 0xFF00 + cpu.C as u16;
        let a = cpu.A;
        cpu.write_ram(addr, a);

        self.last_instruction = "LD (C),A";
        self.operand_mode = 0;
        8
    }


    fn ldd_ahl_3a(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.A = cpu.RAM[hl as usize];
        hl = hl.wrapping_sub(1);
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDD A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn ldd_hla_32(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let a = cpu.A;
        cpu.write_ram(hl, a);

        hl = hl.wrapping_sub(1);
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDD (HL),A";
        self.operand_mode = 0;
        8
    }


    fn ldi_ahl_2a(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.A = cpu.RAM[hl as usize];
        hl = hl.wrapping_add(1);
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDI A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn ldi_hla_22(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let a = cpu.A;
        cpu.write_ram(hl, a);

        hl = hl.wrapping_add(1);
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDD (HL),A";
        self.operand_mode = 0;
        8
    }


    fn ldh_na_e0(&mut self, cpu : &mut CPU) -> u8 {
        let n : u8 = self.fetch(cpu);
        let addr = 0xFF00 + n as u16;
        let a = cpu.A;
        cpu.write_ram(addr, a);

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
        let sp = cpu.SP;
        let addr = Opcode::byte_cat(data_h, data_l);
        cpu.write_ram(addr, (sp & 0x00FF) as u8);
        cpu.write_ram(addr + 1, (sp >> 8) as u8);

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

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.B & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.B = cpu.B.wrapping_add(1);

        if cpu.B == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "INC B";
        self.operand_mode = 0;
        4
    }


    fn inc_c_0c(&mut self, cpu : &mut CPU) -> u8 {

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.C & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.C = cpu.C.wrapping_add(1);

        if cpu.C == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "INC C";
        self.operand_mode = 0;
        4
    }


    fn inc_d_14(&mut self, cpu : &mut CPU) -> u8 {

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.D & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.D = cpu.D.wrapping_add(1);

        if cpu.D == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "INC D";
        self.operand_mode = 0;
        4
    }


    fn inc_e_1c(&mut self, cpu : &mut CPU) -> u8 {

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.E & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.E = cpu.E.wrapping_add(1);

        if cpu.E == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "INC E";
        self.operand_mode = 0;
        4
    }


    fn inc_h_24(&mut self, cpu : &mut CPU) -> u8 {

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.H & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.H = cpu.H.wrapping_add(1);

        if cpu.H == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "INC H";
        self.operand_mode = 0;
        4
    }


    fn inc_l_2c(&mut self, cpu : &mut CPU) -> u8 {

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.L & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.L = cpu.L.wrapping_add(1);

        if cpu.L == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "INC L";
        self.operand_mode = 0;
        4
    }


    fn inc_hl_34(&mut self, cpu : &mut CPU) -> u8 {

        let mut hl : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        cpu.reset_flag("N");    // reset N flag

        if (((hl & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        hl = hl.wrapping_add(1);

        if hl == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        let addr = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.write_ram(addr, hl);


        self.last_instruction = "INC (HL)";
        self.operand_mode = 0;
        12
    }


    fn inc_a_3c(&mut self, cpu : &mut CPU) -> u8 {

        cpu.reset_flag("N");    // reset N flag

        if (((cpu.A & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");    // set H flag
        } else {
            cpu.reset_flag("H");
        }

        cpu.A = cpu.A.wrapping_add(1);

        if cpu.A == 0 {
            cpu.set_flag("Z");    // set Z flag
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "INC A";
        self.operand_mode = 0;
        4
    }


    fn inc_nn_03(&mut self, cpu : &mut CPU) -> u8 {
        let mut bc : u16 = Opcode::byte_cat(cpu.B, cpu.C);
        bc = bc.wrapping_add(1);
        cpu.B = (bc >> 8) as u8;
        cpu.C = (bc & 0x00FF) as u8;

        self.last_instruction = "INC BC";
        self.operand_mode = 0;
        8
    }


    fn inc_nn_13(&mut self, cpu : &mut CPU) -> u8 {
        let mut de : u16 = Opcode::byte_cat(cpu.D, cpu.E);
        de = de.wrapping_add(1);
        cpu.D = (de >> 8) as u8;
        cpu.E = (de & 0x00FF) as u8;

        self.last_instruction = "INC DE";
        self.operand_mode = 0;
        8
    }


    fn inc_nn_23(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);

        hl = hl.wrapping_add(1);
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "INC HL";
        self.operand_mode = 0;
        8
    }


    fn inc_nn_33(&mut self, cpu : &mut CPU) -> u8 {
        cpu.SP = cpu.SP.wrapping_add(1);

        self.last_instruction = "INC SP";
        self.operand_mode = 0;
        8
    }


    fn dec_nn_0b(&mut self, cpu : &mut CPU) -> u8 {
        let mut bc : u16 = Opcode::byte_cat(cpu.B, cpu.C);
        bc = bc.wrapping_sub(1);
        cpu.B = (bc >> 8) as u8;
        cpu.C = (bc & 0x00FF) as u8;

        self.last_instruction = "DEC BC";
        self.operand_mode = 0;
        8
    }


    fn dec_nn_1b(&mut self, cpu : &mut CPU) -> u8 {
        let mut de : u16 = Opcode::byte_cat(cpu.D, cpu.E);
        de = de.wrapping_sub(1);
        cpu.D = (de >> 8) as u8;
        cpu.E = (de & 0x00FF) as u8;

        self.last_instruction = "DEC DE";
        self.operand_mode = 0;
        8
    }


    fn dec_nn_2b(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        hl = hl.wrapping_sub(1);
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "DEC HL";
        self.operand_mode = 0;
        8
    }


    fn dec_nn_3b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.SP = cpu.SP.wrapping_sub(1);

        self.last_instruction = "DEC SP";
        self.operand_mode = 0;
        8
    }


    fn dec_b_05(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.B == 1 {
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

        cpu.B = cpu.B.wrapping_sub(1);


        self.last_instruction = "DEC B";
        self.operand_mode = 0;
        4
    }


    fn dec_c_0d(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.C  == 1 {
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

        cpu.C = cpu.C.wrapping_sub(1);

        self.last_instruction = "DEC C";
        self.operand_mode = 0;
        4
    }


    fn dec_d_15(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.D  == 1 {
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

        cpu.D = cpu.D.wrapping_sub(1);

        self.last_instruction = "DEC D";
        self.operand_mode = 0;
        4
    }


    fn dec_e_1d(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.E == 1 {
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

        cpu.E = cpu.E.wrapping_sub(1);

        self.last_instruction = "DEC E";
        self.operand_mode = 0;
        4
    }


    fn dec_h_25(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.H == 1 {
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

        cpu.H = cpu.H.wrapping_sub(1);

        self.last_instruction = "DEC H";
        self.operand_mode = 0;
        4
    }


    fn dec_l_2d(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.L == 1 {
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

        cpu.L = cpu.L.wrapping_sub(1);

        self.last_instruction = "DEC L";
        self.operand_mode = 0;
        4
    }


    fn dec_hl_35(&mut self, cpu : &mut CPU) -> u8 {

        let mut hl : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        if hl == 1 {
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

        hl = hl.wrapping_sub(1);

        let addr = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.write_ram(addr, hl);

        self.last_instruction = "DEC (HL)";
        self.operand_mode = 0;
        12
    }


    fn dec_a_3d(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.A == 1 {
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

        cpu.A = cpu.A.wrapping_sub(1);

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
        let a : u8 = cpu.A;
        let b : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a.wrapping_add(b) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");

        if (a as u16 + b as u16) > 255 {
            cpu.set_flag("C");
            cpu.A = a.wrapping_add(b);
        } else {
            cpu.reset_flag("C");
            cpu.A = a.wrapping_add(b);
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

        if a.wrapping_add(b) > 255 {
            cpu.set_flag("C");
            cpu.A = 255;
        } else {
            cpu.reset_flag("C");
            cpu.A = cpu.A.wrapping_add(b as u8 + carry);
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
            a = a.wrapping_add(b);
        } else {
            cpu.reset_flag("C");
            a = a.wrapping_add(b);
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
            a = a.wrapping_add(b);
        } else {
            cpu.reset_flag("C");
            a = a.wrapping_add(b);
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
            a = a.wrapping_add(b);
        } else {
            cpu.reset_flag("C");
            a = a.wrapping_add(b);
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
            a = a.wrapping_add(b);
        } else {
            cpu.reset_flag("C");
            a = a.wrapping_add(b);
        }

        cpu.H = (a >> 8) as u8;
        cpu.L = (a & 0x00FF) as u8;

        self.last_instruction = "ADD HL,SP";
        self.operand_mode = 0;
        8
    }


    fn add_sp_n_e8(&mut self, cpu : &mut CPU) -> u8 {
        let mut n : i8 = self.fetch(cpu) as i8;

        cpu.reset_flag("Z");
        cpu.reset_flag("N");

        if n < 0 {

            self.last_instruction = "ADD SP,-";

            n = -n;

            if (cpu.SP & 0x0FFF) < (n as u16 & 0x0FFF) {
                cpu.set_flag("H");
            } else {
                cpu.reset_flag("H");
            }

            if cpu.SP < n as u16 {
                cpu.set_flag("C");
            } else {
                cpu.reset_flag("C");
            }

            cpu.SP = cpu.SP.wrapping_sub(n as u16);

        } else {

            self.last_instruction = "ADD SP,";

            if (((cpu.SP & 0x0fff) + (n as u16 & 0x0fff)) & 0x1000) == 0x1000 {
                cpu.set_flag("H");
            } else {
                cpu.reset_flag("H");
            }

            if cpu.SP as u32 + n as u32 > 65535 {
                cpu.set_flag("C");
            } else {
                cpu.reset_flag("C");
            }

            cpu.SP = cpu.SP.wrapping_add(n as u16);
        }

        self.rhs = n as u16;
        self.operand_mode = 1;
        16
    }


    fn call_nn_cd(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);
        cpu.SP -= 2;

        cpu.PC = Opcode::byte_cat(h,l);

        self.rhs = Opcode::byte_cat(h,l);
        self.last_instruction = "CALL,";
        self.operand_mode = 1;
        12
    }


    fn jr_cc_n_20(&mut self, cpu : &mut CPU) -> u8 {
        let n : i8 = self.fetch(cpu) as i8;

        if cpu.get_flag("Z") == 0 && n <= 0 {
            cpu.PC = cpu.PC.wrapping_sub((-n as u16));
            self.rhs = (-n) as u16;
            self.last_instruction = "JR NZ, -";
            self.operand_mode = 1;
        } else

        if cpu.get_flag("Z") == 0 && n > 0 {
            cpu.PC = cpu.PC.wrapping_add(n as u16);
            self.rhs = n as u16;
            self.last_instruction = "JR NZ,";
            self.operand_mode = 1;
        } else {
            self.last_instruction = "JR NZ";
            self.operand_mode = 0;
        }

        8
    }


    fn jr_cc_n_28(&mut self, cpu : &mut CPU) -> u8 {
        let n : i8 = self.fetch(cpu) as i8;

        if cpu.get_flag("Z") == 1 && n <= 0 {
            cpu.PC = cpu.PC.wrapping_sub((-n as u16));
            self.rhs = (-n) as u16;
            self.last_instruction = "JR Z, -";
            self.operand_mode = 1;
        } else

        if cpu.get_flag("Z") == 1 && n > 0 {
            cpu.PC = cpu.PC.wrapping_add(n as u16);
            self.rhs = n as u16;
            self.last_instruction = "JR Z,";
            self.operand_mode = 1;
        } else {
            self.last_instruction = "JR Z";
            self.operand_mode = 0;
        }

        8
    }


    fn jr_cc_n_30(&mut self, cpu : &mut CPU) -> u8 {
        let n : i8 = self.fetch(cpu) as i8;

        if cpu.get_flag("C") == 0 && n <= 0 {
            cpu.PC = cpu.PC.wrapping_sub((-n) as u16);
            self.rhs = (-n) as u16;
            self.last_instruction = "JR CZ, -";
            self.operand_mode = 1;
        } else

        if cpu.get_flag("C") == 0 && n > 0 {
            cpu.PC = cpu.PC.wrapping_add(n as u16);
            self.rhs = n as u16;
            self.last_instruction = "JR CZ,";
            self.operand_mode = 1;
        } else {
            self.last_instruction = "JR CZ";
            self.operand_mode = 0;
        }

        8
    }


    fn jr_cc_n_38(&mut self, cpu : &mut CPU) -> u8 {
        let n : i8 = self.fetch(cpu) as i8;

        if cpu.get_flag("C") == 1 && n <= 0 {
            cpu.PC = cpu.PC.wrapping_sub((-n) as u16);
            self.rhs = (-n) as u16;
            self.last_instruction = "JR C, -";
            self.operand_mode = 1;
        } else

        if cpu.get_flag("C") == 1 && n > 0 {
            cpu.PC = cpu.PC.wrapping_add(n as u16);
            self.rhs = n as u16;
            self.last_instruction = "JR C,";
            self.operand_mode = 1;
        } else {
            self.last_instruction = "JR C";
            self.operand_mode = 0;
        }

        8
    }


    fn jr_n_18(&mut self, cpu : &mut CPU) -> u8 {
        let n : i8 = self.fetch(cpu) as i8;

        if n <= 0 {
            cpu.PC = cpu.PC - (-n) as u16;
            self.rhs = (-n) as u16;
            self.last_instruction = "JR, -";
        }

        if n > 0 {
            cpu.PC = cpu.PC + (n as u16);
            self.rhs = n as u16;
            self.last_instruction = "JR,";
        }

        self.operand_mode = 1;
        8
    }


    fn jp_hl_e9(&mut self, cpu : &mut CPU) -> u8 {
        cpu.PC = Opcode::byte_cat(cpu.H, cpu.L);

        self.last_instruction = "JP (HL)";
        self.operand_mode = 0;
        4
    }


    fn jp_cc_nn_c2(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        if cpu.get_flag("Z") == 0 {
            cpu.PC = Opcode::byte_cat(h, l);
        }

        self.rhs = Opcode::byte_cat(h, l);
        self.last_instruction = "JP NZ,";
        self.operand_mode = 1;
        12
    }


    fn jp_cc_nn_ca(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        if cpu.get_flag("Z") == 1 {
            cpu.PC = Opcode::byte_cat(h, l);
        }

        self.rhs = Opcode::byte_cat(h, l);
        self.last_instruction = "JP Z,";
        self.operand_mode = 1;
        12
    }


    fn jp_cc_nn_d2(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        if cpu.get_flag("C") == 0 {
            cpu.PC = Opcode::byte_cat(h, l);
        }

        self.rhs = Opcode::byte_cat(h, l);
        self.last_instruction = "JP NC,";
        self.operand_mode = 1;
        12
    }


    fn jp_cc_nn_da(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        if cpu.get_flag("C") == 1 {
            cpu.PC = Opcode::byte_cat(h, l);
        }

        self.rhs = Opcode::byte_cat(h, l);
        self.last_instruction = "JP C,";
        self.operand_mode = 1;
        12
    }


    fn jp_nn_c3(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        cpu.PC = Opcode::byte_cat(h, l);
        self.rhs = Opcode::byte_cat(h, l);
        self.last_instruction = "JP ,";
        self.operand_mode = 1;
        12
    }


    fn call_cc_nn_c4(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        if cpu.get_flag("Z") == 0 {

            let pc_h : u8 = ((cpu.PC) >> 8) as u8;
            let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

            cpu.STACK.push_front(pc_h);
            cpu.STACK.push_front(pc_l);
            cpu.SP -= 2;
            cpu.PC = Opcode::byte_cat(h,l);
        }

        self.rhs = Opcode::byte_cat(h,l);
        self.last_instruction = "CALL NZ,";
        self.operand_mode = 1;

        12
    }


    fn call_cc_nn_cc(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        if cpu.get_flag("Z") == 1 {

            let pc_h : u8 = ((cpu.PC) >> 8) as u8;
            let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

            cpu.STACK.push_front(pc_h);
            cpu.STACK.push_front(pc_l);
            cpu.SP -= 2;
            cpu.PC = Opcode::byte_cat(h,l);
        }

        self.rhs = Opcode::byte_cat(h,l);
        self.last_instruction = "CALL Z,";
        self.operand_mode = 1;

        12
    }


    fn call_cc_nn_d4(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        if cpu.get_flag("C") == 0 {

            let pc_h : u8 = ((cpu.PC) >> 8) as u8;
            let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

            cpu.STACK.push_front(pc_h);
            cpu.STACK.push_front(pc_l);
            cpu.SP -= 2;
            cpu.PC = Opcode::byte_cat(h,l);
        }

        self.rhs = Opcode::byte_cat(h,l);
        self.last_instruction = "CALL NC,";
        self.operand_mode = 1;

        12
    }


    fn call_cc_nn_dc(&mut self, cpu : &mut CPU) -> u8 {
        let l : u8 = self.fetch(cpu);
        let h : u8 = self.fetch(cpu);

        if cpu.get_flag("C") == 1 {

            let pc_h : u8 = ((cpu.PC) >> 8) as u8;
            let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

            cpu.STACK.push_front(pc_h);
            cpu.STACK.push_front(pc_l);
            cpu.SP -= 2;
            cpu.PC = Opcode::byte_cat(h,l);
        }

        self.rhs = Opcode::byte_cat(h,l);
        self.last_instruction = "CALL C,";
        self.operand_mode = 1;

        12
    }


    fn rst_c7(&mut self, cpu : &mut CPU) -> u8 {

        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);
        cpu.SP -= 2;

        cpu.PC = 0x0000;

        self.last_instruction = "RST 00H";
        self.operand_mode = 0;
        32
    }


    fn rst_cf(&mut self, cpu : &mut CPU) -> u8 {

        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);
        cpu.SP -= 2;

        cpu.PC = 0x0008;

        self.last_instruction = "RST 08H";
        self.operand_mode = 0;
        32
    }


    fn rst_d7(&mut self, cpu : &mut CPU) -> u8 {

        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);
        cpu.SP -= 2;

        cpu.PC = 0x0010;

        self.last_instruction = "RST 10H";
        self.operand_mode = 0;
        32
    }


    fn rst_df(&mut self, cpu : &mut CPU) -> u8 {

        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);
        cpu.SP -= 2;

        cpu.PC = 0x0018;

        self.last_instruction = "RST 18H";
        self.operand_mode = 0;
        32
    }


    fn rst_e7(&mut self, cpu : &mut CPU) -> u8 {

        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);
        cpu.SP -= 2;

        cpu.PC = 0x0020;

        self.last_instruction = "RST 20H";
        self.operand_mode = 0;
        32
    }


    fn rst_ef(&mut self, cpu : &mut CPU) -> u8 {

        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);
        cpu.SP -= 2;

        cpu.PC = 0x0028;


        self.last_instruction = "RST 28H";
        self.operand_mode = 0;
        32
    }


    fn rst_f7(&mut self, cpu : &mut CPU) -> u8 {

        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);
        cpu.SP -= 2;

        cpu.PC = 0x0030;

        self.last_instruction = "RST 30H";
        self.operand_mode = 0;
        32
    }


    fn rst_ff(&mut self, cpu : &mut CPU) -> u8 {

        let pc_h : u8 = ((cpu.PC) >> 8) as u8;
        let pc_l : u8 = ((cpu.PC) & 0x00FF) as u8;

        cpu.STACK.push_front(pc_h);
        cpu.STACK.push_front(pc_l);
        cpu.SP = cpu.SP.wrapping_sub(2);

        cpu.PC = 0x0038;

        self.last_instruction = "RST 38H";
        self.operand_mode = 0;
        32
    }


    fn ret_c9(&mut self, cpu : &mut CPU) -> u8 {

        let l : u8 = cpu.STACK.pop_front().unwrap();
        let h : u8 = cpu.STACK.pop_front().unwrap();
        cpu.SP += 2;
        cpu.PC = Opcode::byte_cat(h, l);

        self.last_instruction = "RET";
        self.operand_mode = 0;
        8
    }


    fn reti_d9(&mut self, cpu : &mut CPU) -> u8 {

        let l : u8 = cpu.STACK.pop_front().unwrap();
        let h : u8 = cpu.STACK.pop_front().unwrap();
        cpu.SP += 2;
        cpu.PC = Opcode::byte_cat(h, l);

        cpu.RAM[0xFFFF as usize] = 0b00011111; // enable interrupts

        self.last_instruction = "RETI";
        self.operand_mode = 0;
        8
    }


    fn ret_cc_c0(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.get_flag("Z") == 0 {
            let l : u8 = cpu.STACK.pop_front().unwrap();
            let h : u8 = cpu.STACK.pop_front().unwrap();
            cpu.SP += 2;
            cpu.PC = Opcode::byte_cat(h, l);
        }

        self.last_instruction = "RET NZ";
        self.operand_mode = 0;
        8
    }


    fn ret_cc_c8(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.get_flag("Z") == 1 {
            let l : u8 = cpu.STACK.pop_front().unwrap();
            let h : u8 = cpu.STACK.pop_front().unwrap();
            cpu.SP += 2;
            cpu.PC = Opcode::byte_cat(h, l);
        }

        self.last_instruction = "RET Z";
        self.operand_mode = 0;
        8
    }


    fn ret_cc_d0(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.get_flag("C") == 0 {
            let l : u8 = cpu.STACK.pop_front().unwrap();
            let h : u8 = cpu.STACK.pop_front().unwrap();
            cpu.SP += 2;
            cpu.PC = Opcode::byte_cat(h, l);
        }

        self.last_instruction = "RET NC";
        self.operand_mode = 0;
        8
    }


    fn ret_cc_d8(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.get_flag("C") == 1 {
            let l : u8 = cpu.STACK.pop_front().unwrap();
            let h : u8 = cpu.STACK.pop_front().unwrap();
            cpu.SP += 2;
            cpu.PC = Opcode::byte_cat(h, l);
        }

        self.last_instruction = "RET C";
        self.operand_mode = 0;
        8
    }

    fn halt_76(&mut self, cpu : &mut CPU) -> u8 {

        self.last_instruction = "HALT";
        self.operand_mode = 0;
        4
    }


    fn ccf_3f(&mut self, cpu : &mut CPU) -> u8 {

        if cpu.get_flag("C") == 1 {
            cpu.reset_flag("C");
        } else {
            cpu.set_flag("C");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CCF";
        self.operand_mode = 0;
        4
    }


    fn scf_37(&mut self, cpu : &mut CPU) -> u8 {

        cpu.set_flag("C");
        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "SCF";
        self.operand_mode = 0;
        4
    }


    fn di_f3(&mut self, cpu : &mut CPU) -> u8 {

        self.disable_int = true;

        self.last_instruction = "DI";
        self.operand_mode = 0;
        4
    }


    fn ei_fb(&mut self, cpu : &mut CPU) -> u8 {

        self.enable_int = true;

        self.last_instruction = "EI";
        self.operand_mode = 0;
        4
    }


    fn sub_a_97(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.A;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b);

        self.last_instruction = "SUB A, A";
        self.operand_mode = 0;
        4
    }


    fn sub_a_90(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.B;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b);

        self.last_instruction = "SUB A, B";
        self.operand_mode = 0;
        4
    }


    fn sub_a_91(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.C;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b);

        self.last_instruction = "SUB A, C";
        self.operand_mode = 0;
        4
    }


    fn sub_a_92(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.D;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b);

        self.last_instruction = "SUB A, D";
        self.operand_mode = 0;
        4
    }


    fn sub_a_93(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.E;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b);

        self.last_instruction = "SUB A, E";
        self.operand_mode = 0;
        4
    }


    fn sub_a_94(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.H;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b);

        self.last_instruction = "SUB A, H";
        self.operand_mode = 0;
        4
    }


    fn sub_a_95(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.L;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b);

        self.last_instruction = "SUB A, L";
        self.operand_mode = 0;
        4
    }


    fn sub_a_96(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b);

        self.last_instruction = "SUB A, (HL)";
        self.operand_mode = 0;
        8
    }


    fn sub_a_d6(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = self.fetch(cpu);

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b);

        self.rhs = b as u16;
        self.last_instruction = "SUB A,";
        self.operand_mode = 1;
        8
    }


    fn sbc_an_9f(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.A;
        let c : u8 = cpu.get_flag("C");

        if a == b.wrapping_add(c) {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) + c {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b.wrapping_add(c) {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b.wrapping_add(c));

        self.last_instruction = "SBC A, A";
        self.operand_mode = 0;
        4
    }


    fn sbc_an_98(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.B;
        let c : u8 = cpu.get_flag("C");

        if a == (b + c) {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) + c {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < (b + c) {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b + c);

        self.last_instruction = "SBC A, B";
        self.operand_mode = 0;
        4
    }


    fn sbc_an_99(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.C;
        let c : u8 = cpu.get_flag("C");

        if a == (b + c) {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) + c {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < (b + c) {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b + c);

        self.last_instruction = "SBC A, C";
        self.operand_mode = 0;
        4
    }


    fn sbc_an_9a(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.D;
        let c : u8 = cpu.get_flag("C");

        if a == (b + c) {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) + c {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < (b + c) {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b + c);

        self.last_instruction = "SBC A, D";
        self.operand_mode = 0;
        4
    }


    fn sbc_an_9b(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.E;
        let c : u8 = cpu.get_flag("C");

        if a == (b + c) {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) + c {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < (b + c) {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b + c);

        self.last_instruction = "SBC A, E";
        self.operand_mode = 0;
        4
    }


    fn sbc_an_9c(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.H;
        let c : u8 = cpu.get_flag("C");

        if a == (b + c) {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) + c {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < (b + c) {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b + c);

        self.last_instruction = "SBC A, H";
        self.operand_mode = 0;
        4
    }


    fn sbc_an_9d(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.L;
        let c : u8 = cpu.get_flag("C");

        if a == (b + c) {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) + c {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < (b + c) {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b + c);

        self.last_instruction = "SBC A, L";
        self.operand_mode = 0;
        4
    }


    fn sbc_an_9e(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];
        let c : u8 = cpu.get_flag("C");

        if a == (b + c) {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) + c {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < (b + c) {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b + c);

        self.last_instruction = "SBC A, (HL)";
        self.operand_mode = 0;
        8
    }


    fn sbc_an_de(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = self.fetch(cpu);
        let c : u8 = cpu.get_flag("C");

        if a == (b + c) {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) + c {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < (b + c) {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = a.wrapping_sub(b + c);

        self.rhs = b as u16;
        self.last_instruction = "SBC A,";
        self.operand_mode = 1;
        8
    }


    fn daa_27(&mut self, cpu : &mut CPU) -> u8 {
        let mut adjust : u8 = if cpu.get_flag("C") == 1 { 0x60 } else { 0x00 };

        if cpu.get_flag("H") == 1 {
            adjust |= 0x06;
        }

        if cpu.get_flag("N") == 0 {
            if cpu.A & 0x0F > 0x09 { adjust |= 0x06; }
            if cpu.A > 0x99 { adjust |= 0x60; }
            cpu.A = cpu.A.wrapping_add(adjust);
        } else {
            cpu.A = cpu.A.wrapping_sub(adjust);
        }

        if adjust >= 0x60 { cpu.set_flag("C"); }
        else { cpu.reset_flag("C"); }

        cpu.reset_flag("H");

        if cpu.A == 0 { cpu.set_flag("Z"); }
        else { cpu.reset_flag("Z"); }

        self.last_instruction = "DAA";
        self.operand_mode = 0;
        4
    }


    fn cp_n_bf(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.A;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        self.last_instruction = "CP A, A";
        self.operand_mode = 0;
        4
    }


    fn cp_n_b8(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.B;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        self.last_instruction = "CP A, B";
        self.operand_mode = 0;
        4
    }


    fn cp_n_b9(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.C;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        self.last_instruction = "CP A, C";
        self.operand_mode = 0;
        4
    }


    fn cp_n_ba(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.D;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        self.last_instruction = "CP A, D";
        self.operand_mode = 0;
        4
    }


    fn cp_n_bb(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.E;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        self.last_instruction = "CP A, E";
        self.operand_mode = 0;
        4
    }


    fn cp_n_bc(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.H;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        self.last_instruction = "CP A, H";
        self.operand_mode = 0;
        4
    }


    fn cp_n_bd(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.L;

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        self.last_instruction = "CP A, L";
        self.operand_mode = 0;
        4
    }


    fn cp_n_be(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        self.last_instruction = "CP A, (HL)";
        self.operand_mode = 0;
        8
    }


    fn cp_n_fe(&mut self, cpu : &mut CPU) -> u8 {

        let a : u8 = cpu.A;
        let b : u8 = self.fetch(cpu);

        if a == b {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.set_flag("N");

        if (a & 0x0F) < (b & 0x0F) {
            cpu.set_flag("H");
        } else {
            cpu.reset_flag("H");
        }

        if a < b {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        self.rhs = b as u16;
        self.last_instruction = "CP A,";
        self.operand_mode = 1;
        8
    }


    fn rlca_07(&mut self, cpu : &mut CPU) -> u8 {

        let bit_7 = cpu.A & 0x80 == 0x80;

        cpu.A = cpu.A << 1 | ( if bit_7 { 1 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "RLCA";
        self.operand_mode = 0;
        4
    }


    fn rla_17(&mut self, cpu : &mut CPU) -> u8 {

        let bit_7 = cpu.A & 0x80 == 0x80;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.A = cpu.A << 1 | ( if cpu.get_flag("C") == 1 { 1 } else { 0 });

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "RLA";
        self.operand_mode = 0;
        4
    }


    fn rrca_0f(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.A & 0x01 == 0x01;

        cpu.A = cpu.A >> 1 | ( if bit_0 { 0x80 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "RRCA";
        self.operand_mode = 0;
        4
    }


    fn rra_1f(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.A & 0x01 == 0x01;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.A = cpu.A >> 1 | ( if cpu.get_flag("C") == 1 { 0x80 } else { 0 });

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "RRA";
        self.operand_mode = 0;
        4
    }


    fn cpl_2f(&mut self, cpu : &mut CPU) -> u8 {

        cpu.set_flag("N");
        cpu.set_flag("H");
        cpu.A = !cpu.A;

        self.last_instruction = "CPL";
        self.operand_mode = 0;
        4
    }


























































    // CB OPCODES


    fn cb_bit_40(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(0, cpu.B) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 0, B";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_50(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(2, cpu.B) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 2, B";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_60(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(4, cpu.B) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 4, B";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_70(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(6, cpu.B) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 6, B";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_41(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(0, cpu.C) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 0, C";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_51(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(2, cpu.C) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 2, C";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_61(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(4, cpu.C) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 4, C";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_71(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(6, cpu.C) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 6, C";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_42(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(0, cpu.D) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 0, D";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_52(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(2, cpu.D) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 2, D";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_62(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(4, cpu.D) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 4, D";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_72(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(6, cpu.D) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 6, D";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_43(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(0, cpu.E) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 0, E";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_53(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(2, cpu.E) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 2, E";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_63(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(4, cpu.E) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 4, E";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_73(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(6, cpu.E) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 6, E";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_44(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(0, cpu.H) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 0, H";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_54(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(2, cpu.H) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 2, H";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_64(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(4, cpu.H) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 4, H";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_74(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(6, cpu.H) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 6, H";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_45(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(0, cpu.L) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 0, L";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_55(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(2, cpu.L) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 2, L";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_65(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(4, cpu.L) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 4, L";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_75(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(6, cpu.L) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 6, L";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_46(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(0,
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize]) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 0, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_bit_56(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(2,
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize]) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 2, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_bit_66(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(4,
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize]) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 4, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_bit_76(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(6,
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize]) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 6, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_bit_47(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(0, cpu.A) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 0, A";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_57(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(2, cpu.A) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 2, A";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_67(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(4, cpu.A) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 4, A";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_77(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(6, cpu.A) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 6, A";
        self.operand_mode = 0;
        8
    }


fn cb_bit_48(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(1, cpu.B) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 1, B";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_58(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(3, cpu.B) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 3, B";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_68(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(5, cpu.B) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 5, B";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_78(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(7, cpu.B) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 7, B";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_49(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(1, cpu.C) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 1, C";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_59(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(3, cpu.C) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 3, C";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_69(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(5, cpu.C) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 5, C";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_79(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(7, cpu.C) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 7, C";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_4a(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(1, cpu.D) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 1, D";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_5a(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(3, cpu.D) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 3, D";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_6a(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(5, cpu.D) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 5, D";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_7a(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(7, cpu.D) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 7, D";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_4b(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(1, cpu.E) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 1, E";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_5b(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(3, cpu.E) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 3, E";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_6b(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(5, cpu.E) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 5, E";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_7b(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(7, cpu.E) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 7, E";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_4c(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(1, cpu.H) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 1, H";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_5c(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(3, cpu.H) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 3, H";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_6c(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(5, cpu.H) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 5, H";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_7c(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(7, cpu.H) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 7, H";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_4d(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(1, cpu.L) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 1, L";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_5d(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(3, cpu.L) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 3, L";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_6d(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(5, cpu.L) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 5, L";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_7d(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(7, cpu.L) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 7, L";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_4e(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(1,
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize]) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 1, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_bit_5e(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(3,
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize]) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 3, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_bit_6e(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(5,
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize]) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 5, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_bit_7e(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(7,
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize]) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 7, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_bit_4f(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(1, cpu.A) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 1, A";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_5f(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(3, cpu.A) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 3, A";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_6f(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(5, cpu.A) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 5, A";
        self.operand_mode = 0;
        8
    }


    fn cb_bit_7f(&mut self, cpu : &mut CPU) -> u8 {

        if Opcode::get_bit(7, cpu.A) == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.set_flag("H");

        self.last_instruction = "CB - BIT 7, A";
        self.operand_mode = 0;
        8
    }





    fn cb_res_80(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::reset_bit(0, cpu.B);

        self.last_instruction = "CB - RES 0, B";
        self.operand_mode = 0;
        8
    }


    fn cb_res_90(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::reset_bit(2, cpu.B);

        self.last_instruction = "CB - RES 2, B";
        self.operand_mode = 0;
        8
    }


    fn cb_res_a0(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::reset_bit(4, cpu.B);

        self.last_instruction = "CB - RES 4, B";
        self.operand_mode = 0;
        8
    }


    fn cb_res_b0(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::reset_bit(6, cpu.B);

        self.last_instruction = "CB - RES 6, B";
        self.operand_mode = 0;
        8
    }


    fn cb_res_81(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::reset_bit(0, cpu.C);

        self.last_instruction = "CB - RES 0, C";
        self.operand_mode = 0;
        8
    }


    fn cb_res_91(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::reset_bit(2, cpu.C);

        self.last_instruction = "CB - RES 2, C";
        self.operand_mode = 0;
        8
    }


    fn cb_res_a1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::reset_bit(4, cpu.C);

        self.last_instruction = "CB - RES 4, C";
        self.operand_mode = 0;
        8
    }


    fn cb_res_b1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::reset_bit(6, cpu.C);

        self.last_instruction = "CB - RES 6, C";
        self.operand_mode = 0;
        8
    }


    fn cb_res_82(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::reset_bit(0, cpu.D);

        self.last_instruction = "CB - RES 0, D";
        self.operand_mode = 0;
        8
    }


    fn cb_res_92(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::reset_bit(2, cpu.D);

        self.last_instruction = "CB - RES 2, D";
        self.operand_mode = 0;
        8
    }


    fn cb_res_a2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::reset_bit(4, cpu.D);

        self.last_instruction = "CB - RES 4, D";
        self.operand_mode = 0;
        8
    }


    fn cb_res_b2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::reset_bit(6, cpu.D);

        self.last_instruction = "CB - RES 6, D";
        self.operand_mode = 0;
        8
    }


    fn cb_res_83(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::reset_bit(0, cpu.E);

        self.last_instruction = "CB - RES 0, E";
        self.operand_mode = 0;
        8
    }


    fn cb_res_93(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::reset_bit(2, cpu.E);

        self.last_instruction = "CB - RES 2, E";
        self.operand_mode = 0;
        8
    }


    fn cb_res_a3(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::reset_bit(4, cpu.E);

        self.last_instruction = "CB - RES 4, E";
        self.operand_mode = 0;
        8
    }


    fn cb_res_b3(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::reset_bit(6, cpu.E);

        self.last_instruction = "CB - RES 6, E";
        self.operand_mode = 0;
        8
    }


    fn cb_res_84(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::reset_bit(0, cpu.H);

        self.last_instruction = "CB - RES 0, H";
        self.operand_mode = 0;
        8
    }


    fn cb_res_94(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::reset_bit(2, cpu.H);

        self.last_instruction = "CB - RES 2, H";
        self.operand_mode = 0;
        8
    }


    fn cb_res_a4(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::reset_bit(4, cpu.H);

        self.last_instruction = "CB - RES 4, H";
        self.operand_mode = 0;
        8
    }


    fn cb_res_b4(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::reset_bit(6, cpu.H);

        self.last_instruction = "CB - RES 6, H";
        self.operand_mode = 0;
        8
    }


    fn cb_res_85(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::reset_bit(0, cpu.L);

        self.last_instruction = "CB - RES 0, L";
        self.operand_mode = 0;
        8
    }


    fn cb_res_95(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::reset_bit(2, cpu.L);

        self.last_instruction = "CB - RES 2, L";
        self.operand_mode = 0;
        8
    }


    fn cb_res_a5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::reset_bit(4, cpu.L);

        self.last_instruction = "CB - RES 4, L";
        self.operand_mode = 0;
        8
    }


    fn cb_res_b5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::reset_bit(6, cpu.L);

        self.last_instruction = "CB - RES 6, L";
        self.operand_mode = 0;
        8
    }


    fn cb_res_86(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let reset = Opcode::reset_bit(0,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, reset);

        self.last_instruction = "CB - RES 0, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_res_96(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let reset = Opcode::reset_bit(2,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, reset);

        self.last_instruction = "CB - RES 2, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_res_a6(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let reset = Opcode::reset_bit(4,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, reset);

        self.last_instruction = "CB - RES 4, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_res_b6(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let reset = Opcode::reset_bit(6,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, reset);

        self.last_instruction = "CB - RES 6, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_res_87(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::reset_bit(0, cpu.A);

        self.last_instruction = "CB - RES 0, A";
        self.operand_mode = 0;
        8
    }


    fn cb_res_97(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::reset_bit(2, cpu.A);

        self.last_instruction = "CB - RES 2, A";
        self.operand_mode = 0;
        8
    }


    fn cb_res_a7(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::reset_bit(4, cpu.A);

        self.last_instruction = "CB - RES 4, A";
        self.operand_mode = 0;
        8
    }


    fn cb_res_b7(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::reset_bit(6, cpu.A);

        self.last_instruction = "CB - RES 6, A";
        self.operand_mode = 0;
        8
    }


    fn cb_res_88(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::reset_bit(1, cpu.B);

        self.last_instruction = "CB - RES 1, B";
        self.operand_mode = 0;
        8
    }


    fn cb_res_98(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::reset_bit(3, cpu.B);

        self.last_instruction = "CB - RES 3, B";
        self.operand_mode = 0;
        8
    }


    fn cb_res_a8(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::reset_bit(5, cpu.B);

        self.last_instruction = "CB - RES 5, B";
        self.operand_mode = 0;
        8
    }


    fn cb_res_b8(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::reset_bit(7, cpu.B);

        self.last_instruction = "CB - RES 7, B";
        self.operand_mode = 0;
        8
    }


    fn cb_res_89(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::reset_bit(1, cpu.C);

        self.last_instruction = "CB - RES 1, C";
        self.operand_mode = 0;
        8
    }


    fn cb_res_99(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::reset_bit(3, cpu.C);

        self.last_instruction = "CB - RES 3, C";
        self.operand_mode = 0;
        8
    }


    fn cb_res_a9(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::reset_bit(5, cpu.C);

        self.last_instruction = "CB - RES 5, C";
        self.operand_mode = 0;
        8
    }


    fn cb_res_b9(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::reset_bit(7, cpu.C);

        self.last_instruction = "CB - RES 7, C";
        self.operand_mode = 0;
        8
    }


    fn cb_res_8a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::reset_bit(1, cpu.D);

        self.last_instruction = "CB - RES 1, D";
        self.operand_mode = 0;
        8
    }


    fn cb_res_9a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::reset_bit(3, cpu.D);

        self.last_instruction = "CB - RES 3, D";
        self.operand_mode = 0;
        8
    }


    fn cb_res_aa(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::reset_bit(5, cpu.D);

        self.last_instruction = "CB - RES 5, D";
        self.operand_mode = 0;
        8
    }


    fn cb_res_ba(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::reset_bit(7, cpu.D);

        self.last_instruction = "CB - RES 7, D";
        self.operand_mode = 0;
        8
    }


    fn cb_res_8b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::reset_bit(1, cpu.E);

        self.last_instruction = "CB - RES 1, E";
        self.operand_mode = 0;
        8
    }


    fn cb_res_9b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::reset_bit(3, cpu.E);

        self.last_instruction = "CB - RES 3, E";
        self.operand_mode = 0;
        8
    }


    fn cb_res_ab(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::reset_bit(5, cpu.E);

        self.last_instruction = "CB - RES 5, E";
        self.operand_mode = 0;
        8
    }


    fn cb_res_bb(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::reset_bit(7, cpu.E);

        self.last_instruction = "CB - RES 7, E";
        self.operand_mode = 0;
        8
    }


    fn cb_res_8c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::reset_bit(1, cpu.H);

        self.last_instruction = "CB - RES 1, H";
        self.operand_mode = 0;
        8
    }


    fn cb_res_9c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::reset_bit(3, cpu.H);

        self.last_instruction = "CB - RES 3, H";
        self.operand_mode = 0;
        8
    }


    fn cb_res_ac(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::reset_bit(5, cpu.H);

        self.last_instruction = "CB - RES 5, H";
        self.operand_mode = 0;
        8
    }


    fn cb_res_bc(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::reset_bit(7, cpu.H);

        self.last_instruction = "CB - RES 7, H";
        self.operand_mode = 0;
        8
    }


    fn cb_res_8d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::reset_bit(1, cpu.L);

        self.last_instruction = "CB - RES 1, L";
        self.operand_mode = 0;
        8
    }


    fn cb_res_9d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::reset_bit(3, cpu.L);

        self.last_instruction = "CB - RES 3, L";
        self.operand_mode = 0;
        8
    }


    fn cb_res_ad(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::reset_bit(5, cpu.L);

        self.last_instruction = "CB - RES 5, L";
        self.operand_mode = 0;
        8
    }


    fn cb_res_bd(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::reset_bit(7, cpu.L);

        self.last_instruction = "CB - RES 7, L";
        self.operand_mode = 0;
        8
    }


    fn cb_res_8e(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let reset = Opcode::reset_bit(1,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, reset);

        self.last_instruction = "CB - RES 1, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_res_9e(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let reset = Opcode::reset_bit(3,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, reset);

        self.last_instruction = "CB - RES 3, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_res_ae(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let reset = Opcode::reset_bit(5,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, reset);

        self.last_instruction = "CB - RES 5, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_res_be(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let reset = Opcode::reset_bit(7,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, reset);

        self.last_instruction = "CB - RES 7, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_res_8f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::reset_bit(1, cpu.A);

        self.last_instruction = "CB - RES 1, A";
        self.operand_mode = 0;
        8
    }


    fn cb_res_9f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::reset_bit(3, cpu.A);

        self.last_instruction = "CB - RES 3, A";
        self.operand_mode = 0;
        8
    }


    fn cb_res_af(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::reset_bit(5, cpu.A);

        self.last_instruction = "CB - RES 5, A";
        self.operand_mode = 0;
        8
    }


    fn cb_res_bf(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::reset_bit(7, cpu.A);

        self.last_instruction = "CB - RES 7, A";
        self.operand_mode = 0;
        8
    }


    fn cb_set_c0(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::set_bit(0, cpu.B);

        self.last_instruction = "CB - SET 0, B";
        self.operand_mode = 0;
        8
    }


    fn cb_set_d0(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::set_bit(2, cpu.B);

        self.last_instruction = "CB - SET 2, B";
        self.operand_mode = 0;
        8
    }


    fn cb_set_e0(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::set_bit(4, cpu.B);

        self.last_instruction = "CB - SET 4, B";
        self.operand_mode = 0;
        8
    }


    fn cb_set_f0(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::set_bit(6, cpu.B);

        self.last_instruction = "CB - SET 6, B";
        self.operand_mode = 0;
        8
    }


    fn cb_set_c1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::set_bit(0, cpu.C);

        self.last_instruction = "CB - SET 0, C";
        self.operand_mode = 0;
        8
    }


    fn cb_set_d1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::set_bit(2, cpu.C);

        self.last_instruction = "CB - SET 2, C";
        self.operand_mode = 0;
        8
    }


    fn cb_set_e1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::set_bit(4, cpu.C);

        self.last_instruction = "CB - SET 4, C";
        self.operand_mode = 0;
        8
    }


    fn cb_set_f1(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::set_bit(6, cpu.C);

        self.last_instruction = "CB - SET 6, C";
        self.operand_mode = 0;
        8
    }


    fn cb_set_c2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::set_bit(0, cpu.D);

        self.last_instruction = "CB - SET 0, D";
        self.operand_mode = 0;
        8
    }


    fn cb_set_d2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::set_bit(2, cpu.D);

        self.last_instruction = "CB - SET 2, D";
        self.operand_mode = 0;
        8
    }


    fn cb_set_e2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::set_bit(4, cpu.D);

        self.last_instruction = "CB - SET 4, D";
        self.operand_mode = 0;
        8
    }


    fn cb_set_f2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::set_bit(6, cpu.D);

        self.last_instruction = "CB - SET 6, D";
        self.operand_mode = 0;
        8
    }


    fn cb_set_c3(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::set_bit(0, cpu.E);

        self.last_instruction = "CB - SET 0, E";
        self.operand_mode = 0;
        8
    }


    fn cb_set_d3(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::set_bit(2, cpu.E);

        self.last_instruction = "CB - SET 2, E";
        self.operand_mode = 0;
        8
    }


    fn cb_set_e3(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::set_bit(4, cpu.E);

        self.last_instruction = "CB - SET 4, E";
        self.operand_mode = 0;
        8
    }


    fn cb_set_f3(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::set_bit(6, cpu.E);

        self.last_instruction = "CB - SET 6, E";
        self.operand_mode = 0;
        8
    }


    fn cb_set_c4(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::set_bit(0, cpu.H);

        self.last_instruction = "CB - SET 0, H";
        self.operand_mode = 0;
        8
    }


    fn cb_set_d4(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::set_bit(2, cpu.H);

        self.last_instruction = "CB - SET 2, H";
        self.operand_mode = 0;
        8
    }


    fn cb_set_e4(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::set_bit(4, cpu.H);

        self.last_instruction = "CB - SET 4, H";
        self.operand_mode = 0;
        8
    }


    fn cb_set_f4(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::set_bit(6, cpu.H);

        self.last_instruction = "CB - SET 6, H";
        self.operand_mode = 0;
        8
    }


    fn cb_set_c5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::set_bit(0, cpu.L);

        self.last_instruction = "CB - SET 0, L";
        self.operand_mode = 0;
        8
    }


    fn cb_set_d5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::set_bit(2, cpu.L);

        self.last_instruction = "CB - SET 2, L";
        self.operand_mode = 0;
        8
    }


    fn cb_set_e5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::set_bit(4, cpu.L);

        self.last_instruction = "CB - SET 4, L";
        self.operand_mode = 0;
        8
    }


    fn cb_set_f5(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::set_bit(6, cpu.L);

        self.last_instruction = "CB - SET 6, L";
        self.operand_mode = 0;
        8
    }


    fn cb_set_c6(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let set = Opcode::set_bit(0,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, set);

        self.last_instruction = "CB - SET 0, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_set_d6(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let set = Opcode::set_bit(2,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, set);

        self.last_instruction = "CB - SET 2, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_set_e6(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let set = Opcode::set_bit(4,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, set);

        self.last_instruction = "CB - SET 4, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_set_f6(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let set = Opcode::set_bit(6,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, set);

        self.last_instruction = "CB - SET 6, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_set_c7(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::set_bit(0, cpu.A);

        self.last_instruction = "CB - SET 0, A";
        self.operand_mode = 0;
        8
    }


    fn cb_set_d7(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::set_bit(2, cpu.A);

        self.last_instruction = "CB - SET 2, A";
        self.operand_mode = 0;
        8
    }


    fn cb_set_e7(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::set_bit(4, cpu.A);

        self.last_instruction = "CB - SET 4, A";
        self.operand_mode = 0;
        8
    }


    fn cb_set_f7(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::set_bit(6, cpu.A);

        self.last_instruction = "CB - SET 6, A";
        self.operand_mode = 0;
        8
    }


    fn cb_set_c8(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::set_bit(1, cpu.B);

        self.last_instruction = "CB - SET 1, B";
        self.operand_mode = 0;
        8
    }


    fn cb_set_d8(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::set_bit(3, cpu.B);

        self.last_instruction = "CB - SET 3, B";
        self.operand_mode = 0;
        8
    }


    fn cb_set_e8(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::set_bit(5, cpu.B);

        self.last_instruction = "CB - SET 5, B";
        self.operand_mode = 0;
        8
    }


    fn cb_set_f8(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::set_bit(7, cpu.B);

        self.last_instruction = "CB - SET 7, B";
        self.operand_mode = 0;
        8
    }


    fn cb_set_c9(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::set_bit(1, cpu.C);

        self.last_instruction = "CB - SET 1, C";
        self.operand_mode = 0;
        8
    }


    fn cb_set_d9(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::set_bit(3, cpu.C);

        self.last_instruction = "CB - SET 3, C";
        self.operand_mode = 0;
        8
    }


    fn cb_set_e9(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::set_bit(5, cpu.C);

        self.last_instruction = "CB - SET 5, C";
        self.operand_mode = 0;
        8
    }


    fn cb_set_f9(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::set_bit(7, cpu.C);

        self.last_instruction = "CB - SET 7, C";
        self.operand_mode = 0;
        8
    }


    fn cb_set_ca(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::set_bit(1, cpu.D);

        self.last_instruction = "CB - SET 1, D";
        self.operand_mode = 0;
        8
    }


    fn cb_set_da(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::set_bit(3, cpu.D);

        self.last_instruction = "CB - SET 3, D";
        self.operand_mode = 0;
        8
    }


    fn cb_set_ea(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::set_bit(5, cpu.D);

        self.last_instruction = "CB - SET 5, D";
        self.operand_mode = 0;
        8
    }


    fn cb_set_fa(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::set_bit(7, cpu.D);

        self.last_instruction = "CB - SET 7, D";
        self.operand_mode = 0;
        8
    }


    fn cb_set_cb(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::set_bit(1, cpu.E);

        self.last_instruction = "CB - SET 1, E";
        self.operand_mode = 0;
        8
    }


    fn cb_set_db(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::set_bit(3, cpu.E);

        self.last_instruction = "CB - SET 3, E";
        self.operand_mode = 0;
        8
    }


    fn cb_set_eb(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::set_bit(5, cpu.E);

        self.last_instruction = "CB - SET 5, E";
        self.operand_mode = 0;
        8
    }


    fn cb_set_fb(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::set_bit(7, cpu.E);

        self.last_instruction = "CB - SET 7, E";
        self.operand_mode = 0;
        8
    }


    fn cb_set_cc(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::set_bit(1, cpu.H);

        self.last_instruction = "CB - SET 1, H";
        self.operand_mode = 0;
        8
    }


    fn cb_set_dc(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::set_bit(3, cpu.H);

        self.last_instruction = "CB - SET 3, H";
        self.operand_mode = 0;
        8
    }


    fn cb_set_ec(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::set_bit(5, cpu.H);

        self.last_instruction = "CB - SET 5, H";
        self.operand_mode = 0;
        8
    }


    fn cb_set_fc(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::set_bit(7, cpu.H);

        self.last_instruction = "CB - SET 7, H";
        self.operand_mode = 0;
        8
    }


    fn cb_set_cd(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::set_bit(1, cpu.L);

        self.last_instruction = "CB - SET 1, L";
        self.operand_mode = 0;
        8
    }


    fn cb_set_dd(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::set_bit(3, cpu.L);

        self.last_instruction = "CB - SET 3, L";
        self.operand_mode = 0;
        8
    }


    fn cb_set_ed(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::set_bit(5, cpu.L);

        self.last_instruction = "CB - SET 5, L";
        self.operand_mode = 0;
        8
    }


    fn cb_set_fd(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::set_bit(7, cpu.L);

        self.last_instruction = "CB - SET 7, L";
        self.operand_mode = 0;
        8
    }


    fn cb_set_ce(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let set = Opcode::set_bit(1,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, set);

        self.last_instruction = "CB - SET 1, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_set_de(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let set = Opcode::set_bit(3,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, set);

        self.last_instruction = "CB - SET 3, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_set_ee(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let set = Opcode::set_bit(5,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, set);

        self.last_instruction = "CB - SET 5, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_set_fe(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        let set = Opcode::set_bit(7,
                               cpu.RAM[hl as usize]);
        cpu.write_ram(hl, set);

        self.last_instruction = "CB - SET 7, (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_set_cf(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::set_bit(1, cpu.A);

        self.last_instruction = "CB - SET 1, A";
        self.operand_mode = 0;
        8
    }


    fn cb_set_df(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::set_bit(3, cpu.A);

        self.last_instruction = "CB - SET 3, A";
        self.operand_mode = 0;
        8
    }


    fn cb_set_ef(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::set_bit(5, cpu.A);

        self.last_instruction = "CB - SET 5, A";
        self.operand_mode = 0;
        8
    }


    fn cb_set_ff(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::set_bit(7, cpu.A);

        self.last_instruction = "CB - SET 7, A";
        self.operand_mode = 0;
        8
    }


    fn cb_swap_30(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = Opcode::swap(cpu.B);

        if cpu.B == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");
        cpu.reset_flag("C");

        self.last_instruction = "CB - SWAP B";
        self.operand_mode = 0;
        8
    }


    fn cb_swap_31(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = Opcode::swap(cpu.C);

        if cpu.C == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");
        cpu.reset_flag("C");

        self.last_instruction = "CB - SWAP C";
        self.operand_mode = 0;
        8
    }


    fn cb_swap_32(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = Opcode::swap(cpu.D);

        if cpu.D == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");
        cpu.reset_flag("C");

        self.last_instruction = "CB - SWAP D";
        self.operand_mode = 0;
        8
    }


    fn cb_swap_33(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = Opcode::swap(cpu.E);

        if cpu.E == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");
        cpu.reset_flag("C");

        self.last_instruction = "CB - SWAP E";
        self.operand_mode = 0;
        8
    }


    fn cb_swap_34(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = Opcode::swap(cpu.H);

        if cpu.H == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");
        cpu.reset_flag("C");

        self.last_instruction = "CB - SWAP H";
        self.operand_mode = 0;
        8
    }


    fn cb_swap_35(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = Opcode::swap(cpu.L);

        if cpu.L == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");
        cpu.reset_flag("C");

        self.last_instruction = "CB - SWAP L";
        self.operand_mode = 0;
        8
    }


    fn cb_swap_36(&mut self, cpu : &mut CPU) -> u8 {
        let addr = Opcode::byte_cat(cpu.H, cpu.L);
        let hl = cpu.RAM[addr as usize];
        cpu.write_ram(addr, Opcode::swap(hl));

        if cpu.RAM[addr as usize] == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");
        cpu.reset_flag("C");

        self.last_instruction = "CB - SWAP (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_swap_37(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = Opcode::swap(cpu.A);

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");
        cpu.reset_flag("C");

        self.last_instruction = "CB - SWAP A";
        self.operand_mode = 0;
        8
    }


    fn cb_sla_27(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 : u8 = (cpu.A & 0b1000_0000) >> 7;
        cpu.A = cpu.A << 1;

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 == 0 {
            cpu.reset_flag("C");
        } else {
            cpu.set_flag("C");
        }

        self.last_instruction = "CB - SLA A";
        self.operand_mode = 0;
        8
    }


    fn cb_sla_20(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 : u8 = (cpu.B & 0b1000_0000) >> 7;
        cpu.B = cpu.B << 1;

        if cpu.B == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 == 0 {
            cpu.reset_flag("C");
        } else {
            cpu.set_flag("C");
        }

        self.last_instruction = "CB - SLA B";
        self.operand_mode = 0;
        8
    }


    fn cb_sla_21(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 : u8 = (cpu.C & 0b1000_0000) >> 7;
        cpu.C = cpu.C << 1;

        if cpu.C == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 == 0 {
            cpu.reset_flag("C");
        } else {
            cpu.set_flag("C");
        }

        self.last_instruction = "CB - SLA C";
        self.operand_mode = 0;
        8
    }


    fn cb_sla_22(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 : u8 = (cpu.D & 0b1000_0000) >> 7;
        cpu.D = cpu.D << 1;

        if cpu.D == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 == 0 {
            cpu.reset_flag("C");
        } else {
            cpu.set_flag("C");
        }

        self.last_instruction = "CB - SLA D";
        self.operand_mode = 0;
        8
    }


    fn cb_sla_23(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 : u8 = (cpu.E & 0b1000_0000) >> 7;
        cpu.E = cpu.E << 1;

        if cpu.E == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 == 0 {
            cpu.reset_flag("C");
        } else {
            cpu.set_flag("C");
        }

        self.last_instruction = "CB - SLA E";
        self.operand_mode = 0;
        8
    }


    fn cb_sla_24(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 : u8 = (cpu.H & 0b1000_0000) >> 7;
        cpu.H = cpu.H << 1;

        if cpu.H == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 == 0 {
            cpu.reset_flag("C");
        } else {
            cpu.set_flag("C");
        }

        self.last_instruction = "CB - SLA H";
        self.operand_mode = 0;
        8
    }


    fn cb_sla_25(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 : u8 = (cpu.L & 0b1000_0000) >> 7;
        cpu.L = cpu.L << 1;

        if cpu.L == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 == 0 {
            cpu.reset_flag("C");
        } else {
            cpu.set_flag("C");
        }

        self.last_instruction = "CB - SLA L";
        self.operand_mode = 0;
        8
    }


    fn cb_sla_26(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];
        let addr = Opcode::byte_cat(cpu.H, cpu.L);
        let bit_7 : u8 = (hl & 0b1000_0000) >> 7;
        cpu.write_ram(addr, hl << 1);

        if cpu.RAM[addr as usize] == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 == 0 {
            cpu.reset_flag("C");
        } else {
            cpu.set_flag("C");
        }

        self.last_instruction = "CB - SLA (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_srl_38(&mut self, cpu : &mut CPU) -> u8 {
        if Opcode::get_bit(0, cpu.B) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.B = cpu.B >> 1;

        if cpu.B == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRL B";
        self.operand_mode = 0;
        8
    }


    fn cb_srl_39(&mut self, cpu : &mut CPU) -> u8 {
        if Opcode::get_bit(0, cpu.C) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.C = cpu.C >> 1;

        if cpu.C == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRL C";
        self.operand_mode = 0;
        8
    }


    fn cb_srl_3a(&mut self, cpu : &mut CPU) -> u8 {
        if Opcode::get_bit(0, cpu.D) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.D = cpu.D >> 1;

        if cpu.D == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRL D";
        self.operand_mode = 0;
        8
    }


    fn cb_srl_3b(&mut self, cpu : &mut CPU) -> u8 {
        if Opcode::get_bit(0, cpu.E) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.E = cpu.E >> 1;

        if cpu.E == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRL E";
        self.operand_mode = 0;
        8
    }


    fn cb_srl_3c(&mut self, cpu : &mut CPU) -> u8 {
        if Opcode::get_bit(0, cpu.H) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.H = cpu.H >> 1;

        if cpu.H == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRL H";
        self.operand_mode = 0;
        8
    }


    fn cb_srl_3d(&mut self, cpu : &mut CPU) -> u8 {
        if Opcode::get_bit(0, cpu.L) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.L = cpu.L >> 1;

        if cpu.L == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRL L";
        self.operand_mode = 0;
        8
    }


    fn cb_srl_3e(&mut self, cpu : &mut CPU) -> u8 {
        let addr = Opcode::byte_cat(cpu.H, cpu.L);
        let hl : u8 = cpu.RAM[addr as usize];

        if Opcode::get_bit(0, hl) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.write_ram(addr, hl >> 1);

        if cpu.RAM[addr as usize] == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRL (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_srl_3f(&mut self, cpu : &mut CPU) -> u8 {
        if Opcode::get_bit(0, cpu.A) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = cpu.A >> 1;

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRL A";
        self.operand_mode = 0;
        8
    }


    fn cb_sra_28(&mut self, cpu : &mut CPU) -> u8 {
        let msb : u8 = Opcode::get_bit(7, cpu.B);
        if Opcode::get_bit(0, cpu.B) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.B = cpu.B >> 1;
        cpu.B |= msb << 7;

        if cpu.B == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRA B";
        self.operand_mode = 0;
        8
    }


    fn cb_sra_29(&mut self, cpu : &mut CPU) -> u8 {
        let msb : u8 = Opcode::get_bit(7, cpu.C);
        if Opcode::get_bit(0, cpu.C) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.C = cpu.C >> 1;
        cpu.C |= msb << 7;

        if cpu.C == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRA C";
        self.operand_mode = 0;
        8
    }


    fn cb_sra_2a(&mut self, cpu : &mut CPU) -> u8 {
        let msb : u8 = Opcode::get_bit(7, cpu.D);
        if Opcode::get_bit(0, cpu.D) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.D = cpu.D >> 1;
        cpu.D |= msb << 7;

        if cpu.D == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRA D";
        self.operand_mode = 0;
        8
    }


    fn cb_sra_2b(&mut self, cpu : &mut CPU) -> u8 {
        let msb : u8 = Opcode::get_bit(7, cpu.E);
        if Opcode::get_bit(0, cpu.E) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.E = cpu.E >> 1;
        cpu.E |= msb << 7;

        if cpu.E == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRA E";
        self.operand_mode = 0;
        8
    }


    fn cb_sra_2c(&mut self, cpu : &mut CPU) -> u8 {
        let msb : u8 = Opcode::get_bit(7, cpu.H);
        if Opcode::get_bit(0, cpu.H) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.H = cpu.H >> 1;
        cpu.H |= msb << 7;

        if cpu.H == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRA H";
        self.operand_mode = 0;
        8
    }


    fn cb_sra_2d(&mut self, cpu : &mut CPU) -> u8 {
        let msb : u8 = Opcode::get_bit(7, cpu.L);
        if Opcode::get_bit(0, cpu.L) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.L = cpu.L >> 1;
        cpu.L |= msb << 7;

        if cpu.L == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRA L";
        self.operand_mode = 0;
        8
    }


    fn cb_sra_2e(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];
        let msb : u8 = Opcode::get_bit(7, hl);
        let addr = Opcode::byte_cat(cpu.H, cpu.L);

        if Opcode::get_bit(0, hl) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.write_ram(addr, hl >> 1);
        hl = cpu.RAM[addr as usize] | (msb << 7);
        cpu.write_ram(addr, hl);

        if cpu.RAM[addr as usize] == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRL (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_sra_2f(&mut self, cpu : &mut CPU) -> u8 {
        let msb : u8 = Opcode::get_bit(7, cpu.A);
        if Opcode::get_bit(0, cpu.A) == 1 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        cpu.A = cpu.A >> 1;
        cpu.A |= msb << 7;

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        self.last_instruction = "CB - SRA A";
        self.operand_mode = 0;
        8
    }


    fn cb_rlc_00(&mut self, cpu : &mut CPU) -> u8 {

        let bit_7 = cpu.B & 0x80 == 0x80;

        cpu.B = cpu.B << 1 | ( if bit_7 { 1 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.B == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RLC B";
        self.operand_mode = 0;
        8
    }


    fn cb_rlc_01(&mut self, cpu : &mut CPU) -> u8 {

        let bit_7 = cpu.C & 0x80 == 0x80;

        cpu.C = cpu.C << 1 | ( if bit_7 { 1 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.C == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RLC C";
        self.operand_mode = 0;
        8
    }


    fn cb_rlc_02(&mut self, cpu : &mut CPU) -> u8 {

        let bit_7 = cpu.D & 0x80 == 0x80;

        cpu.D = cpu.D << 1 | ( if bit_7 { 1 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.D == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RLC D";
        self.operand_mode = 0;
        8
    }


    fn cb_rlc_03(&mut self, cpu : &mut CPU) -> u8 {

        let bit_7 = cpu.E & 0x80 == 0x80;

        cpu.E = cpu.E << 1 | ( if bit_7 { 1 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.E == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RLC E";
        self.operand_mode = 0;
        8
    }


    fn cb_rlc_04(&mut self, cpu : &mut CPU) -> u8 {

        let bit_7 = cpu.H & 0x80 == 0x80;

        cpu.H = cpu.H << 1 | ( if bit_7 { 1 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.H == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RLC H";
        self.operand_mode = 0;
        8
    }


    fn cb_rlc_05(&mut self, cpu : &mut CPU) -> u8 {

        let bit_7 = cpu.L & 0x80 == 0x80;

        cpu.L = cpu.L << 1 | ( if bit_7 { 1 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.L == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RLC L";
        self.operand_mode = 0;
        8
    }


    fn cb_rlc_06(&mut self, cpu : &mut CPU) -> u8 {
        let hl : u8 = cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize];
        let addr = Opcode::byte_cat(cpu.H, cpu.L);

        let bit_7 = hl & 0x80 == 0x80;
        cpu.write_ram(addr, hl << 1 | ( if bit_7 { 1 } else { 0 }));

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.RAM[addr as usize] == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RLC (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_rlc_07(&mut self, cpu : &mut CPU) -> u8 {

        let bit_7 = cpu.A & 0x80 == 0x80;

        cpu.A = cpu.A << 1 | ( if bit_7 { 1 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RLC A";
        self.operand_mode = 0;
        8
    }


    fn cb_rrc_08(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.B & 0x01 == 0x01;

        cpu.B = cpu.B >> 1 | ( if bit_0 { 0x80 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.B == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RRC B";
        self.operand_mode = 0;
        8
    }


    fn cb_rrc_09(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.C & 0x01 == 0x01;

        cpu.C = cpu.C >> 1 | ( if bit_0 { 0x80 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.C == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RRC C";
        self.operand_mode = 0;
        8
    }


    fn cb_rrc_0a(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.D & 0x01 == 0x01;

        cpu.D = cpu.D >> 1 | ( if bit_0 { 0x80 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.D == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RRC D";
        self.operand_mode = 0;
        8
    }


    fn cb_rrc_0b(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.E & 0x01 == 0x01;

        cpu.E = cpu.E >> 1 | ( if bit_0 { 0x80 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.E == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RRC E";
        self.operand_mode = 0;
        8
    }


    fn cb_rrc_0c(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.H & 0x01 == 0x01;

        cpu.H = cpu.H >> 1 | ( if bit_0 { 0x80 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.H == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RRC H";
        self.operand_mode = 0;
        8
    }


    fn cb_rrc_0d(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.L & 0x01 == 0x01;

        cpu.L = cpu.L >> 1 | ( if bit_0 { 0x80 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.L == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RRC L";
        self.operand_mode = 0;
        8
    }


    fn cb_rrc_0e(&mut self, cpu : &mut CPU) -> u8 {
        let addr = Opcode::byte_cat(cpu.H, cpu.L);
        let hl : u8 = cpu.RAM[addr as usize];
        let bit_0 = hl & 0x01 == 0x01;

        cpu.write_ram(addr, hl >> 1 | ( if bit_0 { 0x80 } else { 0 }));

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.RAM[addr as usize] == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RRC (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_rrc_0f(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.A & 0x01 == 0x01;

        cpu.A = cpu.A >> 1 | ( if bit_0 { 0x80 } else { 0 });

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RRC A";
        self.operand_mode = 0;
        8
    }


    fn cb_rl_10(&mut self, cpu : &mut CPU) -> u8 {

        let bit_7 = cpu.B & 0x80 == 0x80;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.B = cpu.B << 1 | ( if cpu.get_flag("C") == 1 { 1 } else { 0 });

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.B == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RL B";
        self.operand_mode = 0;
        8
    }


    fn cb_rl_11(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 = cpu.C & 0x80 == 0x80;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.C = cpu.C << 1 | ( if cpu.get_flag("C") == 1 { 1 } else { 0 });

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.C == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RL C";
        self.operand_mode = 0;
        8
    }


    fn cb_rl_12(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 = cpu.D & 0x80 == 0x80;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.D = cpu.D << 1 | ( if cpu.get_flag("C") == 1 { 1 } else { 0 });

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.D == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RL D";
        self.operand_mode = 0;
        8
    }


    fn cb_rl_13(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 = cpu.E & 0x80 == 0x80;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.E = cpu.E << 1 | ( if cpu.get_flag("C") == 1 { 1 } else { 0 });

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.E == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RL E";
        self.operand_mode = 0;
        8
    }


    fn cb_rl_14(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 = cpu.H & 0x80 == 0x80;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.H = cpu.H << 1 | ( if cpu.get_flag("C") == 1 { 1 } else { 0 });

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.H == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RL H";
        self.operand_mode = 0;
        8
    }


    fn cb_rl_15(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 = cpu.L & 0x80 == 0x80;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.L = cpu.L << 1 | ( if cpu.get_flag("C") == 1 { 1 } else { 0 });

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.L == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RL L";
        self.operand_mode = 0;
        8
    }


    fn cb_rl_16(&mut self, cpu : &mut CPU) -> u8 {
        let addr = Opcode::byte_cat(cpu.H, cpu.L);
        let hl : u8 = cpu.RAM[addr as usize];
        let bit_7 = hl & 0x80 == 0x80;

        cpu.reset_flag("N");
        cpu.reset_flag("H");
        let c = cpu.get_flag("C");

        cpu.write_ram(addr,
        hl << 1 | ( if c == 1 { 1 } else { 0 }));

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.RAM[addr as usize] == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RL (HL)";
        self.operand_mode = 0;
        16
    }


    fn cb_rl_17(&mut self, cpu : &mut CPU) -> u8 {
        let bit_7 = cpu.A & 0x80 == 0x80;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.A = cpu.A << 1 | ( if cpu.get_flag("C") == 1 { 1 } else { 0 });

        if bit_7 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RL A";
        self.operand_mode = 0;
        8
    }


    fn cb_rr_1f(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.A & 0x01 == 0x01;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.A = cpu.A >> 1 | ( if cpu.get_flag("C") == 1 { 0x80 } else { 0 });

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.A == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RR A";
        self.operand_mode = 0;
        8
    }


    fn cb_rr_18(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.B & 0x01 == 0x01;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.B = cpu.B >> 1 | ( if cpu.get_flag("C") == 1 { 0x80 } else { 0 });

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.B == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RR B";
        self.operand_mode = 0;
        8
    }


    fn cb_rr_19(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.C & 0x01 == 0x01;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.C = cpu.C >> 1 | ( if cpu.get_flag("C") == 1 { 0x80 } else { 0 });

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.C == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RR C";
        self.operand_mode = 0;
        8
    }


    fn cb_rr_1a(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.D & 0x01 == 0x01;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.D = cpu.D >> 1 | ( if cpu.get_flag("C") == 1 { 0x80 } else { 0 });

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.D == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RR D";
        self.operand_mode = 0;
        8
    }


    fn cb_rr_1b(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.E & 0x01 == 0x01;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.E = cpu.E >> 1 | ( if cpu.get_flag("C") == 1 { 0x80 } else { 0 });

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.E == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RR E";
        self.operand_mode = 0;
        8
    }


    fn cb_rr_1c(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.H & 0x01 == 0x01;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.H = cpu.H >> 1 | ( if cpu.get_flag("C") == 1 { 0x80 } else { 0 });

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.H == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RR H";
        self.operand_mode = 0;
        8
    }


    fn cb_rr_1d(&mut self, cpu : &mut CPU) -> u8 {

        let bit_0 = cpu.L & 0x01 == 0x01;

        cpu.reset_flag("N");
        cpu.reset_flag("H");

        cpu.L = cpu.L >> 1 | ( if cpu.get_flag("C") == 1 { 0x80 } else { 0 });

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.L == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RR L";
        self.operand_mode = 0;
        8
    }


    fn cb_rr_1e(&mut self, cpu : &mut CPU) -> u8 {
        let addr = Opcode::byte_cat(cpu.H, cpu.L);
        let hl : u8 = cpu.RAM[addr as usize];
        let bit_0 = hl & 0x01 == 0x01;

        cpu.reset_flag("N");
        cpu.reset_flag("H");
        let c = cpu.get_flag("C");

        cpu.write_ram(addr,
        hl >> 1 | ( if c == 1 { 0x80 } else { 0 }));

        if bit_0 {
            cpu.set_flag("C");
        } else {
            cpu.reset_flag("C");
        }

        if cpu.RAM[addr as usize] == 0 {
            cpu.set_flag("Z");
        } else {
            cpu.reset_flag("Z");
        }

        self.last_instruction = "CB - RR (HL)";
        self.operand_mode = 0;
        16
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


    fn get_bit(n : u8, reg : u8) -> u8 {
        let mask : u8 = 1 << n;

        if reg & mask == 0 {
            0
        } else {
            1
        }
    }

    fn swap(reg : u8) -> u8 {
        let upper_nibble : u8 = reg & 0xF0;
        let lower_nibble : u8 = reg & 0x0F;

        (upper_nibble >> 4) | (lower_nibble << 4)
    }
















}
