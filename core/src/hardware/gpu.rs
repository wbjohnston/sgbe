// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::mmu::{MBC, SWRAM};
use super::MMU;
use traits::Emulator;

/// A Gameboy graphics processing unit
#[derive(Debug, Clone, Copy)]
pub struct GPU;

impl GPU {
    pub fn new() -> GPU {
        unimplemented!()
    }

    fn draw<M: MBC, S: SWRAM>(&mut self, mmu: &mut MMU<M, S>) {
        unimplemented!()
    }
}

impl Emulator for GPU {
    fn emulate(&self, cycles: usize) {
        unimplemented!()
    }
}
