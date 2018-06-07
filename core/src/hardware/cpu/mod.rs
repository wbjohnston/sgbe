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
    pub fn step<M: Memory>(&mut self, mmu: &mut M) -> u8 {
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
    fn execute<M: Memory>(&mut self, instruction: Instruction, memory: &mut M) -> u8 {
        use self::Instruction::*;
        match instruction {
            i @ Nop => i.cycles(),
            LdRrIi(..) => self.execute_LdRrIi(instruction),
            XorAR(..) => self.execute_XorAR(instruction),
            XorAI(..) => self.execute_XorAI(instruction),
            AndAR(..) => self.execute_AndAR(instruction),
            BitIR(..) => self.execute_BitIR(instruction),
            JrCondS(..) => self.exectue_JrCondS(instruction),
            LdRI(..) => self.execute_LdRI(instruction),
            LdAI(..) => self.execute_LdAI(instruction),
            LdHlR(..) => self.execute_LdHlR(instruction, memory),
            AddAR(..) => self.execute_AddAR(instruction),
            LdIocA => self.execute_LdIocA(instruction, memory),
            Di => self.execute_Di(instruction, memory),
            LddHlA => {
                self.registers.a = memory.read(self.registers.hl());
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
    fn execute_LdHlR<M: Memory>(&mut self, instruction: Instruction, memory: &mut M) -> u8 {
        use self::Instruction::LdHlR;
        use self::Register8::*;
        match instruction {
            LdHlR(A) => memory.write(self.registers.hl(), self.registers.a),
            _ => unimplemented!()
        }
        instruction.cycles()
    }

    #[allow(non_snake_case)]
    fn execute_Di<M: Memory>(&mut self, instruction: Instruction, memory: &mut M) -> u8 {
        memory.write(0xFFFF, 0x00);
        instruction.cycles()
    }

    #[allow(non_snake_case)]
    fn execute_LdIocA<M: Memory>(&mut self, instruction: Instruction, memory: &M) -> u8 {
        self.registers.a = memory.read(0xFF00 + self.registers.c as DoubleWord);
        instruction.cycles()
    }

    #[allow(non_snake_case)]
    fn execute_LdAI(&mut self, instruction: Instruction) -> u8 {
        use self::Instruction::LdAI;
        use self::Register8::*;

        match instruction {
            LdAI(v) => self.registers.a = v,
            _ => unimplemented!(),
        }

        instruction.cycles()
    }

    #[allow(non_snake_case)]
    fn execute_LdRI(&mut self, instruction: Instruction) -> u8 {
        use self::Instruction::LdRI;
        use self::Register8::*;

        match instruction {
            LdRI(C, imm) => self.registers.c = imm,
            _ => unimplemented!(),
        }

        instruction.cycles()
    }

    #[allow(non_snake_case)]
    fn exectue_JrCondS(&mut self, instruction: Instruction) -> u8 {
        use self::Flag::*;
        use self::Instruction::JrCondS;
        match instruction {
            JrCondS(Nf, v) => if self.registers.nf_is_set() {
                self.registers.pc = if v < 0 {
                    self.registers.pc - (-v) as Address
                } else {
                    self.registers.pc + v as Address
                };
                instruction.cycles_on_branch()
            } else {
                instruction.cycles()
            },
            _ => unimplemented!(),
        }
    }

    #[allow(non_snake_case)]
    fn execute_BitIR(&mut self, instruction: Instruction) -> u8 {
        use self::Instruction::BitIR;
        use self::Register8::*;
        debug!("Executing BitIR instruction");
        match instruction {
            BitIR(v, H) => debug!("call to {:?} is not implemented", instruction),
            BitIR(..) => unimplemented!("{:?} instruction is not implemented", instruction),
            _ => unreachable!(),
        }

        instruction.cycles()
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
    fn execute_AddAR(&mut self, instruction: Instruction) -> u8 {
        use self::Instruction::AddAR;
        use self::Register8::*;

        match instruction {
            AddAR(A) => self.registers.a += self.registers.a,
            AddAR(B) => self.registers.a += self.registers.b,
            AddAR(C) => self.registers.a += self.registers.c,
            AddAR(D) => self.registers.a += self.registers.d,
            AddAR(E) => self.registers.a += self.registers.e,
            AddAR(H) => self.registers.a += self.registers.h,
            AddAR(L) => self.registers.a += self.registers.l,
            AddAR(..) => unimplemented!("{:?} instruction not implemented", instruction),
            _ => unreachable!(),
        }

        instruction.cycles()
    }

    #[allow(non_snake_case)]
    fn execute_XorAR(&mut self, instruction: Instruction) -> u8 {
        use self::Instruction::XorAR;
        use self::Register8::*;

        match instruction {
            XorAR(A) => self.registers.a ^= self.registers.a,
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
