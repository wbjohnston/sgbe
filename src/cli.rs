// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate failure;
extern crate core;

use core::cartridge::Cartridge;
use core::system::{CGB, GB};
use failure::Error;
use std::env;
use std::fs;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    let mut args = env::args();
    let _ = args.next();

    let cartridge = {
        let mut fh = fs::File::open(args.next().unwrap())?;
        let mut buffer = String::new();
        let _ = fh.read_to_string(&mut buffer)?;
        let bytes = buffer.as_bytes();
        Cartridge::try_parse_bytes(bytes)
    };

    let bios = fs::File::open(args.next().unwrap())?;
    let mode = Some("gb");

    // let mut emulator = mode = match args.next()
    //     .map(|x| x.to_lowercase().as_str())
    // {
    //     Some("gb") => GB::new((), cartridge),
    //     Some("cgb") => CGB::new((), cartridge),
    //     _ => unimplemented!()
    // };

    Ok(())
}
