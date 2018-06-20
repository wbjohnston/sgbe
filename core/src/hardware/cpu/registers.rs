// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy CPU register file

use std::fmt;

use hardware::{pack_words, split_doubleword};
use isa::{DoubleWord, Flag, Register16, Register8, Word};

const ZF_FLAG_BIT_N: u8 = 7;
const NF_FLAG_BIT_N: u8 = 6;
const HF_FLAG_BIT_N: u8 = 5;
const CF_FLAG_BIT_N: u8 = 4;

const DEFAULT_A_VALUE: Word = 0x00;
const DEFAULT_F_VALUE: Word = 0x00;
const DEFAULT_B_VALUE: Word = 0x00;
const DEFAULT_C_VALUE: Word = 0x00;
const DEFAULT_D_VALUE: Word = 0x00;
const DEFAULT_E_VALUE: Word = 0x00;
const DEFAULT_H_VALUE: Word = 0x00;
const DEFAULT_L_VALUE: Word = 0x00;
const DEFAULT_SP_VALUE: DoubleWord = 0x0000;
const DEFAULT_PC_VALUE: DoubleWord = 0x0000;

// TODO:  implement flag register as bitflags

/// A Gameboy CPU register file
#[derive(Debug, Copy, Clone)]
pub struct Registers {
    pub a: Word,
    pub f: Word,
    pub b: Word,
    pub c: Word,
    pub d: Word,
    pub e: Word,
    pub h: Word,
    pub l: Word,
    pub sp: DoubleWord,
    pub pc: DoubleWord,
}

impl Registers {
    /// Return the value in an 8-bit register
    pub fn read_register8(&self, register: Register8) -> Word {
        use self::Register8::*;
        match register {
            A => self.a,
            F => self.f,
            B => self.b,
            C => self.c,
            D => self.d,
            E => self.e,
            H => self.h,
            L => self.l,
        }
    }

    /// Set the value in an 8-bit register
    pub fn write_register8(&mut self, register: Register8, value: Word) {
        use self::Register8::*;
        match register {
            A => self.a = value,
            F => self.f = value,
            B => self.b = value,
            C => self.c = value,
            D => self.d = value,
            E => self.e = value,
            H => self.h = value,
            L => self.l = value,
        }
    }

    /// Get the value in an 16-bit register
    pub fn read_register16(&self, register: Register16) -> DoubleWord {
        use self::Register16::*;
        use self::Register8::*;
        match register {
            AF => pack_words(self.read_register8(F), self.read_register8(A)),
            BC => pack_words(self.read_register8(C), self.read_register8(B)),
            DE => pack_words(self.read_register8(E), self.read_register8(D)),
            HL => pack_words(self.read_register8(L), self.read_register8(H)),
            SP => self.sp,
            PC => self.pc,
        }
    }

    /// Set the value in an 16-bit register
    pub fn write_register16(&mut self, register: Register16, value: DoubleWord) {
        use self::Register16::*;
        use self::Register8::*;
        let (lo, hi) = split_doubleword(value);
        match register {
            AF => {
                self.write_register8(A, hi);
                self.write_register8(F, lo);
            }
            BC => {
                self.write_register8(B, hi);
                self.write_register8(C, lo);
            }
            DE => {
                self.write_register8(D, hi);
                self.write_register8(E, lo)
            }
            HL => {
                self.write_register8(H, hi);
                self.write_register8(L, lo)
            }
            SP => self.sp = value,
            PC => self.pc = value,
        }
    }

    /// Return the value of a flag
    pub fn read_flag(&self, flag: Flag) -> bool {
        use self::Flag::*;
        match flag {
            Zero => self.f & (1 << ZF_FLAG_BIT_N) != 0,
            AddSub => self.f & (1 << NF_FLAG_BIT_N) != 0,
            HalfCarry => self.f & (1 << HF_FLAG_BIT_N) != 0,
            Carry => self.f & (1 << CF_FLAG_BIT_N) != 0,
        }
    }

    /// Write the value of a flag
    pub fn write_flag(&mut self, flag: Flag, value: bool) {
        use self::Flag::*;

        fn set_bit(byte: &mut u8, n: u8) {
            *byte |= 1 << n;
        }

        fn unset_bit(byte: &mut u8, n: u8) {
            *byte &= !(1 << n);
        }

        if value {
            match flag {
                Zero => set_bit(&mut self.f, ZF_FLAG_BIT_N),
                AddSub => set_bit(&mut self.f, NF_FLAG_BIT_N),
                HalfCarry => set_bit(&mut self.f, HF_FLAG_BIT_N),
                Carry => set_bit(&mut self.f, CF_FLAG_BIT_N),
            }
        } else {
            match flag {
                Zero => unset_bit(&mut self.f, ZF_FLAG_BIT_N),
                AddSub => unset_bit(&mut self.f, NF_FLAG_BIT_N),
                HalfCarry => unset_bit(&mut self.f, HF_FLAG_BIT_N),
                Carry => unset_bit(&mut self.f, CF_FLAG_BIT_N),
            }
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: DEFAULT_A_VALUE,
            f: DEFAULT_F_VALUE,
            b: DEFAULT_B_VALUE,
            c: DEFAULT_C_VALUE,
            d: DEFAULT_D_VALUE,
            e: DEFAULT_E_VALUE,
            h: DEFAULT_H_VALUE,
            l: DEFAULT_L_VALUE,
            sp: DEFAULT_SP_VALUE,
            pc: DEFAULT_PC_VALUE,
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Register16::*;
        write!(
            f,
            r#"
            A: {:4x} F: {:4x} AF: {:8x}
            B: {:4x} C: {:4x} BC: {:8x}
            D: {:4x} E: {:4x} DE: {:8x}
            H: {:4x} L: {:4x} HL: {:8x}
            SP: {:4x}
            PC: {:4x}
            "#,
            self.a,
            self.f,
            self.read_register16(AF),
            self.b,
            self.c,
            self.read_register16(BC),
            self.d,
            self.e,
            self.read_register16(DE),
            self.h,
            self.l,
            self.read_register16(HL),
            self.sp,
            self.pc,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn double_registers_set() {
        use self::Register16::*;
        use self::Register8::*;
        let mut registers = Registers::default();

        registers.write_register16(HL, 0xFFA0);
        assert_eq!(registers.read_register16(HL), 0xFFA0);
        assert_eq!(registers.read_register8(H), 0xFF);
        assert_eq!(registers.read_register8(L), 0xA0);

        registers.write_register16(BC, 0xABCD);
        assert_eq!(registers.read_register16(BC), 0xABCD);
        assert_eq!(registers.read_register8(B), 0xAB);
        assert_eq!(registers.read_register8(C), 0xCD);

        registers.write_register16(DE, 0xDEAD);
        assert_eq!(registers.read_register16(DE), 0xDEAD);
        assert_eq!(registers.read_register8(D), 0xDE);
        assert_eq!(registers.read_register8(E), 0xAD);

        // Verify that changing other registers doesn't interfere
        assert_eq!(registers.read_register16(HL), 0xFFA0);
        assert_eq!(registers.read_register16(BC), 0xABCD);
        assert_eq!(registers.read_register16(DE), 0xDEAD);
    }

    #[test]
    fn flag_set_functions() {
        use self::Flag::*;
        let mut registers = Registers::default();

        // Test zf
        registers.write_flag(Zero, true);
        assert!(registers.read_flag(Zero));
        registers.write_flag(Zero, false);
        assert!(!registers.read_flag(Zero));

        registers.write_flag(AddSub, true);
        assert!(registers.read_flag(AddSub));
        registers.write_flag(AddSub, false);
        assert!(!registers.read_flag(AddSub));

        registers.write_flag(HalfCarry, true);
        assert!(registers.read_flag(HalfCarry));
        registers.write_flag(HalfCarry, false);
        assert!(!registers.read_flag(HalfCarry));

        registers.write_flag(Carry, true);
        assert!(registers.read_flag(Carry));
        registers.write_flag(Carry, false);
        assert!(!registers.read_flag(Carry));
    }
}
