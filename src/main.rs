/****************************************************************************
** egawari - Makes your touchpad work like a graphics tablet.
** Copyright (C) 2021  acedron <acedrons@yahoo.co.jp>
**
** This program is free software: you can redistribute it and/or modify
** it under the terms of the GNU General Public License as published by
** the Free Software Foundation, either version 3 of the License, or
** (at your option) any later version.
**
** This program is distributed in the hope that it will be useful,
** but WITHOUT ANY WARRANTY; without even the implied warranty of
** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
** GNU General Public License for more details.
**
** You should have received a copy of the GNU General Public License
** along with this program.  If not, see <https://www.gnu.org/licenses/>.
****************************************************************************/
extern crate serde;
extern crate fancy_regex;
extern crate pancurses;
extern crate anyhow;
extern crate dirs;
extern crate toml;

use std::env;
use std::process;
use anyhow::Result;

#[macro_use]
pub mod stdout;
pub mod config;

#[cfg(test)]
mod tests;

fn main() -> Result<()> {
    let mut args: Vec<String> = vec![];
    let mut opts: Vec<String> = vec![];

    let raw_args: Vec<String> = env::args().collect();
    for raw in &raw_args[1..] {
        if raw.starts_with("-") {
            if raw.starts_with("--") {
                opts.push(raw[2..].to_string());
                continue;
            }

            for c in raw[1..].chars() {
                opts.push(c.to_string());
            }
            continue;
        }

        args.push(raw.to_string());
    }

    if args.len() < 1 {
        errln!("No command provided.");
        logln!("See: \x1b[0;39megawari help");
        process::exit(1);
    }

    let command = &args[0].to_string();
    args.remove(0);

    match command.as_str() {
        "help" => {
            colln!("---===egawari===---");
            logln!("Makes your touchpad work like a graphics tablet.");
            println!();
            colln!("---====Usage====---");
            logln!("egawari [options] <command> [arguments]");
            println!();
            colln!("---===Commands==---");
            logln!("help => Shows this text.");
            logln!("config => Edits or shows the egawari configuration interactively.");
            println!();
            colln!("---=============---");
        },
        "config" => {
            config::config_interactive()?;
        },
        _ => {
            errln!("Unknown command: \x1b[0;39m{}", command);
            logln!("See: \x1b[0;39megawari help");
            process::exit(1);
        }
    }

    Ok(())
}
