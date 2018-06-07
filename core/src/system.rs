// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy systems

use enumset::EnumSet;
use hardware::bios::{Bios, CgbBios, GbBios};

use hardware::cartridge::Cartridge;
use hardware::cpu::Registers;
use hardware::memory::{Memory4Kb, Memory8Kb};
use hardware::mmu::swram::{self, SWRAM};
use hardware::{APU, CPU, GPU, MMU};
use isa::Word;

/// Gameboy
pub type Gb = System<swram::Fixed, GbBios>;

/// Gameboy color
pub type Cgb = System<swram::Banked, CgbBios>;

/// Gameboy Keys state
pub type Buttons = EnumSet<Button>;

/// State of the Dpad
// #[derive(Debug, Copy, Clone)]
enum_set_type! {
    pub enum Button {
        A,
        B,
        Start,
        Select,
        Left,
        Right,
        Up,
        Down,
    }
}

/// A Gameboy sytem
pub struct System<S: SWRAM, B: Bios> {
    input: Buttons,
    cpu: CPU,
    mmu: MMU<S, B>,
    gpu: GPU,
    apu: APU,
}

impl<S: SWRAM + Default, B: Bios> System<S, B> {
    /// Create a new system with no loaded catridge
    pub fn new(bios: B) -> Self {
        System {
            input: Buttons::empty(),
            cpu: CPU::new(),
            mmu: MMU::new(bios),
            gpu: GPU::new(),
            apu: APU::new(),
        }
    }
}

impl<S: SWRAM, B: Bios> System<S, B> {
    /// Load a catridge into the system
    pub fn load(&mut self, cartridge: Cartridge) {
        self.mmu.load(cartridge)
    }

    /// Unload the current cartridge from the sytem
    pub fn unload(&mut self) {
        self.mmu.unload()
    }

    /// Step the sytem forward on instruction execution
    pub fn step(&mut self) -> u8 {
        self.mmu.update_input_registers(self.input); // update input state

        let cycles_in_step = self.cpu.step(&mut self.mmu);

        self.gpu.emulate(cycles_in_step as usize, &mut self.mmu);
        cycles_in_step
    }

    /// Emulate the system taking a specified number of steps
    pub fn emulate(&mut self, cycles: usize) {
        let mut cycles = cycles; // TODO: (will) what happens when we get cycles not a multiple of 4?
        while cycles > 0 {
            cycles -= self.step() as usize;
        }
    }

    /// Set the input state the next cycle will read from, then return the
    /// input that was passed in
    pub fn set_input(&mut self, buttons: Buttons) -> Buttons {
        self.input = buttons;
        buttons
    }

    /// Return current input state
    pub fn input(&self) -> Buttons {
        self.input
    }

    /// Return the sytem video ram
    pub fn vram(&self) -> &Memory8Kb {
        self.mmu.vram()
    }

    /// Return the registers of the CPU
    pub fn registers(&self) -> &Registers {
        self.cpu.registers()
    }
}
