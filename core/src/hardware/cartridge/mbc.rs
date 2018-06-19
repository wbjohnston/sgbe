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
pub enum MBC {
    // TODO: probably move rom0 to the cartridge iteself since it's not _actually_
    // managed by the MBC
    Fixed {
        rom0: Memory32Kb,
    },
    HuC1 {
        rom0: Memory32Kb, // TODO: add HuC1 fields
    },
    MBC1 {
        rom0: Memory32Kb, // TODO: add MBC1 fields
    },
    MBC2 {
        rom0: Memory32Kb, // TODO: add MBC2 fields
    },
    MBC3 {
        rom0: Memory32Kb, // TODO: add MBC3 fields
    },
    MBC5 {
        rom0: Memory32Kb, // TODO: add MBC5 fields
    },
}

impl MBC {
    pub fn try_parse_bytes_fixed(bytes: &[u8]) -> Result<Self, Error> {
        fn validate(bytes: &[u8]) -> Result<(), Error> {
            // TODO: implement fixed validation
            Ok(())
        }

        validate(bytes)?;

        Ok(MBC::Fixed {
            rom0: Self::try_parse_rom0(bytes)?,
        })
    }

    fn read_fixed(&self, address: Address) -> Word {
        match *self {
            MBC::Fixed { ref rom0 } => rom0.read(address),
            _ => unreachable!(),
        }
    }

    fn write_fixed(&mut self, address: Address, value: Word) {
        match *self {
            MBC::Fixed { ref mut rom0 } => rom0.write(address, value),
            _ => unreachable!(),
        }
    }

    fn switch_bank_fixed(&mut self, _bank_idx: u8) {
        /* nop */
    }

    pub fn try_parse_bytes_huc1(bytes: &[u8]) -> Result<Self, Error> {
        fn validate(bytes: &[u8]) -> Result<(), Error> {
            // TODO: implement fixed validation
            Ok(())
        }

        validate(bytes)?;

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

    fn try_parse_rom0(bytes: &[u8]) -> Result<Memory32Kb, Error> {
        // FIXME: bad alloc, maybe use cow?
        let mut inner = [0; 1024 * 32];
        inner.copy_from_slice(&bytes[0..1024 * 32]);
        Ok(Memory32Kb::from(inner))
    }
}

impl Memory for MBC {
    fn read(&self, address: Address) -> Word {
        use self::MBC::*;
        match *self {
            Fixed { .. } => self.read_fixed(address),
            HuC1 { .. } => self.read_huc1(address),
            MBC1 { .. } => self.read_mbc1(address),
            MBC2 { .. } => self.read_mbc2(address),
            MBC3 { .. } => self.read_mbc3(address),
            MBC5 { .. } => self.read_mbc5(address),
        }
    }

    fn write(&mut self, address: Address, value: Word) {
        use self::MBC::*;
        match *self {
            Fixed { .. } => self.write_fixed(address, value),
            HuC1 { .. } => self.write_huc1(address, value),
            MBC1 { .. } => self.write_mbc1(address, value),
            MBC2 { .. } => self.write_mbc2(address, value),
            MBC3 { .. } => self.write_mbc3(address, value),
            MBC5 { .. } => self.write_mbc5(address, value),
        }
    }
}

impl Switchable for MBC {
    fn switch_bank(&mut self, bank_idx: u8) {
        use self::MBC::*;
        match *self {
            Fixed { .. } => self.switch_bank_fixed(bank_idx),
            HuC1 { .. } => self.switch_bank_huc1(bank_idx),
            MBC1 { .. } => self.switch_bank_mbc1(bank_idx),
            MBC2 { .. } => self.switch_bank_mbc2(bank_idx),
            MBC3 { .. } => self.switch_bank_mbc3(bank_idx),
            MBC5 { .. } => self.switch_bank_mbc5(bank_idx),
        }
    }
}

impl Debug for MBC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
