// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy memory management unit (MMU)

use isa::types::{Address, DoubleWord, Word};

/// Working ram size
const WRAM_OFFSET: Address = 0xC000;
const WRAM_END: Address = 0xCFFF;
const WRAM_SIZE: usize = (ZRAM_END - ZRAM_OFFSET + 1) as usize;

/// Zero-page ram
const ZRAM_OFFSET: Address = 0xFF80;
const ZRAM_END: Address = 0xFFFE;
const ZRAM_SIZE: usize = (ZRAM_END - ZRAM_OFFSET + 1) as usize;

// External Ram
const ERAM_OFFSET: Address = 0xFF80;
const ERAM_END: Address = 0xFFFE;
const ERAM_SIZE: usize = (ZRAM_END - ZRAM_OFFSET + 1) as usize;

/// Size of rom
const ROM_OFFSET: Address = 0xFF80;
const ROM_END: Address = 0xFFFE;
const ROM_SIZE: usize = (ZRAM_END - ZRAM_OFFSET + 1) as usize;

const VRAM_OFFSET: Address = 0x8000;
const VRAM_END: Address = 0x9FFF;
const VRAM_SIZE: usize = (VRAM_END - VRAM_OFFSET + 1) as usize;

/// Size of the system bios
const BIOS_SIZE: usize = 0x0000;

const INTERRUPT_ENABLE_REGISTER_ADDRESS: Address = 0xFFFF;

/// A Gameboy Memory management unit
#[derive(Clone)]
pub struct MMU {
    bios: [Word; BIOS_SIZE],
    wram: [Word; WRAM_SIZE],
    eram: [Word; ERAM_SIZE],
    vram: [Word; VRAM_SIZE as usize],
    rom: [Word; ROM_SIZE],
    /// Interrup enable register
    ier: Word,
}

impl MMU {
    /// Create a new MMU
    pub fn new() -> MMU {
        unimplemented!()
    }

    /// Read a [`isa::Word`] from memory
    pub fn read(&self, addr: DoubleWord) -> Word {
        match addr {
            VRAM_OFFSET...VRAM_END => self.vram[(addr - VRAM_OFFSET) as usize],
            INTERRUPT_ENABLE_REGISTER_ADDRESS => {
                self.vram[INTERRUPT_ENABLE_REGISTER_ADDRESS as usize]
            }
            _ => unimplemented!(),
        }
    }

    /// Write a [`isa::Word`] to memory
    pub fn write(&mut self, addr: DoubleWord) {
        unimplemented!()
    }
}
