// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Memory addresses for the memory map

use isa::Address;

pub const BIOS_OFFSET: Address = 0x0000;
pub const BIOS_END: Address = 0x0100;

/// Rom
pub const ROM0_OFFSET: Address = 0x0100;
pub const ROM0_END: Address = 0x3FFF;

// Switchable rom
pub const SROM_OFFSET: Address = 0x4000;
pub const SROM_END: Address = 0x7FFF;

// Video ram addresses
pub const VRAM_OFFSET: Address = 0x8000;
pub const VRAM_END: Address = 0x9FFF;

// External Ram addresses
pub const ERAM_OFFSET: Address = 0xA000;
pub const ERAM_END: Address = 0xBFFF;

/// Working ram addresses
pub const WRAM_OFFSET: Address = 0xC000;
pub const WRAM_END: Address = 0xCFFF;

// Switchable rom bank addresses
pub const SWRAM_OFFSET: Address = 0xD000;
pub const SWRAM_END: Address = 0xDFFF;

pub const ECHO_RAM_OFFSET: Address = 0xE000;
pub const ECHO_RAM_END: Address = 0xFDFF;

/// Object attribute memory (OAM) addresses
pub const OAM_OFFSET: Address = 0xFE00;
pub const OAM_END: Address = 0xFE9F;
pub const OAM_SIZE: usize = (OAM_END - OAM_OFFSET + 1) as usize;

pub const UNUSABLE_MEMORY_OFFSET: Address = 0xFEA0;
pub const UNUSABLE_MEMORY_END: Address = 0xFEFF;

// IO memory addresses
pub const IOM_OFFSET: Address = 0xFF00;
pub const IOM_END: Address = 0xFF7F;
pub const IOM_SIZE: usize = (IOM_END - IOM_OFFSET + 1) as usize;

// High ram addresses
pub const HRAM_OFFSET: Address = 0xFF80;
pub const HRAM_END: Address = 0x0FFFF;
pub const HRAM_SIZE: usize = (HRAM_END - HRAM_OFFSET + 1) as usize;
