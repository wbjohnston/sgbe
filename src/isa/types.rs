// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Type aliases for gameboy ISA

/// A 8-bit, signed, immediate value
pub type SignedImmediate = i8;

/// A word
pub type Word = u8;

/// A double sized word
pub type DoubleWord = u16;

/// An address
pub type Address = DoubleWord;

/// An 8-bit, unsigned, immediate value
pub type Immediate = Word;

/// A 16-bit, unsigned, immediate value
pub type Immediate16 = DoubleWord;