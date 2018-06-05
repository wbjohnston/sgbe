// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod mbc;
pub use self::mbc::MBC;

mod types;

pub mod swram;
pub use self::swram::SWRAM;

use cartridge::Cartridge;
use isa::types::{Address, Word};
use traits::{Memory, Emulator};

/// Rom
const ROM0_OFFSET: Address = 0x0000;
const ROM0_END: Address = 0x3FFF;
const ROM0_SIZE: usize = (ROM0_END - ROM0_OFFSET) as usize;

// Switchable rom
const SROM_OFFSET: Address = 0x4000;
const SROM_END: Address = 0x7FFF;

// Video ram addresses
const VRAM_OFFSET: Address = 0x8000;
const VRAM_END: Address = 0x9FFF;
const VRAM_SIZE: usize = (VRAM_END - VRAM_OFFSET) as usize;

// External Ram addresses
const ERAM_OFFSET: Address = 0xA000;
const ERAM_END: Address = 0xBFFF;
const ERAM_SIZE: usize = (ERAM_END - ERAM_OFFSET) as usize;

/// Working ram addresses
const WRAM_OFFSET: Address = 0xC000;
const WRAM_END: Address = 0xCFFF;
const WRAM_SIZE: usize = (WRAM_END - WRAM_OFFSET) as usize;

// Switchable rom bank addresses
const SWRAM_OFFSET: Address = 0xD000;
const SWRAM_END: Address = 0xDFFF;

const ECHO_RAM_OFFSET: Address = 0xE000;
const ECHO_RAM_END: Address = 0xFDFF;

/// Object attribute memory (OAM) addresses
const OAM_OFFSET: Address = 0xFE00;
const OAM_END: Address = 0xFE9F;
const OAM_SIZE: usize = (OAM_END - OAM_OFFSET) as usize;

const UNUSABLE_MEMORY_OFFSET: Address = 0xFEA0;
const UNUSABLE_MEMORY_END: Address = 0xFEFF;

// IO memory addresses
const IOM_OFFSET: Address = 0xFF00;
const IOM_END: Address = 0xFF7F;
const IOM_SIZE: usize = (IOM_END - IOM_END) as usize;

// High ram addresses
const HRAM_OFFSET: Address = 0xFF80;
const HRAM_END: Address = 0x0FFFE;
const HRAM_SIZE: usize = (HRAM_END - HRAM_OFFSET) as usize;

const INTERRUPT_ENABLE_REGISTER_ADDRESS: Address = 0xFFFF;

/// A Gameboy Memory management unit
#[derive(Clone)]
pub struct MMU<M: MBC, S: SWRAM> {
    rom0: [Word; ROM0_SIZE], // home rom
    srom: M,                 // switchable rom bank
    vram: [Word; VRAM_SIZE], // video ram
    eram: [Word; ERAM_SIZE], // external ram
    wram: [Word; WRAM_SIZE], // work ram
    swram: S,                // switchable work ram
    oam: [Word; OAM_SIZE],   // Object attribute map
    iom: [Word; IOM_SIZE],   // IO memory
    hram: [Word; HRAM_SIZE], // high ram
    ier: Word,               // interrupt enable register
}

impl<M: MBC, S: SWRAM> MMU<M, S> {
    /// Read a word from memory
    fn read_inner(&self, addr: Address) -> Word {
        match addr {
            ROM0_OFFSET...ROM0_END => self.rom0[(addr - ROM0_OFFSET) as usize],
            SROM_OFFSET...SROM_END => self.srom.read(addr - SROM_OFFSET),
            ERAM_OFFSET...ERAM_END => self.eram[(addr - ERAM_OFFSET) as usize],
            VRAM_OFFSET...VRAM_END => self.vram[(addr - VRAM_OFFSET) as usize],
            WRAM_OFFSET...WRAM_END => self.wram[(addr - WRAM_OFFSET) as usize],
            SWRAM_OFFSET...SWRAM_END => self.swram.read(addr - SWRAM_OFFSET),
            ECHO_RAM_OFFSET...ECHO_RAM_END => self.rom0[(addr - ECHO_RAM_OFFSET) as usize],
            OAM_OFFSET...OAM_END => self.oam[(addr - OAM_OFFSET) as usize],
            UNUSABLE_MEMORY_OFFSET...UNUSABLE_MEMORY_END => unreachable!(), // its not usable
            IOM_OFFSET...IOM_END => self.iom[(addr - IOM_OFFSET) as usize],
            HRAM_OFFSET...HRAM_END => self.hram[(addr - HRAM_OFFSET) as usize],
            INTERRUPT_ENABLE_REGISTER_ADDRESS => self.ier,
            v => unreachable!("Tried to read unmmapped value: {}", v),
        }
    }

    /// Write a [`isa::Word`] to memory
    fn write_inner(&mut self, address: Address, value: Word) {
        // TODO: (will) implement me
        match address {
            ROM0_OFFSET...ROM0_END => unimplemented!(),
            SROM_OFFSET...SROM_END => unimplemented!(),
            ERAM_OFFSET...ERAM_END => unimplemented!(),
            VRAM_OFFSET...VRAM_END => unimplemented!(),
            WRAM_OFFSET...WRAM_END => unimplemented!(),
            SWRAM_OFFSET...SWRAM_END => unimplemented!(),
            ECHO_RAM_OFFSET...ECHO_RAM_END => unimplemented!(),
            OAM_OFFSET...OAM_END => unimplemented!(),
            UNUSABLE_MEMORY_OFFSET...UNUSABLE_MEMORY_END => unreachable!(), // its not usable
            IOM_OFFSET...IOM_END => unimplemented!(),
            HRAM_OFFSET...HRAM_END => unimplemented!(),
            INTERRUPT_ENABLE_REGISTER_ADDRESS => self.ier = value,
            _ => unimplemented!()
        }
    }
}

impl<M: MBC, S: SWRAM> Memory for MMU<M, S> {
    fn read(&self, address: Address) -> Word {
        self.read_inner(address)
    }

    fn write(&mut self, address: Address, value: Word) {
        self.write_inner(address, value)
    }
}

impl<M: MBC, S: SWRAM> From<Cartridge> for MMU<M, S> {
    fn from(cartridge: Cartridge) -> Self {
        unimplemented!()
    }
}