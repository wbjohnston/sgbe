// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Memory structures and address info

pub mod addresses;

use hardware::pack_words;
use isa::{Word, Address, DoubleWord};

/// Number of bytes in one Kilobyte
const KB: usize = 1024;

type Memory4KbInner = Box<[Word; KB * 4]>;

type Memory8KbInner = Box<[Word; KB * 8]>;

type Memory16KbInner = Box<[Word; KB * 16]>;

type Memory32KbInner = Box<[Word; KB * 32]>;

/// A bus to read and write data
pub trait Memory {
    /// Read a value from an address
    fn read(&self, address: Address) -> Word;

    /// Write a value to an address
    fn write(&mut self, address: Address, value: Word);

    /// Read a `DoubleWord`
    fn read_double(&self, address: Address) -> DoubleWord {
        let lo = self.read(address);
        let hi = self.read(address + 1);

        pack_words(lo, hi)
    }
}

/// A switchable memory bank
pub trait Switchable: Memory {
    /// Switch the active memory bank
    fn switch_bank(&mut self, bank_idx: u8);
}

/// A 4KB memory bank
#[derive(Clone)]
pub struct Memory4Kb(Memory4KbInner);

impl Memory for Memory4Kb {
    fn read(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write(&mut self, address: Address, value: Word) {
        unimplemented!()
    }
}

impl Memory4Kb {
    pub fn new() -> Self {
        Memory4Kb(Box::new([0; 4 * KB]))
    }
}

/// A 8KB memory bank
#[derive(Clone)]
pub struct Memory8Kb(Memory8KbInner);

impl Memory8Kb {
    pub fn new() -> Self {
        Memory8Kb(Box::new([0; 8 * KB]))
    }
}

impl Memory for Memory8Kb {
    fn read(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write(&mut self, address: Address, value: Word) {
        unimplemented!()
    }
}

/// A 16KB memory bank
#[derive(Clone)]
pub struct Memory16Kb(Memory16KbInner);

impl Memory16Kb {
    pub fn new() -> Self {
        Memory16Kb(Box::new([0; 16 * KB]))
    }
}

impl Memory for Memory16Kb {
    fn read(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write(&mut self, address: Address, value: Word) {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct Memory32Kb(Memory32KbInner);

impl Memory32Kb {
    pub fn new() -> Self {
        Memory32Kb(Box::new([0; 32 * KB]))
    }
}

impl Memory for Memory32Kb {
    fn read(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write(&mut self, address: Address, value: Word) {
        unimplemented!()
    }
}