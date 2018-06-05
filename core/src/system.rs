// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy systems

use bios::SystemBIOS;

use hardware::{CPU, MMU, GPU, APU};
use cartridge::Cartridge;
use traits::Emulator;
use hardware::mmu::{SWRAM, MBC};
use hardware::mmu::swram;

/// Gameboy
pub type GB<M: MBC> = System<M, swram::Fixed>;

/// Gameboy color
pub type CGB<M: MBC> = System<M, swram::Banked>;

/// A Gameboy sytem
#[derive(Clone)]
pub struct System<M: MBC, S: SWRAM> {
    bios: SystemBIOS,
    cpu: CPU,
    mmu: MMU<M, S>,
    gpu: GPU,
}

impl<M: MBC + Default> GB<M> {
    pub fn new(bios: SystemBIOS, cartridge: Cartridge) -> GB<M> {
        GB {
            bios: bios,
            cpu: CPU::new(),
            mmu: MMU::from(cartridge),
            gpu: GPU::new(),
        }
    }
}

impl <M: MBC + Default> CGB<M> {
    fn new(bios: SystemBIOS, cartridge: Cartridge) -> CGB<M> {
        CGB {
            bios: bios,
            cpu: CPU::new(),
            mmu: MMU::from(cartridge),
            gpu: GPU::new(),
        }
    }
}

impl<M: MBC, S: SWRAM> System<M, S> {
    pub fn step(&mut self) -> u8 {
        unimplemented!()
    }

    pub fn emulate(&self, cycles: usize) {
        unimplemented!()
    }
}
