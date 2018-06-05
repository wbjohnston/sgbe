// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use failure::{Error, Fail};

use hardware::mmu::{MBC, SWRAM};
use hardware::mmu::swram;
use hardware::mmu::mbc;
use hardware::MMU;
use isa::types::Word;

#[derive(Fail, Debug, Clone)]
pub enum CartridgeError {
    #[fail(display = "Failed to parse bytes into a valid catridge")]
    ParsingError
}

#[derive(Clone)]
pub struct Cartridge {
    header: Header,
}

impl Cartridge {
    /// Try to create a cartridge from a stream of bytes
    pub fn try_parse_bytes<'a>(bytes: &'a [u8]) -> Result<Self, Error> {
        unimplemented!()
    }

    /// Return a reference to the cartridge header
    pub fn header(&self) -> &Header {
        &self.header
    }
}


// impl Into<MMU<mbc::Fixed, swram::Fixed>> for Cartridge {
//     fn into(self) -> MMU<mbc::Fixed, swram::Fixed> {
//         unimplemented!()
//     }
// }

// impl<M: MBC, S: SWRAM> Into<MMU<M, S>> for Cartridge {
//     fn into(self) -> MMU<M, S> {
//         let x = 10;
//         if x == 10 {
//             MMU::<mbc::Fixed, swram::Fixed>::new();
//         } else {
//             MMU::<mbc::MBC1, swram::Banked>::new()
//         }
//         // unimplemented!()
//     }
// }

#[derive(Debug, Clone, Copy)]
pub enum CartridgeKind {
    RomOnly,
    MBC1,
    MBC1Ram,
    MBC1RamBattery,
    MBC2,
    MBC2Battery,
    RomRam,
    RomRamBattery,
    MMM01,
    MMM01Ram,
    MMM01RamBattery,
    MBC3TimerBatery,
    MBC3TimerRamBattery,
    MBC3,
    MBC3Ram,
    MBC3RamBattery,
    MBC5,
    MBC5Ram,
    MBC5RamBattery,
    MBC5Rumble,
    MBC5RumbleRam,
    MBC5RumbleRameBattery,
    MBC6,
    MBC7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
}

#[derive(Debug, Clone, Copy)]
pub enum RomSize {
    KB32,
    KB64,
    KB128,
    KB256,
    KB512,
    MB1,
    MB2,
    MB4,
    MB8,
    MB1_1,
    MB1_2,
    MB1_5,
}

#[derive(Debug, Clone, Copy)]
pub enum ERAMSize {
    None,
    KB2,
    KB8,
    KB32,
    KB128,
    KB64,
}

#[derive(Debug, Clone, Copy)]
pub enum Destination {
    Japan,
    NotJapan,
}

#[derive(Clone)]
pub struct Header {
    logo: [Word; Header::NINTENDO_LOGO_SIZE],
    kind: CartridgeKind,
    rom_size: RomSize,
    eram_size: ERAMSize,
    destination: Destination,
    header_checksum: u8,
    global_checksum: u16,
}

impl Header {
    const NINTENDO_LOGO_SIZE: usize = 0x30;

    fn try_parse_bytes<'a>(bytes: &'a [u8]) -> Result<Self, Error> {
        unimplemented!()
    }
}