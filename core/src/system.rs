// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy systems

use bios::SystemBIOS;

use hardware::cartridge::Cartridge;
use hardware::mmu::swram::{self, SWRAM};
use hardware::{APU, CPU, GPU, MMU};

/// Gameboy
pub type GB = System<swram::Fixed>;

/// Gameboy color
pub type CGB = System<swram::Banked>;

/// A Gameboy sytem
pub struct System<S: SWRAM> {
    bios: SystemBIOS,
    cpu: CPU,
    mmu: MMU<S>,
    gpu: GPU,
}

impl<S: SWRAM> System<S> {
    /// Create a new system with no loaded catridge
    pub fn new(bios: SystemBIOS) -> Self {
        System {
            bios: bios,
            cpu: CPU::new(),
            mmu: MMU::new(),
            gpu: GPU::new(),
        }
    }

    /// Load a catridge into the system
    pub fn load(&mut self, cartridge: Cartridge) {
        self.mmu.load(cartridge)
    }

    /// Unload the current cartridge from the sytem
    pub fn unload(&mut self) {
        self.mmu.unload()
    }
}

impl<S: SWRAM> System<S> {
    pub fn step(&mut self) -> u8 {
        unimplemented!()
    }

    pub fn emulate(&mut self, cycles: usize) {
        // FIXME: (will) this probably isn't right
        self.cpu.emulate(cycles, &mut self.mmu);
        self.gpu.emulate(cycles, &mut self.mmu);
    }
}
