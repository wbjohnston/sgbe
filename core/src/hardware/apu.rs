// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hardware::bios::Bios;
use hardware::mmu::Swram;
use hardware::MMU;

/// A gameboy APU
#[derive(Debug, Copy, Clone)]
pub struct APU {
    // TODO: implement the fields of the APU
}

impl APU {
    /// Create a new `APU`
    pub fn new() -> Self {
        APU {}
    }

    /// Emulate the function of an `APU` over a specified number of cycles
    pub fn emulate<S: Swram, B: Bios>(&mut self, cycles: usize, mmu: &mut MMU<S, B>) {
        debug!("APU emulation not yet implemented")
    }
}
