// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy bioses

use hardware::memory::Memory;
use isa::{Address, Word};
use std::fmt::{self, Debug};

const GB_BIOS_SIZE: usize = 0x100;
const CGB_BIOS_SIZE: usize = 0x900;

type GbBiosInner = [Word; GB_BIOS_SIZE];

type CgbBiosInner = [Word; CGB_BIOS_SIZE];

/// A Gameboy bios marker
pub trait Bios: Memory {}

/// A Gameboy bios
#[derive(Clone, Copy)]
pub struct GbBios(GbBiosInner);

impl Memory for GbBios {
    fn read(&self, address: Address) -> Word {
        assert!(address < GB_BIOS_SIZE as Address);
        self.0[address as usize]
    }

    fn write(&mut self, address: Address, value: Word) {
        self.0[address as usize] = value;
    }
}

impl Bios for GbBios {}

impl From<GbBiosInner> for GbBios {
    fn from(bytes: GbBiosInner) -> Self {
        GbBios(bytes)
    }
}

impl Debug for GbBios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: implement GbBios::default
        write!(f, "GbBios(..)")
    }
}

/// A Gameboy color bios
#[derive(Clone, Copy)]
pub struct CgbBios(CgbBiosInner);

impl Memory for CgbBios {
    fn read(&self, address: Address) -> Word {
        assert!(address < CGB_BIOS_SIZE as Address);
        self.0[address as usize]
    }

    fn write(&mut self, _address: Address, _value: Word) {
        /* nop */
    }
}

impl Bios for CgbBios {}

impl From<CgbBiosInner> for CgbBios {
    fn from(bytes: CgbBiosInner) -> Self {
        CgbBios(bytes)
    }
}

impl Debug for CgbBios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: implement CgbBios::debug
        write!(f, "CgbBios(..)")
    }
}
