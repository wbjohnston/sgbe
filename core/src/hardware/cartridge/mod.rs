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
use hardware::Timer;
use isa::{Address, Word};

const CATRIDGE_TYPE_ADDR: usize = 0x147;

/// A gameboy cartridge
#[derive(Debug, Clone)]
pub struct Cartridge {
    mbc: MBC,
    has_battery: bool,
    timer: Option<Timer>,
}

impl Cartridge {
    pub fn try_parse_bytes(bytes: &[u8]) -> Result<Self, Error> {
        use self::CartridgeKind::*;
        let catridge_type = bytes[CATRIDGE_TYPE_ADDR].into();

        let mbc = match catridge_type {
            RomOnly => MBC::try_parse_bytes_fixed(bytes),
            MBC1 | MBC1Ram | MBC1RamBattery => MBC::try_parse_bytes_mbc1(bytes),
            MBC2 | MBC2Battery => MBC::try_parse_bytes_mbc2(bytes),
            MBC3 | MBC3Ram | MBC3RamBattery | MBC3TimerBattery | MBC3TimerRamBattery => {
                MBC::try_parse_bytes_mbc3(bytes)
            }
            MBC5 | MBC5Ram | MBC5RamBattery | MBC5Rumble | MBC5RumbleRam | MBC5RumbleRamBattery => {
                MBC::try_parse_bytes_mbc5(bytes)
            }
            HuC1RamBattery => MBC::try_parse_bytes_huc1(bytes),
            _ => unimplemented!(),
        }?;

        let has_battery = match catridge_type {
            MBC1RamBattery
            | MBC2Battery
            | MBC3TimerBattery
            | MBC3TimerRamBattery
            | MBC5RamBattery
            | MBC5RumbleRamBattery
            | MBC7SensorRumbleRamBattery
            | HuC1RamBattery => true,
            _ => false,
        };

        let timer = match catridge_type {
            MBC3TimerBattery
            | MBC3TimerRamBattery => Some(Timer::new()),
            _ => None,
        };

        Ok(Cartridge {
            mbc,
            has_battery,
            timer,
        })
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
    ParsingError(ParsingError),
}

#[derive(Fail, Debug, Clone)]
pub enum ParsingError {
    #[fail(display = "Header invalid")]
    InvalidHeader,
}