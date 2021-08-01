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
use std::{fs, env};
use dirs::config_dir;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use pancurses;
use toml;

use crate::stdout::init_curses_wcolors;

/// The configuration struct.
///
/// ## Example
/// 
/// ```rust
/// config::Config {
///     input: config::Input {
///         name: String::from("SynPS/2 Synaptics TouchPad")
///     },
///     display: Some(config::Display {
///         display: Some(String::from(":0")),
///         screen: 0
///     })
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub input: Input,
    pub display: Option<Display>
}

/// The input configuration struct.
/// 
/// ## Example
/// 
/// ```rust
/// config::Input {
///     name: String::from("SynPS/2 Synaptics TouchPad")
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Input {
    pub name: String
}

/// The display configuration struct.
/// 
/// ## Example
/// 
/// ```rust
/// config::Display {
///     display: Some(String::from(":0")),
///     screen: 0
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Display {
    pub display: Option<String>,
    pub screen: u8
}

/// Returns the configuration in the config file as struct.
/// Config file is located at `$CONFIG_DIR/egawari/egawari.toml`
/// 
/// ## Example
/// 
/// ```rust
/// let conf: config::Config = config::get_config().unwrap();
/// ```
pub fn get_config() -> Result<Config> {
    let file = config_dir().unwrap().join("egawari").join("egawari.toml");

    match fs::read_to_string(file.as_path()) {
        Ok(s) => {
            let config: Config = toml::from_str(s.as_str()).context("Couldn't parse the config file.")?;
            Ok(config)
        },
        Err(_) => {
            let config = match env::consts::OS {
                "linux" => Config {
                    input: Input {
                        name: String::new()
                    },
                    display: Some(Display {
                        display: Some(":0".to_string()),
                        screen: 0
                    })
                },
                _ => Config {
                    input: Input {
                        name: String::new()
                    },
                    display: None
                }
            };

            Ok(config)
        }
    }
}

/// Saves the given config struct to the config file.
/// Config file is located at `$CONFIG_DIR/egawari/egawari.toml`
/// 
/// ## Example
/// 
/// ```rust
/// let conf = config::Config {
///     input: config::Input {
///         name: String::new()
///     },
///     display: None
/// };
/// 
/// config::save_config(conf).unwrap();
/// ```
pub fn save_config(config: Config) -> Result<()> {
    let dir = config_dir().unwrap().join("egawari");
    let file = dir.join("egawari.toml");
    let raw = toml::to_string_pretty(&config).context("Couldn't convert the config to TOML.")?;
    
    fs::create_dir_all(dir.as_path()).context("Couldn't create the config directory.")?;
    fs::write(file.as_path(), raw).context("Couldn't write to the config file.")?;

    Ok(())
}

/// Edit the config keys and values interactively using curses.
/// Automatically loads and saves the config.
/// 
/// ## Example
/// 
/// ```rust
/// config::config_interactive();
/// ```
pub fn config_interactive() -> Result<()> {
    let window = init_curses_wcolors();
    window.keypad(true);
    pancurses::noecho();

    // Available keys and y locations on window (from top).
    let mut keys: Vec<(&str, i32)> = vec![
        ("input.auto_setup", 3),
        ("input.name", 4)
    ];
    let mut cur = 0;

    let mut edit = false;
    let conf = get_config()?;

    colwln!(&window, "---===egawari=Configuration===---");
    window.printw("\n");

    // Input section
    colwln!(&window, r"=\[Input\]=");
    logwln!(&window, "{{Automatic Setup}}");
    logwln!(&window, "Name = \x1b[0;39m{:?}", conf.input.name);
    window.printw("\n");

    // Display section
    if let Some(display) = &conf.display {
        colwln!(&window, r"=\[Display\]=");
        keys.push(("display.auto_setup", 7));
        logwln!(&window, "{{Automatic Setup}}");

        if let Some(dp) = &display.display {
            keys.push(("display.display", 8));
            logwln!(&window, "Display = \x1b[0;39m{:?}", dp);
        }

        keys.push(("display.screen", 9));
        logwln!(&window, "Screen = \x1b[0;39m{:?}", display.screen);
        window.printw("\n");
    }

    colwln!(&window, "---===========================---");
    window.printw("\n");
    logwln!(&window, r#"Use "Up" and "Down" to move, "Space" to edit and "Enter" to exit."#);

    loop {
        window.attron(pancurses::A_BOLD);
        window.attron(pancurses::ColorPair(6));
        for key in &keys {
            window.mvaddstr(key.1, 0, " => ");
        }

        window.attroff(pancurses::A_BOLD);
        window.attron(pancurses::ColorPair(5));
        window.mvaddstr(keys[cur].1, 0, " >> ");
        window.attron(pancurses::A_BOLD);
        window.mv(0, 0);
        window.refresh();

        // TODO: Implement edit mode.

        match window.getch() {
            Some(pancurses::Input::KeyEnter) | Some(pancurses::Input::Character('\n')) => {
                if !edit {
                    break;
                } else {
                    edit = false;
                }
            },
            Some(pancurses::Input::Character(' ')) => {
                if !edit {
                    if keys[cur].0.split(".").collect::<Vec<&str>>()[1] == "auto_setup" {
                        // TODO: Initialize auto setup.
                    } else {
                        edit = true;
                    }
                }
            },
            Some(pancurses::Input::KeyUp) => {
                if !edit {
                    if cur == 0 {
                        cur = keys.len() - 1;
                    } else {
                        cur -= 1;
                    }
                }
            },
            Some(pancurses::Input::KeyDown) => {
                if !edit {
                    if cur == keys.len() - 1 {
                        cur = 0;
                    } else {
                        cur += 1;
                    }
                }
            },
            _ => ()
        }

        window.refresh();
    }
    
    pancurses::endwin();
    save_config(conf)?;
    successln!("Successfully saved the configuration.");
    Ok(())
}
