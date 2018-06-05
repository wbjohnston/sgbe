// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::mmu::{SWRAM};
use super::MMU;

/// A Gameboy graphics processing unit
#[derive(Debug, Clone, Copy)]
pub struct GPU;

impl GPU {
    pub fn new() -> GPU {
        unimplemented!()
    }
    
    pub fn emulate<S: SWRAM>(&mut self, cycles: usize, mmu: &mut MMU<S>) {
        unimplemented!()
    }

    fn draw<S: SWRAM>(&mut self, mmu: &mut MMU<S>) {
        unimplemented!()
    }
}