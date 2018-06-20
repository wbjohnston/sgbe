// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Hardware components

use isa::{DoubleWord, Word};

pub mod bios;

pub mod memory;

pub mod cartridge;
pub use self::cartridge::Cartridge;

pub mod cpu;
pub use self::cpu::Cpu;

pub mod ppu;
pub use self::ppu::Ppu;

pub mod mmu;
pub use self::mmu::Mmu;

pub mod apu;
pub use self::apu::Apu;

pub mod timer;
pub use self::timer::Timer;

// pub mod memory;

/// Return a double word composed from two [`Word`]s
pub fn pack_words(lo: Word, hi: Word) -> DoubleWord {
    u16::from(lo) | (u16::from(hi) << 8)
}

/// Return two [`Word`]s in a tuple of the form `(lo, hi)`
pub fn split_doubleword(word: DoubleWord) -> (Word, Word) {
    (word as u8, (word >> 8) as u8)
}

/// Return two nibbles
pub fn split_word(word: Word) -> (Word, Word) {
    (word & 0x0F, word >> 4)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_doubleword_functions() {
        let value = 0xDEAD;
        let sampled = split_doubleword(value);
        let expected: (u8, u8) = (0xAD, 0xDE);
        assert_eq!(expected, sampled);
    }

    #[test]
    fn pack_words_functions() {
        let (lo, hi): (u8, u8) = (0xAD, 0xDE);
        let expected = 0xDEAD;
        let sampled = pack_words(lo, hi);
        assert_eq!(expected, sampled);
    }

    #[test]
    fn split_word_functions() {
        let value = 0xAB;
        let expected = (0xB, 0xA);
        let sampled = split_word(value);
        assert_eq!(expected, sampled);
    }

    quickcheck! {
        /// Check that splitting then packing return the same result
        fn split_then_pack_is_identity(word: DoubleWord) -> bool {
            let (lo, hi) = split_doubleword(word);
            word == pack_words(lo, hi)
        }
    }
}
