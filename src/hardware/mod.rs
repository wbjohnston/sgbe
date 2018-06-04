// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy hardware components (excludes CPU)

use isa::types::{Address, Word};
use traits::Bus;

pub mod gpu;

pub mod mmu;

pub mod timer;

pub mod irq;

pub mod apu;

pub mod cartridge;

/// Gameboy Hardware
#[derive(Debug, Clone, Copy)]
pub struct Hardware {}

impl Bus for Hardware {
    fn read(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write(&mut self, address: Address, value: Word) {
        unimplemented!()
    }
}
