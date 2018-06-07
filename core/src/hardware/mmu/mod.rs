// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod reginfo;
use self::reginfo::*;

mod meminfo;
use self::meminfo::*;

pub mod swram;
pub use self::swram::SWRAM;

use super::memory::Memory;
use super::memory::{Memory16Kb, Memory4Kb, Memory8Kb};
use super::Cartridge;
use isa::{Address, Word};

/// A Gameboy Memory management unit
pub struct MMU<S: SWRAM> {
    cartridge: Option<Cartridge>,
    vram: Memory8Kb,         // video ram
    wram: Memory4Kb,         // work ram
    swram: S,                // switchable work ram
    oam: [Word; OAM_SIZE],   // Object attribute map
    iom: [Word; IOM_SIZE],   // IO memory
    hram: [Word; HRAM_SIZE], // high ram
}

impl<S: SWRAM + Default> Default for MMU<S> {
    fn default() -> Self {
        let oam = [0; OAM_SIZE];
        let iom = Self::new_io_memory();
        let hram = [0; HRAM_SIZE];

        Self {
            cartridge: None,
            vram: Memory8Kb::new(),
            wram: Memory4Kb::new(),
            swram: S::default(),
            oam: oam,
            iom: iom,
            hram: hram,
        }
    }
}

impl<S: SWRAM> MMU<S> {
    /// Load a cartridge into the MMU
    pub fn load(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
    }

    /// Unload the current catridge from the MMU
    pub fn unload(&mut self) {
        self.cartridge = None;
    }

    /// Create a new IO memory section for the gameboy
    fn new_io_memory() -> [Word; IOM_SIZE] {
        let mut iom = [0; IOM_SIZE];
        let io_addr = |address| (address - IOM_OFFSET) as usize;

        // initialize correct values for io memory
        iom[io_addr(0xFF05)] = 0x00; // TIMA
        iom[io_addr(0xFF06)] = 0x00; // TMA
        iom[io_addr(0xFF07)] = 0x00; // TAC
        iom[io_addr(0xFF10)] = 0x80; // NR10
        iom[io_addr(0xFF11)] = 0xBF; // NR11
        iom[io_addr(0xFF12)] = 0xF3; // NR12
        iom[io_addr(0xFF14)] = 0xBF; // NR14
        iom[io_addr(0xFF16)] = 0x3F; // NR21
        iom[io_addr(0xFF17)] = 0x00; // NR22
        iom[io_addr(0xFF19)] = 0xBF; // NR24
        iom[io_addr(0xFF1A)] = 0x7F; // NR30
        iom[io_addr(0xFF1B)] = 0xFF; // NR31
        iom[io_addr(0xFF1C)] = 0x9F; // NR32
        iom[io_addr(0xFF1E)] = 0xBF; // NR33
        iom[io_addr(0xFF20)] = 0xFF; // NR41
        iom[io_addr(0xFF21)] = 0x00; // NR42
        iom[io_addr(0xFF22)] = 0x00; // NR43
        iom[io_addr(0xFF23)] = 0xBF; // NR44
        iom[io_addr(0xFF24)] = 0x77; // NR50
        iom[io_addr(0xFF25)] = 0xF3; // NR51
        iom[io_addr(0xFF26)] = 0xF1; // NR52
        iom[io_addr(0xFF40)] = 0x91; // LCDC
        iom[io_addr(0xFF42)] = 0x00; // SCY
        iom[io_addr(0xFF43)] = 0x00; // SCX
        iom[io_addr(0xFF45)] = 0x00; // LYC
        iom[io_addr(0xFF47)] = 0xFC; // BGP
        iom[io_addr(0xFF48)] = 0xFF; // OBGP0
        iom[io_addr(0xFF49)] = 0xFF; // OBGP1
        iom[io_addr(0xFF4A)] = 0x00; // WY
        iom[io_addr(0xFF4B)] = 0x00; // WX

        iom
    }

    /// Read a word from memory
    fn read_inner(&self, address: Address) -> Word {
        match address {
            ROM0_OFFSET...ROM0_END => if let Some(ref cartridge) = self.cartridge {
                cartridge.read(address - ROM0_OFFSET)
            } else {
                Word::default()
            },
            SROM_OFFSET...SROM_END => if let Some(ref cartridge) = self.cartridge {
                cartridge.read(address - SROM_END)
            } else {
                Word::default()
            },
            ERAM_OFFSET...ERAM_END => if let Some(ref cartridge) = self.cartridge {
                cartridge.read(address - ERAM_OFFSET)
            } else {
                Word::default()
            },
            VRAM_OFFSET...VRAM_END => self.vram.read(address - VRAM_OFFSET),
            WRAM_OFFSET...WRAM_END => self.wram.read(address - WRAM_OFFSET),
            SWRAM_OFFSET...SWRAM_END => self.swram.read(address - SWRAM_OFFSET),
            ECHO_RAM_OFFSET...ECHO_RAM_END => if let Some(ref cartridge) = self.cartridge {
                cartridge.read(address - ECHO_RAM_OFFSET)
            } else {
                Word::default()
            },
            OAM_OFFSET...OAM_END => self.oam[(address - OAM_OFFSET) as usize],
            UNUSABLE_MEMORY_OFFSET...UNUSABLE_MEMORY_END => unreachable!(), // its not usable
            IOM_OFFSET...IOM_END => self.iom[(address - IOM_OFFSET) as usize],
            HRAM_OFFSET...HRAM_END => self.hram[(address - HRAM_OFFSET) as usize],
            v => unreachable!("Tried to read unmmapped value: {}", v),
        }
    }

    /// Write a `Word` to memory
    fn write_inner(&mut self, address: Address, value: Word) {
        match address {
            ROM0_OFFSET...ROM0_END => if let Some(ref mut cartridge) = self.cartridge {
                cartridge.write(address - ROM0_OFFSET, value)
            },
            SROM_OFFSET...SROM_END => if let Some(ref mut cartridge) = self.cartridge {
                cartridge.write(address - SROM_END, value)
            },
            ERAM_OFFSET...ERAM_END => if let Some(ref mut cartridge) = self.cartridge {
                cartridge.write(address - ERAM_OFFSET, value)
            },
            VRAM_OFFSET...VRAM_END => self.vram.write(address - VRAM_OFFSET, value),
            WRAM_OFFSET...WRAM_END => self.wram.write(address - WRAM_OFFSET, value),
            SWRAM_OFFSET...SWRAM_END => self.swram.write(address - SWRAM_OFFSET, value),
            ECHO_RAM_OFFSET...ECHO_RAM_END => if let Some(ref mut cartridge) = self.cartridge {
                cartridge.write(address - ECHO_RAM_OFFSET, value)
            },
            OAM_OFFSET...OAM_END => self.oam[(address - OAM_OFFSET) as usize] = value,
            UNUSABLE_MEMORY_OFFSET...UNUSABLE_MEMORY_END => unreachable!(), // its not usable
            IOM_OFFSET...IOM_END => self.iom[(address - IOM_OFFSET) as usize] = value,
            HRAM_OFFSET...HRAM_END => self.hram[(address - HRAM_OFFSET) as usize] = value,
            v => unreachable!("Tried to read unmmapped value: {}", v),
        }
    }
}

impl<S: SWRAM> Memory for MMU<S> {
    fn read(&self, address: Address) -> Word {
        self.read_inner(address)
    }

    fn write(&mut self, address: Address, value: Word) {
        self.write_inner(address, value)
    }
}
