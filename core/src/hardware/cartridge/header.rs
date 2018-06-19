// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy cartridge header types

use isa::{Address, Word};

/// Start of game entry point multi-byte header field
pub const ENTRY_POINT_OFFSET: Address = 0x100;

/// End of game entry point multi-byte header field
pub const ENTRY_POINT_END: Address = 0x103;

/// Start of nintendo logo bitmap multi-byte header field
pub const NINTENDO_LOGO_OFFSET: Address = 0x104;

/// End of nintendo logo bitmap multi-byte header field
pub const NINTENDO_LOGO_END: Address = 0x133;

/// Start of title multi-byte header field
pub const TITLE_OFFSET: Address = 0x134;
/// End of title multi-byte header field
pub const TITLE_END: Address = 0x143;

/// Color gameboy header flag address
pub const CGB_FLAG_ADDRESS: Address = 0x143;

/// Start of new licensee code multi-byte header field
pub const NEW_LICENSEE_CODE_OFFSET: Address = 0x144;

/// End of new licensee code multi-byte header field
pub const NEW_LICENSEE_CODE_END: Address = 0x145;

/// Super gameboy feature header flag address
pub const SGB_FLAG_ADDRESS: Address = 0x146;

/// Cartridge type header flag address
pub const CATRIDGE_TYPE_ADDRESS: Address = 0x147;

/// Rom size header flag address
pub const ROM_SIZE_ADDRESS: Address = 0x148;

/// Ram size header flag address
pub const RAM_SIZE_ADDRESS: Address = 0x149;

/// Desitation code header flag address
pub const DESTINATION_CODE_ADDRESS: Address = 0x14A;

/// Old licensee header flag address
pub const OLD_LICENSEE_CODE_ADDRESS: Address = 0x14B;

/// Version number of the game header field
pub const MASK_ROM_VERSION_ADDRESS: Address = 0x14C;

/// Checksum to verify the header header field
pub const HEADER_CHECKSUM_ADDRESS: Address = 0x14D;

/// Start of checksum to verify the contents of the cartridge multi-byte field
pub const GLOBAL_CHECKSUM_OFFSET: Address = 0x14E;

/// End of checksum to verify the contents of the cartridge multi-byte field
pub const GLOBAL_CHECKSUM_END: Address = 0x14F;

/// A cartridge kind
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
    MBC3TimerBattery,
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

// TODO: change this to try_from once the `TryFrom` trait is stabilized
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
            0x0F => MBC3TimerBattery,
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
            _ => unreachable!(),
        }
    }
}

impl CartridgeKind {
    /// Returns true if the cartridge kind has a battery
    pub fn has_battery(&self) -> bool {
        use self::CartridgeKind::*;
        match *self {
            MBC1RamBattery
            | MBC2Battery
            | MBC3TimerBattery
            | MBC3TimerRamBattery
            | MBC5RamBattery
            | MBC5RumbleRamBattery
            | MBC7SensorRumbleRamBattery
            | HuC1RamBattery => true,
            _ => false,
        }
    }

    /// Returns true if the cartridge kind has a timer
    pub fn has_timer(&self) -> bool {
        use self::CartridgeKind::*;
        match *self {
            MBC3TimerBattery | MBC3TimerRamBattery => true,
            _ => false,
        }
    }
}

/// A cartridge rom size flag value
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

// TODO: change this to try_from once the `TryFrom` trait is stabilized
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
            _ => unreachable!(),
        }
    }
}

/// A cartridge RAM size flag value
#[derive(Debug, Clone, Copy)]
pub enum RamSize {
    None,
    Kb2,
    Kb8,
    Kb32,
    Kb128,
    Kb64,
}

// TODO: change this to try_from once the `TryFrom` trait is stabilized
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
            _ => unreachable!(),
        }
    }
}

/// A cartridge desintation flag value
#[derive(Debug, Clone, Copy)]
pub enum Destination {
    Japan,
    NotJapan,
}

// TODO: change this to try_from once the `TryFrom` trait is stabilized
impl From<Word> for Destination {
    fn from(value: Word) -> Self {
        use self::Destination::*;
        match value {
            0x00 => Japan,
            0x01 => NotJapan,
            _ => unreachable!(),
        }
    }
}
