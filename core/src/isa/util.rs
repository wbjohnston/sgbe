// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utility functions

use super::types::{DoubleWord, Word};

/// Return a double word composed from two [`Word`]s
pub fn pack_words(lo: Word, hi: Word) -> DoubleWord {
    (lo as DoubleWord) & ((hi as DoubleWord) << 8)
}

/// Return two [`Word`]s in a tuple of the form `(lo, hi)`
pub fn split_doubleword(word: DoubleWord) -> (Word, Word) {
    (word as u8, (word << 8) as u8)
}

/// Return two nibbles
pub fn split_word(word: Word) -> (Word, Word) {
    let lo = word & 0xF0;
    let hi = (word & 0x0F) << 4;
    (lo, hi)
}