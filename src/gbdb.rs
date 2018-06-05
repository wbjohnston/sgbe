// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate core;
use core::system::GB;
use core::cartridge::Cartridge;
use core::bios::SystemBIOS;

#[macro_use]
extern crate failure;

extern crate common;
use common::try_read_rom;

use failure::Error;
use std::io::{self, Write, Read, BufRead, BufReader};
use std::fs;

const PS1: &'static str = "gbdb> ";

fn print_help() {
    println!(r#"TODO: write help message"#);
}

fn parse_command<'a>(command: &'a str) -> Command {
    unimplemented!()
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Help,
    ShowRegisters,
    Step,
}

fn main() -> Result<(), Error> {
    let bios_path = "./roms/bios.gb";
    let rom_path = "./roms/tetris.gb";

    let bios = SystemBIOS::from(try_read_rom(bios_path)?);
    let cartridge = Cartridge::try_parse_bytes(try_read_rom(rom_path)?)?;

    let mut emulator = GB::new(bios, cartridge);

    let mut input = String::new();
    let stdout = io::stdout();
    let mut stdin = BufReader::new(io::stdin());
    loop {
        let mut out_handle = stdout.lock();
        out_handle.write(PS1.as_bytes())?;
        out_handle.flush()?;

        let len = stdin.read_line(&mut input)?;

        match parse_command(&input[0..len - 1]) {
            Command::Help => print_help(),
            Command::Step => {emulator.step(); },
            Command::ShowRegisters => println!("TODO"),
            _ => unimplemented!()
        }
    }

    Ok(())
}