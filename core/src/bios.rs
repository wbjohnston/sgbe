// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Clone, Copy)]
pub struct SystemBIOS([u8; 0x100]); 

impl<'a> From<&'a [u8]> for SystemBIOS {
    fn from(stream: &'a [u8]) -> SystemBIOS {
        unimplemented!()
    }
}