// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy CPU register file

use isa::{DoubleWord, Word};
use hardware::{pack_words, split_doubleword, split_word};

const DEFAULT_A_VALUE: Word = 0x01;
const DEFAULT_F_VALUE: Word = 0xB0;
const DEFAULT_B_VALUE: Word = 0x00;
const DEFAULT_C_VALUE: Word = 0x13;
const DEFAULT_D_VALUE: Word = 0x00;
const DEFAULT_E_VALUE: Word = 0xD8;
const DEFAULT_H_VALUE: Word = 0x01;
const DEFAULT_L_VALUE: Word = 0x4D;
const DEFAULT_SP_VALUE: DoubleWord = 0xFFFE; // FIXME: not the right value
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
    pub sp: DoubleWord,
    pub pc: DoubleWord,
}

impl Registers {
    /// Set the value of the `HL` register
    pub fn set_bc(&mut self, value: DoubleWord) {
        let (lo, hi) = split_doubleword(value);
        self.b = lo;
        self.c = hi;
    }

    /// Return the value in the `BC` register
    pub fn bc(&self) -> DoubleWord {
        pack_words(self.b, self.c)
    }

    /// Set the value of the `HL` register
    pub fn set_de(&mut self, value: DoubleWord) {
        let (lo, hi) = split_doubleword(value);
        self.d = lo;
        self.e = hi;
    }

    /// Return the value in the `DE` register
    pub fn de(&self) -> DoubleWord {
        pack_words(self.d, self.e)
    }

    /// Set the value of the `HL` register
    pub fn set_hl(&mut self, value: DoubleWord) {
        let (lo, hi) = split_doubleword(value);
        self.h = lo;
        self.l = hi;
    }

    /// Return the value in the `HL` register
    pub fn hl(&self) -> DoubleWord {
        pack_words(self.h, self.l)
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

        registers.set_hl(0xDEED);
        assert_eq!(registers.de(), 0xDEED);
        assert_eq!(registers.d, 0xDE);
        assert_eq!(registers.e, 0xED);
    }
}

