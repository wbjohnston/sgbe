// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// NOTE: (will) both the gb cpu and x86_64 arch are little endian

/// A Gameboy CPU
pub struct CPU {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

impl CPU {
    /// Return the current program counter
    pub fn program_counter(&self) -> u16 {
        self.pc
    }

    /// Return the current stack pointer
    pub fn stack_pointer(&self) -> u16 {
        self.sp
    }

    /// Return the value of the `A` psuedo-register
    ///
    /// Note: the `A` "register" is actually the upper 8 bits of the `AF` register
    pub fn a_reg(&self) -> u8 {
        // TODO: (will) determine if this actually works
        (self.af & 0x00FF) as u8
    }

    /// Return the value of the `B` psuedo-register
    ///
    /// Note: the `B` register is actually the upper 8 bits of the `BC` register
    pub fn b_reg(&self) -> u8 {
        (self.bc & 0x00FF) as u8
    }

    /// Return the value of the `C` psuedo-register
    ///
    /// Note: the `C` "register" is actually the lower 8 bits of the `BC` register
    pub fn c_reg(&self) -> u8 {
        (self.bc & 0xFF00) as u8
    }

    /// Return the value of the `D` psuedo-register
    ///
    /// Note: the `D` "register" is actually the upper 8 bits of the `DE` register
    pub fn d_reg(&self) -> u8 {
        (self.de & 0x00FF) as u8
    }

    /// Return the value of the `E` psuedo-register
    ///
    /// Note: the `E` "register" is actually the lower 8 bits of the `DE` register
    pub fn e_reg(&self) -> u8 {
        (self.de & 0xFF00) as u8
    }

    /// Return the value of the `H` psuedo-register
    ///
    /// Note: the `H` "register" is actually the upper 8 bits of the `HL` register
    pub fn h_reg(&self) -> u8 {
        (self.hl & 0x00FF) as u8
    }

    /// Return the value of the `L` psuedo-register
    ///
    /// Note: the `L` register is actually the lower 8 bits of the `HL` register
    pub fn l_reg(&self) -> u8 {
        (self.hl & 0xFF00) as u8
    }

    /// Return the `FLAG` register
    ///
    /// Note: the `FLAG` register is actually the lower 8 bits of the `AF` register
    pub fn flag_reg(&self) -> u8 {
        (self.af & 0xFF00) as u8
    }

    /// Returns true if the result of an operation was zero
    pub fn zero_flag(&self) -> bool {
        // Flags:    xxxx_chnz_xxxx_xxxx
        (self.af & 0b0000_0001_0000_0000) != 0
    }

    /// Returns true under any of the following conditions:
    /// * An addition instruction resulted in an overflow
    /// * A subtraction operation resulted in an underflow
    /// * A shift operation shifted out a `1`
    pub fn carry_flag(&self) -> bool {
        // Flags:    xxxx_chnz_xxxx_xxxx
        (self.af & 0b0000_1000_0000_0000) != 0
    }

    /// TODO: (will) what does this do
    pub fn add_sub_flag(&self) -> bool {
        // Flags:    xxxx_chnz_xxxx_xxxx
        (self.af & 0b0000_0010_0000_0000) != 0
    }

    /// TODO: (will) what does this do
    pub fn half_carry_flag(&self) -> bool {
        // Flags:    xxxx_chnz_xxxx_xxxx
        (self.af & 0b0000_0100_0000_0000) != 0
    }
}

/// Gameboy CPU opcode
pub enum Opcode {}

impl Opcode {
    /// Return the number of cpu cycles a given instruction takes to execute
    pub fn cycles(&self) -> u8 {
        match *self {
            _ => unimplemented!(),
        }
    }
}
