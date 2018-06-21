// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Memory structures and address info

pub mod addresses;

use std::fmt;
use hardware::{pack_words, split_doubleword};
use isa::{Address, DoubleWord, Word};

/// Number of bytes in one Kilobyte
const KB: usize = 1024;

type Memory4KbInner = Box<[Word; KB * 4]>;

type Memory8KbInner = Box<[Word; KB * 8]>;

type Memory16KbInner = Box<[Word; KB * 16]>;

type Memory32KbInner = Box<[Word; KB * 32]>;

/// A piece of memory to read and write data
pub trait Memory {
    /// Read a value from an address
    fn read(&self, address: Address) -> Word;

    /// Write a value to an address
    fn write(&mut self, address: Address, value: Word);

    /// Write a double to an address
    fn write_double(&mut self, address: Address, value: DoubleWord) {
        let (lo, hi) = split_doubleword(value);
        self.write(address, lo);
        self.write(address, hi);
    }

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
        self.0[address as usize]
    }

    fn write(&mut self, address: Address, value: Word) {
        self.0[address as usize] = value;
    }
}

impl Memory4Kb {
    pub fn new() -> Self {
        Memory4Kb(Box::new([0; 4 * KB]))
    }
}

impl Default for Memory4Kb {
    fn default() -> Self {
        Memory4Kb(Box::new([0; 4 * KB]))
    }
}

impl fmt::Debug for Memory4Kb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: better debug pritning for Memory8Kb
        write!(f, "Memory4Kb(..)")
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
        self.0[address as usize]
    }

    fn write(&mut self, address: Address, value: Word) {
        self.0[address as usize] = value;
    }
}

impl Default for Memory8Kb {
    fn default() -> Self {
        Memory8Kb(Box::new([0; 8 * KB]))
    }
}

impl fmt::Debug for Memory8Kb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: better debug pritning for Memory8Kb
        write!(f, "Memory8Kb(..)")
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
        self.0[address as usize]
    }

    fn write(&mut self, address: Address, value: Word) {
        self.0[address as usize] = value;
    }
}

impl Default for Memory16Kb {
    fn default() -> Self {
        Memory16Kb(Box::new([0; 16 * KB]))
    }
}

impl fmt::Debug for Memory16Kb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: better debug print for Memory16Kb
        write!(f, "Memory16Kb(..)")
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
        self.0[address as usize]
    }

    fn write(&mut self, address: Address, value: Word) {
        self.0[address as usize] = value;
    }
}

impl From<[u8; KB * 32]> for Memory32Kb {
    fn from(bytes: [u8; KB * 32]) -> Self {
        Memory32Kb(Box::new(bytes))
    }
}

impl Default for Memory32Kb {
    fn default() -> Self {
        Memory32Kb(Box::new([0; 32 * KB]))
    }
}

impl fmt::Debug for Memory32Kb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: better debug print for Memory32Kb
        write!(f, "Memory32Kb(..)")
    }
}