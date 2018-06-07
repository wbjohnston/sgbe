// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hardware::bios::Bios;
use hardware::mmu::SWRAM;
use hardware::MMU;

#[derive(Debug, Copy, Clone)]
pub struct APU {}

impl APU {
    pub fn new() -> Self {
        APU {}
    }

    pub fn emulate<S: SWRAM, B: Bios>(&mut self, mmu: &mut MMU<S, B>, cycles: usize) {
        unimplemented!()
    }
}
