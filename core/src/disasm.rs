// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Functions and structs for disassembling instructions

use hardware::memory::Memory;
use isa::*;

/// Decode an instruction
pub fn decode<M: Memory>(memory: &M, address: Address) -> Instruction {
    use self::Flag::*;
    use self::Instruction::*;
    use self::Register16::*;
    use self::Register8::*;

    /// Decode a `0xCB` prefixed instruction
    #[inline(always)]
    fn decode_cb<B: Memory>(address: Address, memory: &B) -> Instruction {
        match memory.read(address) {
            0x00 => RlcR(B),
            0x01 => RlcR(C),
            0x02 => RlcR(D),
            0x03 => RlcR(E),
            0x04 => RlcR(H),
            0x05 => RlcR(L),
            0x06 => RlcHl,
            0x07 => RlcR(A),
            0x08 => RrcR(B),
            0x09 => RrcR(C),
            0x0A => RrcR(D),
            0x0B => RrcR(E),
            0x0C => RrcR(H),
            0x0D => RrcR(L),
            0x0E => RrcHl,
            0x0F => RrcR(A),

            0x10 => RlR(B),
            0x11 => RlR(C),
            0x12 => RlR(D),
            0x13 => RlR(E),
            0x14 => RlR(H),
            0x15 => RlR(L),
            0x16 => RlHl,
            0x17 => RlR(A),
            0x18 => RrR(B),
            0x19 => RlR(C),
            0x1A => RlR(D),
            0x1B => RlR(E),
            0x1C => RlR(H),
            0x1D => RlR(L),
            0x1E => RlHl,
            0x1F => RlR(A),

            0x20 => SlaR(B),
            0x21 => SlaR(C),
            0x22 => SlaR(D),
            0x23 => SlaR(E),
            0x24 => SlaR(H),
            0x25 => SlaR(L),
            0x26 => SlaHl,
            0x27 => SlaR(A),
            0x28 => SraR(B),
            0x29 => SraR(C),
            0x2A => SraR(D),
            0x2B => SraR(E),
            0x2C => SraR(H),
            0x2D => SraR(L),
            0x2E => SraHl,
            0x2F => SraR(A),

            0x30 => SwapR(B),
            0x31 => SwapR(C),
            0x32 => SwapR(D),
            0x33 => SwapR(E),
            0x34 => SwapR(H),
            0x35 => SwapR(L),
            0x36 => SwapHl,
            0x37 => SwapR(A),
            0x38 => SrlR(B),
            0x39 => SrlR(C),
            0x3A => SrlR(D),
            0x3B => SrlR(E),
            0x3C => SrlR(H),
            0x3D => SrlR(L),
            0x3E => SrlHl,
            0x3F => SrlR(A),

            0x40 => BitIR(0, B),
            0x41 => BitIR(0, C),
            0x42 => BitIR(0, D),
            0x43 => BitIR(0, E),
            0x44 => BitIR(0, H),
            0x45 => BitIR(0, L),
            0x46 => BitIHl(0),
            0x47 => BitIR(0, A),
            0x48 => BitIR(1, B),
            0x49 => BitIR(1, C),
            0x4A => BitIR(1, D),
            0x4B => BitIR(1, E),
            0x4C => BitIR(1, H),
            0x4D => BitIR(1, L),
            0x4E => BitIHl(1),
            0x4F => BitIR(1, A),

            0x50 => BitIR(2, B),
            0x51 => BitIR(2, C),
            0x52 => BitIR(2, D),
            0x53 => BitIR(2, E),
            0x54 => BitIR(2, H),
            0x55 => BitIR(2, L),
            0x56 => BitIHl(2),
            0x57 => BitIR(2, A),
            0x58 => BitIR(3, B),
            0x59 => BitIR(3, C),
            0x5A => BitIR(3, D),
            0x5B => BitIR(3, E),
            0x5C => BitIR(3, H),
            0x5D => BitIR(3, L),
            0x5E => BitIHl(3),
            0x5F => BitIR(3, A),

            0x60 => BitIR(4, B),
            0x61 => BitIR(4, C),
            0x62 => BitIR(4, D),
            0x63 => BitIR(4, E),
            0x64 => BitIR(4, H),
            0x65 => BitIR(4, L),
            0x66 => BitIHl(4),
            0x67 => BitIR(4, A),
            0x68 => BitIR(5, B),
            0x69 => BitIR(5, C),
            0x6A => BitIR(5, D),
            0x6B => BitIR(5, E),
            0x6C => BitIR(5, H),
            0x6D => BitIR(5, L),
            0x6E => BitIHl(5),
            0x6F => BitIR(5, A),

            0x70 => BitIR(6, B),
            0x71 => BitIR(6, C),
            0x72 => BitIR(6, D),
            0x73 => BitIR(6, E),
            0x74 => BitIR(6, H),
            0x75 => BitIR(6, L),
            0x76 => BitIHl(6),
            0x77 => BitIR(6, A),
            0x78 => BitIR(7, B),
            0x79 => BitIR(7, C),
            0x7A => BitIR(7, D),
            0x7B => BitIR(7, E),
            0x7C => BitIR(7, H),
            0x7D => BitIR(7, L),
            0x7E => BitIHl(7),
            0x7F => BitIR(7, A),

            0x80 => ResIR(0, B),
            0x81 => ResIR(0, C),
            0x82 => ResIR(0, D),
            0x83 => ResIR(0, E),
            0x84 => ResIR(0, H),
            0x85 => ResIR(0, L),
            0x86 => ResIHl(0),
            0x87 => ResIR(0, A),
            0x88 => ResIR(1, B),
            0x89 => ResIR(1, C),
            0x8A => ResIR(1, D),
            0x8B => ResIR(1, E),
            0x8C => ResIR(1, H),
            0x8D => ResIR(1, L),
            0x8E => ResIHl(1),
            0x8F => ResIR(1, A),

            0x90 => ResIR(2, B),
            0x91 => ResIR(2, C),
            0x92 => ResIR(2, D),
            0x93 => ResIR(2, E),
            0x94 => ResIR(2, H),
            0x95 => ResIR(2, L),
            0x96 => ResIHl(2),
            0x97 => ResIR(2, A),
            0x98 => ResIR(3, B),
            0x99 => ResIR(3, C),
            0x9A => ResIR(3, D),
            0x9B => ResIR(3, E),
            0x9C => ResIR(3, H),
            0x9D => ResIR(3, L),
            0x9E => ResIHl(3),
            0x9F => ResIR(3, A),

            0xA0 => ResIR(4, B),
            0xA1 => ResIR(4, C),
            0xA2 => ResIR(4, D),
            0xA3 => ResIR(4, E),
            0xA4 => ResIR(4, H),
            0xA5 => ResIR(4, L),
            0xA6 => ResIHl(4),
            0xA7 => ResIR(4, A),
            0xA8 => ResIR(5, B),
            0xA9 => ResIR(5, C),
            0xAA => ResIR(5, D),
            0xAB => ResIR(5, E),
            0xAC => ResIR(5, H),
            0xAD => ResIR(5, L),
            0xAE => ResIHl(5),
            0xAF => ResIR(5, A),

            0xB0 => ResIR(6, B),
            0xB1 => ResIR(6, C),
            0xB2 => ResIR(6, D),
            0xB3 => ResIR(6, E),
            0xB4 => ResIR(6, H),
            0xB5 => ResIR(6, L),
            0xB6 => ResIHl(6),
            0xB7 => ResIR(6, A),
            0xB8 => ResIR(7, B),
            0xB9 => ResIR(7, C),
            0xBA => ResIR(7, D),
            0xBB => ResIR(7, E),
            0xBC => ResIR(7, H),
            0xBD => ResIR(7, L),
            0xBE => ResIHl(7),
            0xBF => ResIR(7, A),

            0xC0 => SetIR(0, B),
            0xC1 => SetIR(0, C),
            0xC2 => SetIR(0, D),
            0xC3 => SetIR(0, E),
            0xC4 => SetIR(0, H),
            0xC5 => SetIR(0, L),
            0xC6 => SetIHl(0),
            0xC7 => SetIR(0, A),
            0xC8 => SetIR(1, B),
            0xC9 => SetIR(1, C),
            0xCA => SetIR(1, D),
            0xCB => SetIR(1, E),
            0xCC => SetIR(1, H),
            0xCD => SetIR(1, L),
            0xCE => SetIHl(1),
            0xCF => SetIR(1, A),

            0xD0 => SetIR(2, B),
            0xD1 => SetIR(2, C),
            0xD2 => SetIR(2, D),
            0xD3 => SetIR(2, E),
            0xD4 => SetIR(2, H),
            0xD5 => SetIR(2, L),
            0xD6 => SetIHl(2),
            0xD7 => SetIR(2, A),
            0xD8 => SetIR(3, B),
            0xD9 => SetIR(3, C),
            0xDA => SetIR(3, D),
            0xDB => SetIR(3, E),
            0xDC => SetIR(3, H),
            0xDD => SetIR(3, L),
            0xDE => SetIHl(3),
            0xDF => SetIR(3, A),

            0xE0 => SetIR(4, B),
            0xE1 => SetIR(4, C),
            0xE2 => SetIR(4, D),
            0xE3 => SetIR(4, E),
            0xE4 => SetIR(4, H),
            0xE5 => SetIR(4, L),
            0xE6 => SetIHl(4),
            0xE7 => SetIR(4, A),
            0xE8 => SetIR(5, B),
            0xE9 => SetIR(5, C),
            0xEA => SetIR(5, D),
            0xEB => SetIR(5, E),
            0xEC => SetIR(5, H),
            0xED => SetIR(5, L),
            0xEE => SetIHl(5),
            0xEF => SetIR(5, A),

            0xF0 => SetIR(6, B),
            0xF1 => SetIR(6, C),
            0xF2 => SetIR(6, D),
            0xF3 => SetIR(6, E),
            0xF4 => SetIR(6, H),
            0xF5 => SetIR(6, L),
            0xF6 => SetIHl(6),
            0xF7 => SetIR(6, A),
            0xF8 => SetIR(7, B),
            0xF9 => SetIR(7, C),
            0xFA => SetIR(7, D),
            0xFB => SetIR(7, E),
            0xFC => SetIR(7, H),
            0xFD => SetIR(7, L),
            0xFE => SetIHl(7),
            0xFF => SetIR(7, A),
            _ => unreachable!(),
        }
    }

    let next_address = address + 1;
    match memory.read(address) {
        0x00 => Nop,
        0x01 => LdRrIi(BC, memory.read_double(next_address) as Immediate16),
        0x02 => LdRrA(BC),
        0x03 => IncRr(BC),
        0x04 => IncR(B),
        0x05 => DecR(B),
        0x06 => LdRI(B, memory.read(next_address) as Immediate),
        0x07 => Rlca,
        0x08 => LdIiSp(memory.read_double(next_address) as Immediate16),
        0x09 => AddHlRr(BC),
        0x0A => LdARr(BC),
        0x0B => DecRr(BC),
        0x0C => IncR(C),
        0x0D => DecR(C),
        0x0E => LdRI(C, memory.read(next_address) as Immediate),
        0x0F => Rrca,

        0x10 => Stop,
        0x11 => LdRrIi(DE, memory.read_double(next_address) as Immediate16),
        0x12 => LdRrA(DE),
        0x13 => IncRr(DE),
        0x14 => IncR(D),
        0x15 => DecR(D),
        0x16 => LdRI(D, memory.read(next_address) as Immediate),
        0x17 => Rla,
        0x18 => JrS(memory.read(next_address) as SignedImmediate),
        0x19 => AddHlRr(DE),
        0x1A => LdARr(BC),
        0x1B => DecRr(DE),
        0x1C => IncR(E),
        0x1D => DecR(E),
        0x1E => LdRI(E, memory.read(next_address) as Immediate),
        0x1F => Rra,

        0x20 => JrCondS(Nf, memory.read(next_address) as SignedImmediate),
        0x21 => LdRrIi(HL, memory.read(next_address) as Immediate16),
        0x22 => LdiHlA,
        0x23 => IncHl,
        0x24 => IncR(H),
        0x25 => DecR(H),
        0x26 => LdRI(H, memory.read(next_address) as Immediate),
        0x27 => Daa,
        0x28 => JrCondS(Zf, memory.read(next_address) as SignedImmediate),
        0x29 => AddHlRr(DE),
        0x2A => LdiAHl,
        0x2B => DecRr(HL),
        0x2C => IncR(L),
        0x2D => DecR(L),
        0x2E => LdRI(L, memory.read(next_address) as Immediate),
        0x2F => Cpl,

        0x30 => JrCondS(Cf, memory.read(next_address) as SignedImmediate),
        0x31 => LdRrIi(SP, memory.read_double(next_address) as Immediate16),
        0x32 => LddHlA,
        0x33 => IncRr(SP),
        0x34 => IncHl,
        0x35 => DecHl,
        0x36 => LdHlI(memory.read(next_address) as Immediate),
        0x37 => Scf,
        0x38 => JrCondS(Cf, memory.read(next_address) as SignedImmediate),
        0x39 => IncR(A),
        0x3A => LddAHl,
        0x3B => DecRr(SP),
        0x3C => IncR(A),
        0x3D => DecR(A),
        0x3E => LdAI(memory.read(next_address) as Immediate),
        0x3F => Ccf,

        0x40 => LdRR(B, B),
        0x41 => LdRR(B, C),
        0x42 => LdRR(B, D),
        0x43 => LdRR(B, E),
        0x44 => LdRR(B, H),
        0x45 => LdRR(B, L),
        0x46 => LdRHl(B),
        0x47 => LdRR(B, A),
        0x48 => LdRR(C, B),
        0x49 => LdRR(C, C),
        0x4A => LdRR(C, D),
        0x4B => LdRR(C, E),
        0x4C => LdRR(C, H),
        0x4D => LdRR(C, L),
        0x4E => LdRHl(C),
        0x4F => LdRR(C, A),

        0x50 => LdRR(D, B),
        0x51 => LdRR(D, C),
        0x52 => LdRR(D, D),
        0x53 => LdRR(D, E),
        0x54 => LdRR(D, H),
        0x55 => LdRR(D, L),
        0x56 => LdRHl(D),
        0x57 => LdRR(D, A),
        0x58 => LdRR(E, B),
        0x59 => LdRR(E, C),
        0x5A => LdRR(E, D),
        0x5B => LdRR(E, E),
        0x5C => LdRR(E, H),
        0x5D => LdRR(E, L),
        0x5E => LdRHl(E),
        0x5F => LdRR(E, A),

        0x60 => LdRR(H, B),
        0x61 => LdRR(H, C),
        0x62 => LdRR(H, D),
        0x63 => LdRR(H, E),
        0x64 => LdRR(H, H),
        0x65 => LdRR(H, L),
        0x66 => LdRHl(H),
        0x67 => LdRR(H, A),
        0x68 => LdRR(L, B),
        0x69 => LdRR(L, C),
        0x6A => LdRR(L, D),
        0x6B => LdRR(L, E),
        0x6C => LdRR(L, H),
        0x6D => LdRR(L, L),
        0x6E => LdRHl(L),
        0x6F => LdRR(L, A),

        0x70 => LdHlR(B),
        0x71 => LdHlR(C),
        0x72 => LdHlR(D),
        0x73 => LdHlR(E),
        0x74 => LdHlR(H),
        0x75 => LdHlR(L),
        0x76 => Stop,
        0x77 => LdHlR(A),
        0x78 => LdRR(A, B),
        0x79 => LdRR(A, C),
        0x7A => LdRR(A, D),
        0x7B => LdRR(A, E),
        0x7C => LdRR(A, H),
        0x7D => LdRR(A, L),
        0x7E => LdRHl(A),
        0x7F => LdRR(A, A),

        0x80 => AddAR(B),
        0x81 => AddAR(C),
        0x82 => AddAR(D),
        0x83 => AddAR(E),
        0x84 => AddAR(H),
        0x85 => AddAR(L),
        0x86 => AddAHl,
        0x87 => AddAR(A),
        0x88 => AdcAR(B),
        0x89 => AdcAR(C),
        0x8A => AdcAR(D),
        0x8B => AdcAR(E),
        0x8C => AdcAR(H),
        0x8D => AdcAR(L),
        0x8E => AdcAHl,
        0x8F => AdcAR(A),

        0x90 => SubAR(B),
        0x91 => SubAR(C),
        0x92 => SubAR(D),
        0x93 => SubAR(E),
        0x94 => SubAR(H),
        0x95 => SubAR(L),
        0x96 => SubAHl,
        0x97 => SubAR(A),
        0x98 => SbcAR(B),
        0x99 => SbcAR(C),
        0x9A => SbcAR(D),
        0x9B => SbcAR(E),
        0x9C => SbcAR(H),
        0x9D => SbcAR(L),
        0x9E => SbcAHl,
        0x9F => SbcAR(A),

        0xA0 => AndAR(B),
        0xA1 => AndAR(C),
        0xA2 => AndAR(D),
        0xA3 => AndAR(E),
        0xA4 => AndAR(H),
        0xA5 => AndAR(L),
        0xA6 => AndAHl,
        0xA7 => AndAR(A),
        0xA8 => XorAR(B),
        0xA9 => XorAR(C),
        0xAA => XorAR(D),
        0xAB => XorAR(E),
        0xAC => XorAR(H),
        0xAD => XorAR(L),
        0xAE => XorAHl,
        0xAF => XorAR(A),

        0xB0 => OrAR(B),
        0xB1 => OrAR(C),
        0xB2 => OrAR(D),
        0xB3 => OrAR(E),
        0xB4 => OrAR(H),
        0xB5 => OrAR(L),
        0xB6 => OrAHl,
        0xB7 => OrAR(A),
        0xB8 => CpAR(B),
        0xB9 => CpAR(C),
        0xBA => CpAR(D),
        0xBB => CpAR(E),
        0xBC => CpAR(H),
        0xBD => CpAR(L),
        0xBE => CpAHl,
        0xBF => CpAR(A),

        0xC0 => RetCond(Cf),
        0xC1 => Pop(BC),
        0xC2 => JpCondIi(Zf, memory.read_double(next_address) as Address),
        0xC3 => JpIi(memory.read_double(next_address) as Address),
        0xC4 => CallCondIi(Zf, memory.read_double(next_address) as Address),
        0xC5 => PushRr(BC),
        0xC6 => AddAI(memory.read(next_address) as Immediate),
        0xC7 => Rst(memory.read(next_address) as Immediate),
        0xC8 => RetCond(Zf),
        0xC9 => Ret,
        0xCA => JpCondIi(Zf, memory.read_double(next_address) as Address),
        0xCB => decode_cb(next_address, memory),
        0xCC => CallCondIi(Zf, memory.read_double(next_address) as Address),
        0xCD => CallIi(memory.read_double(next_address) as Address),
        0xCE => AdcAI(memory.read(next_address) as Immediate),
        0xCF => Rst(memory.read(next_address) as Immediate),

        0xD0 => RetCond(Cf),
        0xD1 => Pop(DE),
        0xD2 => JpCondIi(Zf, memory.read_double(next_address) as Address),
        v @ 0xD3 => Undefined(v),
        0xD4 => CallCondIi(Zf, memory.read_double(next_address) as Address),
        0xD5 => PushRr(DE),
        0xD6 => SubAI(memory.read(next_address) as Immediate),
        0xD7 => Rst(memory.read(next_address) as Immediate),
        0xD8 => RetCond(Cf),
        0xD9 => Reti,
        0xDA => JpCondIi(Cf, memory.read_double(next_address) as Address),
        v @ 0xDB => Undefined(v),
        0xDC => CallCondIi(Cf, memory.read_double(next_address) as Address),
        v @ 0xDD => Undefined(v),
        0xDE => SbcAI(memory.read(next_address) as Immediate),
        0xDF => Rst(memory.read(next_address) as Immediate),

        0xE0 => LdIoA(memory.read(next_address) as Immediate),
        0xE1 => Pop(HL),
        0xE2 => LdIocA,
        v @ 0xE3 => Undefined(v),
        v @ 0xE4 => Undefined(v),
        0xE5 => PushRr(HL),
        0xE6 => AndAI(memory.read(next_address) as Immediate),
        0xE7 => Rst(memory.read(next_address) as Immediate),
        0xE8 => AddSpS(memory.read(next_address) as SignedImmediate),
        0xE9 => JpHl,
        0xEA => LdIiA,
        v @ 0xEB => Undefined(v),
        v @ 0xEC => Undefined(v),
        v @ 0xED => Undefined(v),
        0xEE => XorAI(memory.read(next_address) as Immediate),
        0xEF => Rst(memory.read(next_address) as Immediate),

        0xF0 => LdAIo(memory.read(next_address) as Immediate),
        0xF1 => Pop(AF),
        0xF2 => LdAIoc,
        0xF3 => Di,
        v @ 0xF4 => Undefined(v),
        0xF5 => PushRr(AF),
        0xF6 => OrAI(memory.read(next_address) as Immediate),
        0xF7 => Rst(memory.read(next_address) as Immediate),
        0xF8 => LdHlSp(memory.read(next_address) as SignedImmediate),
        0xF9 => LdSpHl,
        0xFA => unimplemented!(), // TODO: implement me
        0xFB => Ei,
        v @ 0xFC => Undefined(v),
        v @ 0xFD => Undefined(v),
        0xFE => CpAI(memory.read(next_address) as Immediate),
        0xFF => Rst(memory.read(next_address) as Immediate),
        _ => unreachable!(),
    }
}
