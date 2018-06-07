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
const INTERRUPT_ENABLE_REGISTER_ADDRESS: Address = 0xFFFF;

/// LCD Control register addres
const LCDC_REGISTER_ADDRESS: Address = 0xFF40;

/// LCD Status register address
const LCDS_REGISTER_ADDRESS: Address = 0xFF41;

/// Scroll position Y
const SCY_REGISTER_ADDRESS: Address = 0xFF42;

/// Scroll position X
const SCX_REGISTER_ADDRESS: Address = 0xFF43;

const LYC_ADDRESS: Address = 0x0000;