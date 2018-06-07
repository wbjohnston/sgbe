// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy CPU register file

use std::fmt;

use hardware::{pack_words, split_doubleword, split_word};
use isa::{DoubleWord, Word};

const DEFAULT_IR_VALUE: Word = 0x00;
const DEFAULT_A_VALUE: Word = 0x00;
const DEFAULT_F_VALUE: Word = 0x00;
const DEFAULT_B_VALUE: Word = 0x00;
const DEFAULT_C_VALUE: Word = 0x00;
const DEFAULT_D_VALUE: Word = 0x00;
const DEFAULT_E_VALUE: Word = 0x00;
const DEFAULT_H_VALUE: Word = 0x00;
const DEFAULT_L_VALUE: Word = 0x00;
const DEFAULT_SP_VALUE: DoubleWord = 0x0000; // FIXME: not the right value
const DEFAULT_PC_VALUE: DoubleWord = 0x0000;

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
    pub ir: Word,
    pub sp: DoubleWord,
    pub pc: DoubleWord,
}

impl Registers {
    /// Set the value of the `HL` register
    pub fn set_bc(&mut self, value: DoubleWord) {
        let (lo, hi) = split_doubleword(value);
        self.b = hi;
        self.c = lo;
    }

    /// Return the value in the `BC` register
    pub fn bc(&self) -> DoubleWord {
        pack_words(self.c, self.b)
    }

    /// Set the value of the `DE` register
    pub fn set_de(&mut self, value: DoubleWord) {
        let (lo, hi) = split_doubleword(value);
        self.d = hi;
        self.e = lo;
    }

    /// Return the value in the `DE` register
    pub fn de(&self) -> DoubleWord {
        pack_words(self.e, self.d)
    }

    /// Set the value of the `HL` register
    pub fn set_hl(&mut self, value: DoubleWord) {
        let (lo, hi) = split_doubleword(value);
        self.h = hi;
        self.l = lo;
    }

    /// Return the value in the `HL` register
    pub fn hl(&self) -> DoubleWord {
        pack_words(self.l, self.h)
    }

    fn af(&self) -> DoubleWord {
        pack_words(self.a, self.f)
    }

    /// Return the value of the `ZERO` flag
    pub fn zf_is_set(&self) -> bool {
        (self.f & 0b0001_0000) != 0
    }

    /// Return the value of the `CARRY` flag
    pub fn cf_is_set(&self) -> bool {
        (self.f & 0b1000_0000) != 0
    }

    /// Return the value of the `ADD/SUB` flag
    pub fn nf_is_set(&self) -> bool {
        (self.f & 0b0010_0000) != 0
    }

    /// Return the value of the `HALF-CARRY` flag
    pub fn hf_is_set(&self) -> bool {
        (self.f & 0b0100_0000) != 0
    }
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            ir: DEFAULT_IR_VALUE,
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
        write!(
            f,
            r#"
            A: {:4x} F: {:4x} AF: {:8x}
            B: {:4x} C: {:4x} BC: {:8x}
            D: {:4x} E: {:4x} DE: {:8x}
            H: {:4x} L: {:4x} HL: {:8x}
            SP: {:4x}
            PC: {:4x}
            IR: {:4x}
            "#,
            self.a,
            self.f,
            self.af(),
            self.b,
            self.c,
            self.bc(),
            self.d,
            self.e,
            self.de(),
            self.h,
            self.l,
            self.hl(),
            self.sp,
            self.pc,
            self.ir,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn double_registers_set() {
        let mut registers = Registers::default();
        registers.set_hl(0xFFA0);

        assert_eq!(registers.hl(), 0xFFA0);
        assert_eq!(registers.h, 0xFF);
        assert_eq!(registers.l, 0xA0);

        registers.set_bc(0xABCD);
        assert_eq!(registers.bc(), 0xABCD);
        assert_eq!(registers.b, 0xAB);
        assert_eq!(registers.c, 0xCD);

        registers.set_de(0xDEAD);
        assert_eq!(registers.de(), 0xDEAD);
        assert_eq!(registers.d, 0xDE);
        assert_eq!(registers.e, 0xAD);
    }
}
