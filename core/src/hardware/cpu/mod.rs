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
    Address, DoubleWord, Flag, Immediate16, Immediate8, Instruction, Register16, Register8,
    SignedImmediate8, Word,
};

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
    pub fn step<M: Memory>(&mut self, memory: &mut M) -> u8 {
        // read in raw value of instruction into ir
        let instruction = decode(memory, self.registers.pc);
        self.registers.pc += Address::from(instruction.size());

        let cycles_used = self.execute(instruction, memory);
        cycles_used
    }

    /// Return a reference to the CPU's registers
    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    /// Execute an instruction, returning the number of cycles used
    fn execute<M: Memory>(&mut self, instruction: Instruction, memory: &mut M) -> u8 {
        use self::Instruction::*;
        let did_branch = match instruction {
            Nop => self.exectue_nop(),
            LdRrIi(register, immediate) => self.execute_ld_rr_ii(register, immediate),
            XorAR(register) => self.execute_xor_a_r(register),
            XorAI(immediate) => self.execute_xor_a_i(immediate),
            AndAR(register) => self.execute_and_a_r(register),
            BitIR(immediate, register) => self.execute_bit_i_r(immediate, register),
            LddAHl => self.execute_ldd_a_hl(memory),
            LddHlA => self.execute_ldd_hl_a(memory),
            LdHlR(register) => self.execute_ld_hl_r(register, memory),
            Di => self.execute_di(memory),
            Ei => self.execute_ei(memory),
            LdIocA => self.execute_ld_ioc_a(memory),
            LdAI(immediate) => self.execute_ld_a_i(immediate),
            LdRI(register, immediate) => self.execute_ld_r_i(register, immediate),
            LdIoA(immediate) => self.execute_ld_io_a(immediate, memory),
            LdAIo(immediate) => self.execute_ld_a_io(immediate, memory),
            AddAR(register) => self.execute_add_a_r(register),
            JrCondS(condition, offset) => self.exectue_jr_cond_s(condition, offset),
            LdRR(a, b) => self.execute_ld_r_r(a, b),
            LdARr(register) => self.execute_ld_a_rr(register, memory),
            CallIi(address) => self.execute_call_ii(address, memory),
            IncRr(register) => self.execute_inc_rr(register),
            IncR(register) => self.execute_inc_r(register),
            DecRr(register) => self.execute_dec_rr(register),
            DecR(register) => self.execute_dec_r(register),
            CpAI(immediate) => self.execute_cp_a_i(immediate),
            PushRr(register) => self.execute_push_rr(register, memory),
            v => {
                unimplemented!("{:?} instruction not implemented", v);
            }
        };

        if did_branch {
            instruction.cycles_on_branch()
        } else {
            instruction.cycles()
        }
    }

    #[inline]
    fn exectue_nop(&mut self) -> bool {
        false
    }

    #[inline]
    fn execute_ldd_hl_a<M: Memory>(&mut self, memory: &mut M) -> bool {
        let a_val = self.registers.read_register8(Register8::A);
        let hl_val = self.registers.read_register16(Register16::HL);
        memory.write(hl_val, a_val);

        // decrement HL
        self.registers.write_register16(Register16::HL, hl_val - 1);
        false
    }

    #[inline]
    fn execute_ldd_a_hl<M: Memory>(&mut self, memory: &M) -> bool {
        let hl_val = self.registers.read_register16(Register16::HL);
        self.registers
            .write_register8(Register8::A, memory.read(hl_val));
        self.registers.write_register16(Register16::HL, hl_val - 1);
        false
    }

    #[inline]
    fn execute_ld_hl_r<M: Memory>(&mut self, register: Register8, memory: &M) -> bool {
        let hl_val = self.registers.read_register16(Register16::HL);
        self.registers
            .write_register8(Register8::A, memory.read(hl_val));
        false
    }

    #[inline]
    fn execute_di<M: Memory>(&mut self, memory: &mut M) -> bool {
        memory.write(0xFFFF, 0x00); // TODO: Is this the right value?
        false
    }

    #[inline]
    fn execute_ei<M: Memory>(&mut self, memory: &mut M) -> bool {
        memory.write(0xFFFF, 0xFF); // TODO: is this the right value?
        false
    }

    #[inline]
    fn execute_ld_ioc_a<M: Memory>(&mut self, memory: &M) -> bool {
        self.registers.a = memory.read(0xFF00 + Address::from(self.registers.c));
        false
    }

    #[inline]
    fn execute_ld_a_i(&mut self, immediate: Immediate8) -> bool {
        self.registers.a = immediate;
        false
    }

    #[inline]
    fn execute_ld_r_i(&mut self, register: Register8, immediate: Immediate8) -> bool {
        self.registers.write_register8(register, immediate);
        false
    }

    #[inline]
    fn exectue_jr_cond_s(&mut self, condition: Flag, offset: SignedImmediate8) -> bool {
        if self.registers.read_flag(condition) {
            debug!("Instruction not implemented");
            true
        } else {
            debug!("Instruction not implemented");
            false
        }
    }

    #[inline]
    fn execute_bit_i_r(&mut self, immediate: Immediate8, register: Register8) -> bool {
        debug!("Instruction not implemented");

        // TODO:  implement me
        false
    }

    #[inline]
    fn execute_ld_rr_ii(&mut self, register: Register16, immediate: Immediate16) -> bool {
        self.registers.write_register16(register, immediate);
        false
    }

    #[inline]
    fn execute_and_a_r(&mut self, register: Register8) -> bool {
        // TODO: need to set flags
        let a_value = self.registers.read_register8(Register8::A);
        let reg_value = self.registers.read_register8(register);

        self.registers
            .write_register8(Register8::A, a_value & reg_value);

        false
    }

    #[inline]
    fn execute_add_a_r(&mut self, register: Register8) -> bool {
        // TODO: need to set flags
        let a_value = self.registers.read_register8(Register8::A);
        let reg_value = self.registers.read_register8(register);

        self.registers
            .write_register8(Register8::A, a_value + reg_value);

        false
    }

    #[inline]
    fn execute_xor_a_r(&mut self, register: Register8) -> bool {
        // TODO: need to set flags
        use self::Register8::A;
        let a_value = self.registers.read_register8(A);
        let reg_value = self.registers.read_register8(register);

        self.registers.write_register8(A, a_value ^ reg_value);

        false
    }

    #[inline]
    fn execute_xor_a_i(&mut self, immediate: Immediate8) -> bool {
        // TODO: need to set flags
        use self::Register8::A;
        let a_value = self.registers.read_register8(A);
        self.registers.write_register8(A, a_value ^ immediate);

        false
    }

    #[inline]
    fn execute_ld_io_a<M: Memory>(&mut self, immediate: Immediate8, memory: &M) -> bool {
        self.registers
            .write_register8(Register8::A, memory.read(0xFF00 + Address::from(immediate)));
        false
    }

    #[inline]
    fn execute_ld_a_io<M: Memory>(&mut self, immediate: Immediate8, memory: &mut M) -> bool {
        memory.write(
            0xFF00 + immediate as Address,
            self.registers.read_register8(Register8::A),
        );
        false
    }

    #[inline]
    fn execute_ld_r_r(&mut self, register_a: Register8, register_b: Register8) -> bool {
        let src = self.registers.read_register8(register_a);
        self.registers.write_register8(register_b, src);
        false
    }

    #[inline]
    fn execute_ld_a_rr<M: Memory>(&mut self, register: Register16, memory: &mut M) -> bool {
        let val = self.registers.read_register8(Register8::A);
        let address = self.registers.read_register16(register);
        memory.write(address, val);
        false
    }

    #[inline]
    fn execute_call_ii<M: Memory>(&mut self, address: Address, memory: &mut M) -> bool {
        self.registers.sp -= 2;
        let sp = self.registers.sp;
        memory.write_double(sp, self.registers.pc);
        self.registers.pc = address;
        false
    }

    #[inline]
    fn execute_inc_rr(&mut self, register: Register16) -> bool {
        let val = self.registers.read_register16(register);
        self.registers.write_register16(register, val + 1);
        false
    }

    #[inline]
    fn execute_cp_a_i(&mut self, immediate: Immediate8) -> bool {
        let a = self.registers.read_register8(Register8::A);
        self.registers.write_flag(Flag::Zero, a == immediate);
        self.registers.write_flag(Flag::AddSub, true);
        self.registers.write_flag(Flag::HalfCarry, true); // TODO: this is wrong
        self.registers.write_flag(Flag::Carry, a < immediate);
        false
    }

    #[inline]
    fn execute_push_rr<M: Memory>(&mut self, register: Register16, memory: &mut M) -> bool {
        self.registers.pc -= 2;
        let val = self.registers.read_register16(register);
        memory.write_double(self.registers.pc, val);
        false
    }

    #[inline]
    fn execute_inc_r(&mut self, register: Register8) -> bool {
        let val = self.registers.read_register8(register);
        self.registers.write_register8(register, val + 1);
        false
    }

    #[inline]
    fn execute_dec_rr(&mut self, register: Register16) -> bool {
        let val = self.registers.read_register16(register);
        self.registers.write_register16(register, val - 1);
        false
    }

    #[inline]
    fn execute_dec_r(&mut self, register: Register8) -> bool {
        let val = self.registers.read_register8(register);
        self.registers.write_register8(register, val - 1);
        false
    }
}
