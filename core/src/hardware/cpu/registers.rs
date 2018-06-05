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
    pub fn new() -> Registers {
        unimplemented!()
    }

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
    pub fn z_is_set(&self) -> bool {
        (self.f & 0b0001_0000) != 0
    }

    /// Return the value of the `CARRY` flag
    pub fn cy_is_set(&self) -> bool {
        (self.f & 0b1000_0000) != 0
    }

    /// Return the value of the `ADD/SUB` flag
    pub fn n_is_set(&self) -> bool {
        (self.f & 0b0010_0000) != 0
    }

    /// Return the value of the `HALF-CARRY` flag
    pub fn h_is_set(&self) -> bool {
        (self.f & 0b0100_0000) != 0
    }
}

#[cfg(test)]
mod test {}
