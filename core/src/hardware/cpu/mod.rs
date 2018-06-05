// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy CPU

// NOTE: (will) both the gb cpu and x86_64 arch are little endian
// TODO: (will) implement all functions for big-endian platforms
// TODO: (will) implement gameboy color register file

mod registers;
use self::registers::Registers;

use disasm::decode;
use isa::{
    Address, DoubleWord, Flag, Immediate, Immediate16, Instruction, Register16, Register8, Word,
};

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
            registers: Registers::new(),
        }
    }

    pub fn emulate<S: SWRAM>(&mut self, cycles: usize, mmu: &mut MMU<S>) {
        unimplemented!()
    }

    /// Execute the current instruction and advance the CPU forward one step, Returns the
    /// number of cycles used
    pub fn step<S: SWRAM>(&mut self, bus: &mut MMU<S>) -> u8 {
        let instruction = decode(self.registers.pc, bus);
        self.registers.pc += instruction.size() as Address;

        let cycles_used = self.execute(instruction);
        cycles_used
    }

    /// Execute an instruction, returning the number of cycles used
    fn execute(&mut self, instr: Instruction) -> u8 {
        use self::Instruction::*;
        let did_branch = match instr {
            Nop => false,
            _ => unimplemented!(),
        };

        if did_branch {
            instr.cycles_on_branch()
        } else {
            instr.cycles()
        }
    }
}
