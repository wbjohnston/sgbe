// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Common frontend functions

use std::io::{self, Read};
use std::fs;

/// Read a rom
pub fn try_read_rom<'a>(path: &'a str) -> io::Result<&[u8]> {
    unimplemented!()
    // let mut buffer = String::new();
    // let handle = fs::File::open(path)?;
    // handle.read_to_string(&mut buffer)?;
    // Ok(buffer.as_bytes())
}
