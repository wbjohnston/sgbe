// Copyright 2018 Will Johnston
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature="logging")]
extern crate env_logger;

extern crate core;
use core::disasm::decode;
use core::hardware::bios::{Bios, GbBios};
use core::hardware::memory::Memory;
use core::hardware::Cartridge;
use core::isa::Address;
use core::system::Gb;

#[macro_use]
extern crate failure;

use failure::Error;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};

const PS1: &str = "gbdb> ";

#[derive(Debug, Copy, Clone)]
enum Command {
    Help,
    ShowRegisters,
    ShowMemory,
    ShowInstruction,
    Step,
    Exit,
    Undefined,
}

fn print_help() {
    println!(r#"TODO: write help message"#);
}

fn parse_command(command: &str) -> Command {
    match command {
        "h" | "help" => Command::Help,
        "s" | "step" => Command::Step,
        "r" | "show reg" => Command::ShowRegisters,
        "m" | "show mem" => Command::ShowMemory,
        "i" | "show instruction" => Command::ShowInstruction,
        "x" | "exit" => Command::Exit,
        _ => Command::Undefined,
    }
}

fn print_memory<M: Memory>(memory: &M, base: Address, margin: usize) {
    // FIXME: make this put the active memory address at the cente of the readout
    for addr in (0x00..0xFF_u8)
        .cycle()
        .skip((base) as usize)
        .take((margin * 2 + 1) as usize)
    {
        let addr = Address::from(addr);
        if addr == base {
            println!("[{:4x}] : {:x}", addr, memory.read(addr))
        } else {
            println!(" {:4x} : {:x}", addr, memory.read(addr))
        }
    }
}

fn print_instruction<M: Memory>(memory: &M, address: Address) {
    println!("{:?}", decode(memory, address));
}

fn main() -> Result<(), Error> {
    #[cfg(feature="logging")]
    let _ = env_logger::init();

    let bios = GbBios::from(*include_bytes!("../roms/gb_bios.bin"));
    let rom = include_bytes!("../roms/tetris.gb");
    let cartridge = Cartridge::try_parse_bytes(rom)?;
    let mut emulator = Gb::new(bios);
    emulator.load(cartridge);

    // let input = "step\n".to_string();
    let mut input = String::new();

    // let len = input.len();
    let stdout = io::stdout();
    let mut stdin = BufReader::new(io::stdin());
    loop {
        let mut out_handle = stdout.lock();
        out_handle.write_all(PS1.as_bytes())?;
        out_handle.flush()?;

        let len = stdin.read_line(&mut input)?;
        let last_pc = emulator.pc();

        match parse_command(&input[0..len - 1]) {
            Command::Help => print_help(),
            Command::Step => {
                emulator.step();
            }
            Command::ShowRegisters => println!("{}", emulator.registers()),
            Command::ShowMemory => print_memory(emulator.mmu(), emulator.pc(), 5),
            Command::ShowInstruction => print_instruction(emulator.mmu(), last_pc),
            Command::Undefined => {
                out_handle.write_all(b"Undefined command\n")?;
            }
            Command::Exit => break,
        }

        input.clear();
    }

    Ok(())
}
