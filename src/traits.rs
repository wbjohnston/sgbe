// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use isa::types::{Address, DoubleWord, Word};
use isa::util::pack_words;

pub trait Bus {
    /// Read a value from an address
    fn read(&self, address: Address) -> Word;

    /// Read a `DoubleWord`
    fn read_double(&self, address: Address) -> DoubleWord {
        let lo = self.read(address);
        let hi = self.read(address + 1);

        pack_words(lo, hi)
    }

    /// Write a value to an address
    fn write(&mut self, address: Address, value: Word);
}
