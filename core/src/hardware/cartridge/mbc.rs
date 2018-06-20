// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Memory bank controller types

use std::fmt::{self, Debug};

use failure::Error;
use hardware::memory::{Memory, Memory32Kb, Switchable};
use isa::{Address, Word};

/// Memory bank controller
#[derive(Clone)]
pub enum Mbc {
    HuC1 {
        // TODO: add HuC1 fields
    },
    Mbc1 {
        // TODO: add MBC1 fields
    },
    Mbc2 {
        // TODO: add MBC2 fields
    },
    Mbc3 {
        // TODO: add MBC3 fields
    },
    Mbc5 {
        // TODO: add MBC5 fields
    },
}

impl Mbc {
    pub fn try_parse_bytes_huc1(bytes: &[u8]) -> Result<Self, Error> {
        fn validate(bytes: &[u8]) -> Result<(), Error> {
            // TODO: implement fixed validation
            Ok(())
        }

        validate(bytes)?;

        // TODO: implement parsing for huc1 cartridges
        unimplemented!()
    }

    fn read_huc1(&self, address: Address) -> Word {
        // TODO:  implement me
        unimplemented!()
    }

    fn write_huc1(&mut self, address: Address, value: Word) {
        // TODO:  implement me
        unimplemented!()
    }

    fn switch_bank_huc1(&mut self, bank_idx: u8) {
        unimplemented!()
    }

    pub fn try_parse_bytes_mbc1(bytes: &[u8]) -> Result<Self, Error> {
        fn validate(bytes: &[u8]) -> Result<(), Error> {
            // TODO: implement fixed validation
            Ok(())
        }

        validate(bytes)?;

        // TODO: implement me
        unimplemented!()
    }

    fn read_mbc1(&self, address: Address) -> Word {
        // TODO:  implement me
        unimplemented!()
    }

    fn write_mbc1(&mut self, address: Address, value: Word) {
        // TODO:  implement me
        unimplemented!()
    }

    fn switch_bank_mbc1(&mut self, bank_idx: u8) {
        unimplemented!()
    }

    pub fn try_parse_bytes_mbc2(bytes: &[u8]) -> Result<Self, Error> {
        fn validate(bytes: &[u8]) -> Result<(), Error> {
            // TODO: implement fixed validation
            Ok(())
        }

        validate(bytes)?;

        // TODO: implement me
        unimplemented!()
    }

    fn read_mbc2(&self, address: Address) -> Word {
        // TODO:  implement me
        unimplemented!()
    }

    fn write_mbc2(&mut self, address: Address, value: Word) {
        // TODO:  implement me
        unimplemented!()
    }

    fn switch_bank_mbc2(&mut self, bank_idx: u8) {
        unimplemented!()
    }

    pub fn try_parse_bytes_mbc3(bytes: &[u8]) -> Result<Self, Error> {
        fn validate(bytes: &[u8]) -> Result<(), Error> {
            // TODO: implement fixed validation
            Ok(())
        }

        validate(bytes)?;

        // TODO: validate me
        unimplemented!()
    }

    fn read_mbc3(&self, address: Address) -> Word {
        // TODO:  implement me
        unimplemented!()
    }

    fn write_mbc3(&mut self, address: Address, value: Word) {
        // TODO:  implement me
        unimplemented!()
    }

    fn switch_bank_mbc3(&mut self, bank_idx: u8) {
        unimplemented!()
    }

    pub fn try_parse_bytes_mbc5(bytes: &[u8]) -> Result<Self, Error> {
        fn validate(bytes: &[u8]) -> Result<(), Error> {
            // TODO: implement fixed validation
            Ok(())
        }

        validate(bytes)?;

        // TODO: implement me
        unimplemented!()
    }

    fn read_mbc5(&self, address: Address) -> Word {
        // TODO:  implement me
        unimplemented!()
    }

    fn write_mbc5(&mut self, address: Address, value: Word) {
        // TODO:  implement me
        unimplemented!()
    }

    fn switch_bank_mbc5(&mut self, bank_idx: u8) {
        // TODO:  implement me
        unimplemented!()
    }
}

impl Memory for Mbc {
    fn read(&self, address: Address) -> Word {
        use self::Mbc::*;
        match *self {
            HuC1 { .. } => self.read_huc1(address),
            Mbc1 { .. } => self.read_mbc1(address),
            Mbc2 { .. } => self.read_mbc2(address),
            Mbc3 { .. } => self.read_mbc3(address),
            Mbc5 { .. } => self.read_mbc5(address),
        }
    }

    fn write(&mut self, address: Address, value: Word) {
        use self::Mbc::*;
        match *self {
            HuC1 { .. } => self.write_huc1(address, value),
            Mbc1 { .. } => self.write_mbc1(address, value),
            Mbc2 { .. } => self.write_mbc2(address, value),
            Mbc3 { .. } => self.write_mbc3(address, value),
            Mbc5 { .. } => self.write_mbc5(address, value),
        }
    }
}

impl Switchable for Mbc {
    fn switch_bank(&mut self, bank_idx: u8) {
        use self::Mbc::*;
        match *self {
            HuC1 { .. } => self.switch_bank_huc1(bank_idx),
            Mbc1 { .. } => self.switch_bank_mbc1(bank_idx),
            Mbc2 { .. } => self.switch_bank_mbc2(bank_idx),
            Mbc3 { .. } => self.switch_bank_mbc3(bank_idx),
            Mbc5 { .. } => self.switch_bank_mbc5(bank_idx),
        }
    }
}

impl Debug for Mbc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: implement debug printing for MBC
        unimplemented!()
    }
}
