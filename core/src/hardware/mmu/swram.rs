// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Switchable work ram types and trait

use hardware::memory::{Memory, Memory4Kb, Switchable};
use isa::{Address, Word};
use std::fmt;

const SWRAM_MAX_ADDRESS: Address = 0xDFFF - 0xD000;

/// Marker trait to signify memory can be used as switchable work ram
pub trait Swram: Switchable {}

/// Fixed work ram
#[derive(Clone)]
pub struct Fixed(Memory4Kb);

impl Fixed {
    /// Create a new fixed work ram
    pub fn new() -> Self {
        Fixed(Memory4Kb::new())
    }
}

impl Memory for Fixed {
    fn read(&self, address: Address) -> Word {
        assert!(address < SWRAM_MAX_ADDRESS);
        self.0.read(address)
    }

    fn write(&mut self, address: Address, value: Word) {
        self.0.write(address, value)
    }
}

impl Switchable for Fixed {
    fn switch_bank(&mut self, _: u8) {
        /* nop */
    }
}

impl Swram for Fixed {}

impl fmt::Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: better display
        write!(f, "Fixed WRAM")
    }
}

impl fmt::Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: better debug
        write!(f, "Fixed WRAM")
    }
}

impl Default for Fixed {
    fn default() -> Self {
        Self::new()
    }
}

/// Banked switchable work ram
#[derive(Clone)]
pub struct Banked {
    banks: Vec<Memory4Kb>,
    active_bank: u8,
}

impl Banked {
    const BANK_COUNT: usize = 7;

    /// Create a new banked work RAM
    pub fn new() -> Self {
        Banked {
            banks: vec![Memory4Kb::new(); Banked::BANK_COUNT],
            active_bank: 0,
        }
    }
}

impl Memory for Banked {
    fn read(&self, address: Address) -> Word {
        assert!(address < SWRAM_MAX_ADDRESS);
        self.banks[self.active_bank as usize].read(address)
    }

    fn write(&mut self, address: Address, value: Word) {
        assert!(address < SWRAM_MAX_ADDRESS);
        self.banks[self.active_bank as usize].write(address, value)
    }
}

impl Switchable for Banked {
    fn switch_bank(&mut self, bank_idx: u8) {
        assert!((bank_idx as usize) < Self::BANK_COUNT);
        self.active_bank = bank_idx;
    }
}

impl Swram for Banked {}

impl fmt::Display for Banked {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: implement
        unimplemented!()
    }
}

impl fmt::Debug for Banked {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: implement
        unimplemented!()
    }
}

impl Default for Banked {
    fn default() -> Self  {
        Self::new()
    }
}