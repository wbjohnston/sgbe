// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod mbc;
pub use self::mbc::MBC;

pub mod header;

use failure::Error;
use hardware::memory::Memory;
use hardware::Timer;
use isa::{Address, Word};

#[derive(Debug, Clone)]
pub struct Cartridge {
    mbc: MBC,
    has_battery: bool,
    timer: Option<Timer>,
}

impl Cartridge {
    pub fn try_parse_bytes(bytes: &[u8]) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl Memory for Cartridge {
    fn read(&self, address: Address) -> Word {
        self.mbc.read(address)
    }

    fn write(&mut self, address: Address, value: Word) {
        self.mbc.write(address, value)
    }
}

#[derive(Fail, Debug, Clone)]
pub enum CartridgeError {
    #[fail(display = "Failed to parse bytes into a valid catridge")]
    ParsingError,
}
