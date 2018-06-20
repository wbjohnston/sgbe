// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

/// A 8-bit, signed, immediate value
pub type SignedImmediate8 = i8;

/// A word
pub type Word = u8;

/// A double sized word
pub type DoubleWord = u16;

/// An address
pub type Address = DoubleWord;

/// An 8-bit, unsigned, immediate value
pub type Immediate8 = Word;

/// A 16-bit, unsigned, immediate value
pub type Immediate16 = DoubleWord;

// An 8-bit register
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl fmt::Display for Register8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Register8::*;
        let st = match *self {
            A => "A",
            F => "F",
            B => "B",
            C => "C",
            D => "D",
            E => "E",
            H => "H",
            L => "L",
        };

        write!(f, "{}", st)
    }
}

/// A 16-bit register
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl fmt::Display for Register16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Register16::*;
        let st = match *self {
            AF => "AF",
            BC => "BC",
            DE => "DE",
            HL => "HL",
            SP => "SP",
            PC => "PC",
        };

        write!(f, "{}", st)
    }
}

/// A CPU flag
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Flag {
    Zero,
    AddSub,
    HalfCarry,
    Carry,
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Flag::*;
        let st = match *self {
            Zero => "Z",
            AddSub => "N",
            HalfCarry => "H",
            Carry => "C",
        };

        write!(f, "{}", st)
    }
}

/// A Gameboy CPU Instruction
///
/// # Notes
/// The naming conventions for is the name of the instruction followed by one or two suffixes:
/// * `R`: 8-bit register address
/// * `Rr`: 16-bit register address
/// * `I`: 8-bit, unsigned, immediate value
/// * `Ii`: 16-bit, unsigned, immediate value
/// * `S`: 8-bit, signed, immediate value
/// * `A`: the 8-bit `A` accumulator register
/// * `Hl`: the `HL 16-bit register
/// * `SP`: The 16-bit `SP` stack pointer register
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    // 8-bit load instructions
    LdRR(Register8, Register8),
    LdRI(Register8, Immediate8),
    LdRHl(Register8),
    LdHlR(Register8),
    LdHlI(Immediate8),
    LdARr(Register16),
    LdAI(Immediate8),
    LdRrA(Register16),
    LdIA(Immediate8),
    LdAIo(Immediate8),
    LdIoA(Immediate8),
    LdIiA,
    LdAIoc,
    LdIocA,

    LdiHlA,
    LdiAHl,

    LddHlA,
    LddAHl,

    // 16-bit load instructions
    LdRrIi(Register16, Immediate16),
    LdSpHl,
    LdIiSp(Immediate16),
    PushRr(Register16),
    Pop(Register16),

    // 8-bit ALU instructions
    AddAR(Register8),
    AddAI(Immediate8),
    AddAHl,

    AdcAR(Register8),
    AdcAI(Immediate8),
    AdcAHl,

    SubAR(Register8),
    SubAI(Immediate8),
    SubAHl,

    SbcAR(Register8),
    SbcAI(Immediate8),
    SbcAHl,

    AndAR(Register8),
    AndAI(Immediate8),
    AndAHl,

    XorAR(Register8),
    XorAI(Immediate8),
    XorAHl,

    OrAR(Register8),
    OrAI(Immediate8),
    OrAHl,

    CpAR(Register8),
    CpAI(Immediate8),
    CpAHl,

    IncR(Register8),
    IncHl,

    DecR(Register8),
    DecHl,

    Daa,
    Cpl,

    // 16-bit ALU instructions
    AddHlRr(Register16),
    AddSpS(SignedImmediate8),
    IncRr(Register16),
    DecRr(Register16),
    LdHlSp(SignedImmediate8),

    // Rotate/shift commands
    Rlca,

    Rla,

    Rrca,

    Rra,

    RlcR(Register8),
    RlcHl,

    RlR(Register8),
    RlHl,

    RrcR(Register8),
    RrcHl,

    RrR(Register8),
    RrHl,

    SlaR(Register8),
    SlaHl,

    SwapR(Register8),
    SwapHl,

    SraR(Register8),
    SraHl,

    SrlR(Register8),
    SrlHl,

    // Single-bit commands
    BitIR(Immediate8, Register8),
    BitIHl(Immediate8),

    SetIR(Immediate8, Register8),
    SetIHl(Immediate8),

    ResIR(Immediate8, Register8),
    ResIHl(Immediate8),

    // Control commands
    Ccf,

    Scf,

    Nop,

    Halt,

    Stop,

    Di,

    Ei,

    // Jump commands
    JpIi(Immediate16),
    JpCondIi(Flag, Immediate16),
    JpHl,

    JrS(SignedImmediate8),
    JrCondS(Flag, SignedImmediate8),

    CallIi(Address),
    CallCondIi(Flag, Address),

    Ret,
    RetCond(Flag),

    Reti,

    Rst(Immediate8),

    // Undefined instruction
    Undefined(Word),
}

impl Instruction {
    /// Return the number of cycles an instruction takes
    ///
    /// # Notes
    /// Assumes the instruction is not branching
    pub fn cycles(self) -> u8 {
        use self::Instruction::*;
        // TODO:  implement Instruction::cycles()
        match self {
            // 8-bit load instructions
            LdRR(_, _) => 4,
            LdRI(_, _) => 8,
            LdRHl(_) => 8,
            LdHlR(_) => 8,
            LdHlI(_) => 12,
            LdARr(_) => 8,
            LdAI(_) => 16,
            LdRrA(_) => 8,
            LdIA(_) => 16,
            LdAIo(_) => 12,
            LdIoA(_) => 12,
            LdIiA => 16,
            LdAIoc => 8,
            LdIocA => 8,

            LdiHlA => 8,
            LdiAHl => 8,

            LddHlA => 8,
            LddAHl => 8,

            // 16-bit load instructions
            LdRrIi(_, _) => 12,

            LdSpHl => 8,

            PushRr(_) => 16,

            Pop(_) => 12,

            // 8-bit ALU instructions
            AddAR(_) => 4,
            AddAI(_) => 8,
            AddAHl => 8,

            AdcAR(_) => 4,
            AdcAI(_) => 8,
            AdcAHl => 8,

            SubAR(_) => 4,
            SubAI(_) => 8,
            SubAHl => 8,

            SbcAR(_) => 4,
            SbcAI(_) => 8,
            SbcAHl => 8,

            AndAR(_) => 4,
            AndAI(_) => 8,
            AndAHl => 8,

            XorAR(_) => 4,
            XorAI(_) => 8,
            XorAHl => 8,

            OrAR(_) => 4,
            OrAI(_) => 8,
            OrAHl => 8,

            CpAR(_) => 4,
            CpAI(_) => 8,
            CpAHl => 8,

            IncR(_) => 4,
            IncHl => 12,

            DecR(_) => 4,
            DecHl => 12,

            Daa => 4,

            Cpl => 4,

            // 16-bit ALU instrcutions
            AddHlRr(_) => 8,
            AddSpS(_) => 16,

            IncRr(_) => 8,

            DecRr(_) => 8,

            LdHlSp(_) => 12,
            LdIiSp(_) => unimplemented!(), // TODO:  implement me

            // Shift and rotate commands
            Rlca => 4,

            Rla => 4,

            Rrca => 4,

            Rra => 4,

            RlcR(_) => 8,
            RlcHl => 16,

            RlR(_) => 8,
            RlHl => 16,

            RrcR(_) => 8,
            RrcHl => 16,

            RrR(_) => 8,
            RrHl => 16,

            SlaR(_) => 8,
            SlaHl => 16,

            SwapR(_) => 8,
            SwapHl => 16,

            SraR(_) => 8,
            SraHl => 16,

            SrlR(_) => 8,
            SrlHl => 16,

            // Bitwise commands
            BitIR(_, _) => 8,
            BitIHl(_) => 12,

            SetIR(_, _) => 8,
            SetIHl(_) => 12,

            ResIR(_, _) => 8,
            ResIHl(_) => 12,

            // Control commands
            Ccf => 4,

            Scf => 4,

            Nop => 4,

            Halt => 4,

            Stop => 0,

            Di => 4,

            Ei => 4,

            JpIi(_) => 16,
            JpHl => 4,
            JpCondIi(_, _) => 12,

            JrS(_) => 12,
            JrCondS(_, _) => 12,

            CallIi(_) => 24,
            CallCondIi(_, _) => 12,

            Ret => 16,
            RetCond(_) => 8,

            Reti => 16,

            Rst(_) => 16,

            Undefined(_) => 0,
        }
    }

    /// Return the number of cycles an instruction takes when it branches
    ///
    /// # Notes
    /// Non-branching instructions return the same value as `cycles` was called on it
    pub fn cycles_on_branch(self) -> u8 {
        use self::Instruction::*;
        match self {
            JpCondIi(_, _) => 16,

            JrCondS(_, _) => 12,

            CallCondIi(_, _) => 24,

            RetCond(_) => 20,

            v => v.cycles(),
        }
    }

    pub fn size(self) -> u8 {
        use self::Instruction::*;
        match self {
            // 8-bit load instructions
            LdRR(_, _) => 1,
            LdRI(_, _) => 2,
            LdRHl(_) => 1,
            LdHlR(_) => 1,
            LdHlI(_) => 2,
            LdARr(_) => 1,
            LdAI(_) => 2,
            LdRrA(_) => 1,
            LdIA(_) => 1,
            LdAIo(_) => 1,
            LdIoA(_) => 1,
            LdIiA => 3,
            LdAIoc => 2,
            LdIocA => 2,

            LdiHlA => 1,
            LdiAHl => 1,

            LddHlA => 1,
            LddAHl => 1,

            // 16-bit load instructions
            LdRrIi(_, _) => 3,

            LdSpHl => 1,

            PushRr(_) => 1,

            Pop(_) => 1,

            // 8-bit ALU instructions
            AddAR(_) => 1,
            AddAI(_) => 2,
            AddAHl => 1,

            AdcAR(_) => 1,
            AdcAI(_) => 2,
            AdcAHl => 1,

            SubAR(_) => 1,
            SubAI(_) => 2,
            SubAHl => 1,

            SbcAR(_) => 1,
            SbcAI(_) => 2,
            SbcAHl => 1,

            AndAR(_) => 1,
            AndAI(_) => 2,
            AndAHl => 1,

            XorAR(_) => 1,
            XorAI(_) => 2,
            XorAHl => 1,

            OrAR(_) => 1,
            OrAI(_) => 2,
            OrAHl => 1,

            CpAR(_) => 1,
            CpAI(_) => 2,
            CpAHl => 1,

            IncR(_) => 1,
            IncHl => 1,

            DecR(_) => 1,
            DecHl => 1,

            Daa => 1,

            Cpl => 1,

            // 16-bit ALU instrcutions
            AddHlRr(_) => 1,
            AddSpS(_) => 2,

            IncRr(_) => 1,

            DecRr(_) => 1,

            LdHlSp(_) => 2,
            LdIiSp(_) => unimplemented!(), // TODO:  implement me

            // Shift and rotate commands
            Rlca => 1,

            Rla => 1,

            Rrca => 1,

            Rra => 1,

            RlcR(_) => 2,
            RlcHl => 2,

            RlR(_) => 2,
            RlHl => 2,

            RrcR(_) => 2,
            RrcHl => 2,

            RrR(_) => 2,
            RrHl => 2,

            SlaR(_) => 2,
            SlaHl => 2,

            SwapR(_) => 2,
            SwapHl => 2,

            SraR(_) => 2,
            SraHl => 2,

            SrlR(_) => 2,
            SrlHl => 2,

            // Bitwise commands
            BitIR(_, _) => 2,
            BitIHl(_) => 2,

            SetIR(_, _) => 2,
            SetIHl(_) => 2,

            ResIR(_, _) => 2,
            ResIHl(_) => 2,

            // Control commands
            Ccf => 1,

            Scf => 1,

            Nop => 1,

            Halt => 1,

            Stop => 2,

            Di => 1,

            Ei => 1,

            JpIi(_) => 3,
            JpHl => 1,
            JpCondIi(_, _) => 3,

            JrS(_) => 2,
            JrCondS(_, _) => 2,

            CallIi(_) => 3,
            CallCondIi(_, _) => 3,

            Ret => 1,
            RetCond(_) => 1,

            Reti => 1,

            Rst(_) => 1,

            Undefined(_) => 0,
        }
    }
}
