// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Memory bank controller trait and implementations

use traits::Memory;

mod fixed;
pub use self::fixed::Fixed;

mod mbc1;
pub use self::mbc1::MBC1;

mod mbc2;
pub use self::mbc2::MBC2;

mod mbc3;
pub use self::mbc3::MBC3;

mod mbc5;
pub use self::mbc5::MBC5;

mod huc1;
pub use self::huc1::HuC1;

/// A memory bank controller
pub trait MBC: Memory {}
