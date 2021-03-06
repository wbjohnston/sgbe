// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gameboy timer type

/// A Gameboy timer
#[derive(Debug, Copy, Clone)]
pub struct Timer;

impl Timer {
    /// Emulate the function of a timer over a given number of cycles
    pub fn emulate(&mut self, cycles: usize) {
        // TODO: implement timer emmulation
        debug!("FIXME: timer not implemented")
    }
}

impl Default for Timer {
    fn default() -> Self {
        // TODO: implement Default for timer
        Timer {}
    }
}
