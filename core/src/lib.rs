// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "nightly", feature(test))]

#[macro_use]
extern crate log;
#[macro_use]
extern crate smallvec;

#[macro_use]
extern crate failure;

pub mod disasm;

pub mod bios;

pub mod hardware;

pub mod system;

pub mod isa;
