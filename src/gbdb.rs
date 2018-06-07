// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate core;
use core::hardware::bios::{GbBios, Bios};
use core::hardware::Cartridge;
use core::system::Gb;

#[macro_use]
extern crate failure;

extern crate common;
use common::try_read_rom;

use failure::Error;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};

const PS1: &'static str = "gbdb> ";

fn print_help() {
    println!(r#"TODO: write help message"#);
}

fn parse_command<'a>(command: &'a str) -> Command {
    match command {
        "help" => Command::Help,
        "step" => Command::Step,
        "show reg" => Command::ShowRegisters,
        "exit" => Command::Exit,
        _ => Command::Undefined,
    }
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Help,
    ShowRegisters,
    Step,
    Exit,
    Undefined,
}

fn main() -> Result<(), Error> {
    let bios_path = "./roms/bios.gb";
    let rom_path = "./roms/tetris.gb";

    let bios = GbBios::from(*include_bytes!("../roms/gb_bios.bin"));
    let mut emulator = Gb::new(bios);

    // let input = "step\n".to_string();
    let mut input = String::new();
    let mut last_command = String::new();

    // let len = input.len();
    let stdout = io::stdout();
    let mut stdin = BufReader::new(io::stdin());
    loop {
        let mut out_handle = stdout.lock();
        out_handle.write(PS1.as_bytes())?;
        out_handle.flush()?;

        let len = stdin.read_line(&mut input)?;

        match len {
            0 => break, // Not using stdin
            1 => input = last_command.clone(),
            v => last_command = input.clone(),
        }

        match parse_command(&input[0..len - 1]) {
            Command::Help => print_help(),
            Command::Step => {
                emulator.step();
            }
            Command::ShowRegisters => println!("{}", emulator.registers()),
            Command::Undefined => {
                out_handle.write(b"Undefined command\n")?;
            }
            Command::Exit => break,
        }

        input.clear();
    }

    Ok(())
}
