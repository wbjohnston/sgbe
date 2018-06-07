// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Cartridge header types

use isa::Word;

/// Types of cartridges
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
    MBC5RumbleRamBattery,
    MBC6,
    MBC7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
}

impl From<Word> for CartridgeKind {
    fn from(value: Word) -> Self {
        use self::CartridgeKind::*;
        match value {
            0x00 => RomOnly,
            0x01 => MBC1,
            0x02 => MBC1Ram,
            0x03 => MBC1RamBattery,
            0x05 => MBC2,
            0x06 => MBC2Battery,
            0x08 => RomRam,
            0x09 => RomRamBattery,
            0x0B => MMM01,
            0x0C => MMM01Ram,
            0x0D => MMM01RamBattery,
            0x0F => MBC3TimerBatery,
            0x10 => MBC3TimerRamBattery,
            0x11 => MBC3,
            0x12 => MBC3Ram,
            0x13 => MBC3RamBattery,
            0x19 => MBC5,
            0x1A => MBC5Ram,
            0x1B => MBC5RamBattery,
            0x1C => MBC5Rumble,
            0x1D => MBC5RumbleRam,
            0x1E => MBC5RumbleRamBattery,
            0x20 => MBC7SensorRumbleRamBattery,
            0xFC => PocketCamera,
            0xFD => BandaiTama5,
            0xFE => HuC3,
            0xFF => HuC1RamBattery,
            _ => unimplemented!(),
        }
    }
}

/// Size of catrdige Rom
#[derive(Debug, Clone, Copy)]
pub enum RomSize {
    Kb32,
    Kb64,
    Kb128,
    Kb256,
    Kb512,
    Mb1,
    Mb2,
    Mb4,
    Mb8,
    Mb1_1,
    Mb1_2,
    Mb1_5,
}

impl From<Word> for RomSize {
    fn from(value: Word) -> Self {
        use self::RomSize::*;
        match value {
            0x00 => Kb32,
            0x01 => Kb64,
            0x02 => Kb128,
            0x03 => Kb256,
            0x04 => Kb512,
            0x05 => Mb1,
            0x06 => Mb2,
            0x07 => Mb4,
            0x08 => Mb8,
            0x52 => Mb1_1,
            0x53 => Mb1_2,
            0x54 => Mb1_5,
            _ => unimplemented!(),
        }
    }
}

/// Catrdige RAM size
#[derive(Debug, Clone, Copy)]
pub enum RamSize {
    None,
    Kb2,
    Kb8,
    Kb32,
    Kb128,
    Kb64,
}

impl From<Word> for RamSize {
    fn from(value: Word) -> Self {
        use self::RamSize::*;
        match value {
            0x00 => RamSize::None,
            0x01 => Kb2,
            0x02 => Kb8,
            0x03 => Kb32,
            0x04 => Kb128,
            0x05 => Kb64,
            _ => unimplemented!(),
        }
    }
}

/// Catrdige destination locale
#[derive(Debug, Clone, Copy)]
pub enum Destination {
    Japan,
    NotJapan,
}

impl From<Word> for Destination {
    fn from(value: Word) -> Self {
        use self::Destination::*;
        match value {
            0x00 => Japan,
            0x01 => NotJapan,
            _ => unimplemented!(),
        }
    }
}
