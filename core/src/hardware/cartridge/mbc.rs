// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use isa::{Word, Address};
use hardware::Memory;
use failure::Error;

/// Memory bank controller
pub enum MBC {
    Fixed {},
    HuC1 {},
    MBC1 {},
    MBC2 {},
    MBC3 {},
    MBC5 {},
}

impl MBC {
    pub fn try_parse_bytes<'a>(bytes: &'a [u8]) -> Result<Self, Error> {
        unimplemented!()
    }

    fn read_fixed(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write_fixed(&mut self, address: Address, value: Word) {
        unimplemented!()
    }
    
    fn read_huc1(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write_huc1(&mut self, address: Address, value: Word) {
        unimplemented!()
    }
    
    fn read_mbc1(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write_mbc1(&mut self, address: Address, value: Word) {
        unimplemented!()
    }
    
    fn read_mbc2(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write_mbc2(&mut self, address: Address, value: Word) {
        unimplemented!()
    }
    
    fn read_mbc3(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write_mbc3(&mut self, address: Address, value: Word) {
        unimplemented!()
    }

    fn read_mbc5(&self, address: Address) -> Word {
        unimplemented!()
    }

    fn write_mbc5(&mut self, address: Address, value: Word) {
        unimplemented!()
    }
}

impl Memory for MBC {
    fn read(&self, address: Address) -> Word {
        use self::MBC::*;
        match *self {
            Fixed{..} => self.read_fixed(address),
            HuC1{..} => self.read_huc1(address),
            MBC1{..} => self.read_mbc1(address),
            MBC2{..} => self.read_mbc2(address),
            MBC3{..} => self.read_mbc3(address),
            MBC5{..} => self.read_mbc5(address),
        }
    }

    fn write(&mut self, address: Address, value: Word) { 
        use self::MBC::*;
        match *self {
            Fixed{..} => self.write_fixed(address, value),
            HuC1{..} => self.write_huc1(address, value),
            MBC1{..} => self.write_mbc1(address, value),
            MBC2{..} => self.write_mbc2(address, value),
            MBC3{..} => self.write_mbc3(address, value),
            MBC5{..} => self.write_mbc5(address, value),
        }
    }
}