// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Picture processing unit stuff

use super::mmu::SWRAM;
use super::MMU;
use hardware::bios::Bios;

/// A Gameboy pictue processes unit
#[derive(Debug, Clone, Copy)]
pub struct PPU;

impl PPU {
    /// Create a new pixel processing unit
    pub fn new() -> PPU {
        PPU {}
    }

    pub fn emulate<S: SWRAM, B: Bios>(&mut self, cycles: usize, mmu: &mut MMU<S, B>) {
        debug!("GPU not yet implemented")
        // unimplemented!()
    }
}
