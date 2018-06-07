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

const SWRAM_MAX_ADDRESS: Address = 0xDFFF - 0xD000;

/// Marker trait to signify memory can be used as switchable work ram
pub trait SWRAM: Switchable {}

/// Fixed work ram
#[derive(Clone)]
pub struct Fixed(Memory4Kb);

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

impl SWRAM for Fixed {}

impl Default for Fixed {
    fn default() -> Self {
        Fixed(Memory4Kb::new())
    }
}

/// Banked switchable work ram
#[derive(Clone)]
pub struct Banked {
    banks: [Memory4Kb; Banked::BANK_COUNT],
    active_bank: u8,
}

impl Banked {
    const BANK_COUNT: usize = 7;

    pub fn switch_bank(&mut self, bank_idx: u8) {
        assert!((bank_idx as usize) < Self::BANK_COUNT);
        self.active_bank = bank_idx;
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

impl SWRAM for Banked {}

impl Default for Banked {
    fn default() -> Self {
        unimplemented!()
    }
}
