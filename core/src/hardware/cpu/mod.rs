// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy CPU

mod registers;
pub use self::registers::Registers;

use disasm::decode;
use hardware::memory::Memory;
use isa::{
    Address, DoubleWord, Flag, Immediate, Immediate16, Instruction, Register16, Register8, Word,
};

use hardware::bios::Bios;
use hardware::mmu::SWRAM;
use hardware::MMU;

pub const CYCLES_PER_SECOND: usize = 4_194_304;

/// A Gameboy central processing unit
#[derive(Debug, Clone, Copy)]
pub struct CPU {
    ime: bool,
    is_halted: bool,
    registers: Registers,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            // interrupt master enable
            ime: false,
            is_halted: false,
            registers: Registers::default(),
        }
    }

    /// Execute the current instruction and advance the CPU forward one step, Returns the
    /// number of cycles used
    pub fn step<S: SWRAM, B: Bios>(&mut self, mmu: &mut MMU<S, B>) -> u8 {
        // read in raw value of instruction into ir
        self.registers.ir = mmu.read(self.registers.pc);
        let instruction = decode(self.registers.pc, mmu);
        self.registers.pc += instruction.size() as Address;

        let cycles_used = self.execute(instruction);
        cycles_used
    }

    /// Return a reference to the CPU's registers
    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    /// Execute an instruction, returning the number of cycles used
    fn execute(&mut self, instr: Instruction) -> u8 {
        use self::Instruction::*;
        let did_branch = match instr {
            Nop => false,
            v => unimplemented!("{:?} instruction not implemented", v),
        };

        if did_branch {
            instr.cycles_on_branch()
        } else {
            instr.cycles()
        }
    }
}
