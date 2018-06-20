// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Picture processing unit type

use super::mmu::Swram;
use super::Mmu;
use hardware::bios::Bios;

/// A Gameboy picture processing unit
#[derive(Debug, Clone, Copy)]
pub struct Ppu;

impl Ppu {
    /// Emulate the function of a `PPU` over a given number of cycles
    pub fn emulate<S: Swram>(&mut self, cycles: usize, mmu: &mut Mmu<S>) {
        debug!("PPU not yet implemented")
        // TODO: implement PPU::emulate
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Ppu {}
    }
}