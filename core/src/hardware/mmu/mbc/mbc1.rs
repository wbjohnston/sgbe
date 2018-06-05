// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! MBC1 memory bank controller

use super::MBC;
use isa::types::{Address, Word};
use traits::Memory;

/// MBC1 Memory bank controller
#[derive(Debug, Clone, Copy)]
pub struct MBC1;

impl Memory for MBC1 {
    fn read(&self, address: Address) -> Word {
        // TODO: (will) implement me
        unimplemented!()
    }

    fn write(&mut self, address: Address, value: Word) {
        // TODO: (will) implement me
        unimplemented!()
    }
}

impl MBC for MBC1 {}
