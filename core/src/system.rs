// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Full emulator systems

use enumset::EnumSet;
use hardware::bios::{Bios, CgbBios, GbBios};

use hardware::cartridge::Cartridge;
use hardware::cpu::Registers;
use hardware::memory::Memory8Kb;
use hardware::mmu::swram::{self, Swram};
use hardware::{Apu, Cpu, Mmu, Ppu};
use isa::Address;

/// Gameboy
pub type Gb = System<swram::Fixed>;

/// Gameboy color
pub type Cgb = System<swram::Banked>;

/// Gameboy Keys state
pub type Buttons = EnumSet<Button>;

/// A gameboy button
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
pub struct System<S: Swram> {
    input: Buttons,
    cpu: Cpu,
    mmu: Mmu<S>,
    gpu: Ppu,
    apu: Apu,
}

impl<S: Swram + Default> System<S> {
    /// Create a new system with no loaded catridge
    pub fn new<B: Bios>(bios: B) -> Self {
        System {
            input: Buttons::empty(),
            cpu: Cpu::default(),
            mmu: Mmu::default(),
            gpu: Ppu::default(),
            apu: Apu::default(),
        }
    }
}

impl<S: Swram> System<S> {
    /// Load a catridge into the system and return the old one if there was one
    pub fn load(&mut self, cartridge: Cartridge) -> Option<Cartridge> {
        self.mmu.load(cartridge)
    }

    /// Unload the current cartridge, if there is one, and return it
    pub fn maybe_unload(&mut self) -> Option<Cartridge> {
        self.mmu.maybe_unload()
    }

    /// Step the sytem forward on instruction execution
    pub fn step(&mut self) -> u8 {
        self.mmu.update_input_registers(self.input); // update input state

        let cycles_in_step = self.cpu.step(&mut self.mmu);

        self.gpu.emulate(cycles_in_step as usize, &mut self.mmu);
        self.apu.emulate(cycles_in_step as usize, &mut self.mmu);
        cycles_in_step
    }

    /// Emulate the system taking a specified number of steps
    pub fn emulate(&mut self, cycles: usize) {
        let mut cycles = cycles; // TODO:  what happens when we get cycles not a multiple of 4?
        while cycles > 0 {
            cycles = cycles.saturating_sub(usize::from(self.step()));
        }
    }

    /// Set the input state the next cycle will read from, then return the
    /// input that was passed in
    pub fn set_input(&mut self, buttons: Buttons) {
        self.input = buttons;
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

    /// Return the current program counter
    pub fn pc(&self) -> Address {
        self.cpu.registers().pc
    }

    /// Return a reference to the memory managment unit (MMU)
    pub fn mmu(&self) -> &Mmu<S> {
        &self.mmu
    }
}
