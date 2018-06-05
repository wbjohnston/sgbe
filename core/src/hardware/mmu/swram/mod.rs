// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Switchable work ram trait and implementations

mod fixed;
pub use self::fixed::Fixed;

mod banked;
pub use self::banked::Banked;

use hardware::Memory;

/// Marker trait to signify memory can be used as switchable work ram
pub trait SWRAM: Memory {}
