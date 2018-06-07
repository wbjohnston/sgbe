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

    pub fn new_with_ir(ir: Word) -> Self {
        let mut registers = Registers::default();
        registers.ir = ir;
        CPU {
            ime: false,
            is_halted: false,
            registers: registers,
        }
    }

    /// Execute the current instruction and advance the CPU forward one step, Returns the
    /// number of cycles used
    pub fn step<S: SWRAM, B: Bios>(&mut self, mmu: &mut MMU<S, B>) -> u8 {
        // FIXME: make sure the ir is loaded with correct value on init
        self.registers.ir = mmu.read(self.registers.pc);
        // read in raw value of instruction into ir
        let instruction = decode(mmu, self.registers.pc);
        self.registers.pc += instruction.size() as Address;

        let cycles_used = self.execute(instruction, mmu);
        cycles_used
    }

    /// Return a reference to the CPU's registers
    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    /// Execute an instruction, returning the number of cycles used
    fn execute<S: SWRAM, B: Bios>(&mut self, instruction: Instruction, mmu: &mut MMU<S, B>) -> u8 {
        use self::Instruction::*;
        match instruction {
            i @ Nop => i.cycles(),
            LdRrIi(..) => self.execute_LdRrIi(instruction),
            XorAR(..) => self.execute_XorAR(instruction),
            XorAI(..) => self.execute_XorAI(instruction),
            AndAR(..) => self.execute_AndAR(instruction),
            LddHlA => {
                self.registers.a = mmu.read(self.registers.hl());
                let hl = self.registers.hl();
                self.registers.set_hl(hl + 1);

                instruction.cycles()
            }
            v => {
                unimplemented!("{:?} instruction not implemented", v);
            }
        }
    }

    #[allow(non_snake_case)]
    fn execute_LdRrIi(&mut self, instruction: Instruction) -> u8 {
        use self::Instruction::LdRrIi;
        use self::Register16::*;

        match instruction {
            LdRrIi(BC, imm) => self.registers.set_bc(imm),
            LdRrIi(DE, imm) => self.registers.set_de(imm),
            LdRrIi(HL, imm) => self.registers.set_hl(imm),
            LdRrIi(SP, imm) => self.registers.sp = imm,
            LdRrIi(..) => unimplemented!("{:?} instruction not implemented", instruction),
            _ => unreachable!(),
        }

        instruction.cycles()
    }

    #[allow(non_snake_case)]
    fn execute_AndAR(&mut self, instruction: Instruction) -> u8 {
        use self::Instruction::AndAR;
        use self::Register8::*;

        match instruction {
            AndAR(A) => self.registers.a &= self.registers.a,
            AndAR(B) => self.registers.a &= self.registers.b,
            AndAR(C) => self.registers.a &= self.registers.c,
            AndAR(D) => self.registers.a &= self.registers.d,
            AndAR(E) => self.registers.a &= self.registers.e,
            AndAR(H) => self.registers.a &= self.registers.h,
            AndAR(L) => self.registers.a &= self.registers.l,
            AndAR(..) => unimplemented!("{:?} instruction not implemented", instruction),
            _ => unreachable!(),
        }

        instruction.cycles()
    }

    #[allow(non_snake_case)]
    fn execute_XorAR(&mut self, instruction: Instruction) -> u8 {
        use self::Instruction::XorAR;
        use self::Register8::*;

        match instruction {
            XorAR(A) => self.registers.a = 0,
            XorAR(B) => self.registers.a ^= self.registers.b,
            XorAR(C) => self.registers.a ^= self.registers.c,
            XorAR(D) => self.registers.a ^= self.registers.d,
            XorAR(E) => self.registers.a ^= self.registers.e,
            XorAR(H) => self.registers.a ^= self.registers.h,
            XorAR(L) => self.registers.a ^= self.registers.l,
            XorAR(..) => unimplemented!("{:?} instruction not implemented", instruction),
            _ => unreachable!(),
        }

        instruction.cycles()
    }

    #[allow(non_snake_case)]
    pub fn execute_XorAI(&mut self, instruction: Instruction) -> u8 {
        use self::Instruction::XorAI;

        match instruction {
            XorAI(v) => self.registers.a ^= v,
            _ => unreachable!(),
        }

        instruction.cycles()
    }
}
