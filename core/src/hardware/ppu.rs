// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Picture processing unit type

use super::mmu::Swram;
use super::MMU;
use hardware::bios::Bios;

/// A Gameboy picture processing unit
#[derive(Debug, Clone, Copy)]
pub struct PPU;

impl PPU {
    /// Create a new `PPU`
    pub fn new() -> PPU {
        PPU {}
    }

    /// Emulate the function of a `PPU` over a given number of cycles
    pub fn emulate<S: Swram, B: Bios>(&mut self, cycles: usize, mmu: &mut MMU<S, B>) {
        debug!("GPU not yet implemented")
        // TODO: implement PPU::emulate
    }
}
