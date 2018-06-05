// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use isa::Word;

const KB: usize = 1024;

/// 4KB memory
pub type Memory4KB = [Word; 4 * KB];

/// 8KB memory
pub type Memory8KB = [Word; 8 * KB];

/// 16KB memory
pub type Memory16KB = [Word; 16 * KB];

/// 32KB memory
pub type Memory32KB = [Word; 32 * KB];
