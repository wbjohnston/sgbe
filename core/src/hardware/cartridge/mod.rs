// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy cartridge types

mod mbc;
pub use self::mbc::MBC;

pub mod header;
use self::header::CartridgeKind;

use failure::Error;
use hardware::memory::Memory;
use hardware::memory::Memory32Kb;
use hardware::Timer;
use isa::{Address, Word};

/// A gameboy cartridge
#[derive(Debug, Clone)]
pub struct Cartridge {
    rom0: Memory32Kb,
    mbc: Option<MBC>,
    has_battery: bool,
    timer: Option<Timer>,
}

impl Cartridge {
    /// Try to parse a byte stream into a catridge, returning an error if any occurred
    pub fn try_parse_bytes(bytes: &[u8]) -> Result<Self, Error> {
        use self::CartridgeKind::*;
        // don't try to parse if a header can't even be read
        Self::validate_header(bytes)?;

        let rom0 = Self::try_parse_rom0(bytes)?;

        // get the cartridge type flag
        let catridge_type = rom0.read(header::CATRIDGE_TYPE_ADDRESS).into();

        let mbc = match catridge_type {
            RomOnly => None,
            MBC1 | MBC1Ram | MBC1RamBattery => Some(MBC::try_parse_bytes_mbc1(bytes)?),
            MBC2 | MBC2Battery => Some(MBC::try_parse_bytes_mbc2(bytes)?),
            MBC3 | MBC3Ram | MBC3RamBattery | MBC3TimerBattery | MBC3TimerRamBattery => {
                Some(MBC::try_parse_bytes_mbc3(bytes)?)
            }
            MBC5 | MBC5Ram | MBC5RamBattery | MBC5Rumble | MBC5RumbleRam | MBC5RumbleRamBattery => {
                Some(MBC::try_parse_bytes_mbc5(bytes)?)
            }
            HuC1RamBattery => Some(MBC::try_parse_bytes_huc1(bytes)?),
            _ => unimplemented!(),
        };

        let has_battery = catridge_type.has_battery();

        // make a timer if its required
        let timer = if catridge_type.has_timer() {
            Some(Timer::new())
        } else {
            None
        };

        Ok(Cartridge {
            rom0,
            mbc,
            has_battery,
            timer,
        })
    }

    /// Returns an `[failure::Error]` if there is a problem parsing the header
    fn validate_header(bytes: &[u8]) -> Result<(), Error> {
        // basic check. make sure there are enough bytes to form a header
        if bytes.len() < 0x14 {
            return Err(ParsingError::InvalidHeaderLength {
                length: bytes.len(),
            }.into());
        }

        Ok(())
    }

    fn try_parse_rom0(bytes: &[u8]) -> Result<Memory32Kb, Error> {
        // FIXME: bad alloc, maybe use cow?
        let mut inner = [0; 1024 * 32]; // 32kb
        inner.copy_from_slice(&bytes[0..1024 * 32]);
        Ok(Memory32Kb::from(inner))
    }
}

impl Memory for Cartridge {
    fn read(&self, address: Address) -> Word {
        if let Some(ref mbc) = self.mbc {
            mbc.read(address)
        } else {
            Word::default()
        }
    }

    fn write(&mut self, address: Address, value: Word) {
        if let Some(ref mut mbc) = self.mbc {
            mbc.write(address, value)
        }
    }
}

/// Errors that can occur during cartridge parsing
#[derive(Fail, Debug, Clone)]
pub enum ParsingError {
    #[fail(display = "Header invalid length {}", length)]
    InvalidHeaderLength { length: usize },
}
