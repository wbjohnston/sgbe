// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Memory addresses for registers in memory

use isa::Address;

// Registers addresses
pub const INTERRUPT_ENABLE: Address = 0xFFFF;

/// LCD Control register addres
pub const LCDC: Address = 0xFF40;

/// LCD Status register address
pub const LCDS: Address = 0xFF41;

/// Scroll position Y
pub const SCY: Address = 0xFF42;

/// Scroll position X
pub const SCX: Address = 0xFF43;

/// LY Compare reigster
pub const LYC: Address = 0x0000;

/// Window Y position
pub const WY: Address = 0xFF4A;

/// Window X position minus 7
pub const WX: Address = 0xFF4b;

/// BG Pallete data
pub const BGP: Address = 0xFF47;

/// Background pallete index
pub const BCPS: Address = 0xFF68;

/// Background pallete data
pub const BCPD: Address = 0xFF69;

/// Sprite pallete index
pub const OCPS: Address = 0xFF6A;

/// Spire pallete data
pub const OCPD: Address = 0xFF6B;

/// VRAM bank
pub const VBK: Address = 0xFF4F;

/// DMA transfer and start
pub const DMA: Address = 0xFF46;

/// New dma source, high
pub const HHDMA1: Address = 0xFF51;

/// New DMA source, low
pub const HDMA2: Address = 0xFF52;

/// New dma destination, high
pub const HDMA3: Address = 0xFF52;

/// New dma destination, low
pub const HDMA4: Address = 0xFF52;

/// New dma length or mode or start
pub const HDMA5: Address = 0xFF52;

pub const JOYP: Address = 0xFF0;