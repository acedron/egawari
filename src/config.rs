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
pub fn save_config(config: &Config) -> Result<()> {
    let dir = config_dir().unwrap().join("egawari");
    let file = dir.join("egawari.toml");
    let raw = toml::to_string_pretty(config).context("Couldn't convert the config to TOML.")?;
    
    fs::create_dir_all(dir.as_path()).context("Couldn't create the config directory.")?;
    fs::write(file.as_path(), raw).context("Couldn't write to the config file.")?;

    Ok(())
}

/// The behaviour of the config key.
#[derive(PartialEq, Eq)]
enum ConfigKeyType {
    Button,
    String,
    Number
}

/// The pointer types of the config keys.
#[derive(Debug)]
enum ConfigKeyPointer {
    String(*mut String),
    Number(*mut u8)
}

/// Information about the config key.
/// 
/// ## Example
/// 
/// ```rust
/// let conf: &mut Config = &mut get_config()?;
/// ConfigKey {
///     key_type: ConfigKeyType::String,
///     ptr: Some(ConfigKeyPointer::String(&mut conf.input.name)),
///     name: "Input Name",
///     ypos: -1
/// }
/// ```
struct ConfigKey<'a> {
    key_type: ConfigKeyType,
    ptr: Option<ConfigKeyPointer>,
    name: &'a str,
    ypos: i32
}

impl ConfigKey<'_> {
    fn val_xpos(&self) -> i32 {
        format!(" => {} = ", self.name).len() as i32
    }
}

/// Config section.
/// 
/// ## Example
/// 
/// ```rust
/// let conf: &mut Config = &mut get_config()?;
/// 
/// ConfigKeySection {
///     name: "Input",
///     keys: vec![
///         ConfigKey {
///             key_type: ConfigKeyType::String,
///             ptr: Some(ConfigKeyPointer::String(&mut conf.input.name)),
///             name: "Input Name",
///             ypos: -1
///         }
///     ]
/// }
/// ```
struct ConfigKeySection<'a> {
    name: &'a str,
    keys: Vec<ConfigKey<'a>>
}

/// The location of the config key.
/// 
/// ## Example
/// 
/// ```rust
/// ConfigKeyLocation {
///     section: 0,
///     key: 0
/// }
/// ```
struct ConfigKeyLocation {
    section: usize,
    key: usize
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
    let conf: &mut Config = &mut get_config()?;
    let mut key_sections: Vec<ConfigKeySection> = vec![
        ConfigKeySection {
            name: "Input",
            keys: vec![
                ConfigKey {
                    key_type: ConfigKeyType::Button,
                    ptr: None,
                    name: "Automatic Setup",
                    ypos: -1
                },
                ConfigKey {
                    key_type: ConfigKeyType::String,
                    ptr: Some(ConfigKeyPointer::String(&mut conf.input.name)),
                    name: "Name",
                    ypos: -1
                }
            ]
        }
    ];

    if let Some(display) = &mut conf.display {
        let mut arr: Vec<ConfigKey> = vec![
            ConfigKey {
                key_type: ConfigKeyType::Button,
                ptr: None,
                name: "Automatic Setup",
                ypos: -1
            }
        ];

        if let Some(dp) = &mut display.display {
            arr.push(ConfigKey {
                key_type: ConfigKeyType::String,
                ptr: Some(ConfigKeyPointer::String(dp)),
                name: "Display",
                ypos: -1
            });
        }

        arr.push(ConfigKey {
            key_type: ConfigKeyType::Number,
            ptr: Some(ConfigKeyPointer::Number(&mut display.screen)),
            name: "Screen",
            ypos: -1
        });

        key_sections.push(ConfigKeySection {
            name: "Display",
            keys: arr
        });
    }

    let window = init_curses_wcolors();
    window.keypad(true);
    pancurses::noecho();
    colwln!(&window, "---===egawari=Configuration===---");

    let mut cur = ConfigKeyLocation {
        section: 0,
        key: 0
    };
    let mut edit = false;

    let mut line_buf = 1;
    for section in &mut key_sections {
        window.printw("\n");
        colwln!(&window, r"=\[{}\]=", section.name);
        line_buf += 2;

        for mut key in &mut section.keys {
            if key.key_type == ConfigKeyType::Button {
                colwln!(&window, " => \x1b[0;39m{{{{{}}}}}", key.name);
            } else {
                match key.ptr.as_ref().unwrap() {
                    ConfigKeyPointer::String(val) => unsafe {
                        colwln!(&window, " => {} = \x1b[0;39m{:?}", key.name, **val);
                    },
                    ConfigKeyPointer::Number(val) => unsafe {
                        colwln!(&window, " => {} = \x1b[0;39m{:?}", key.name, **val);
                    }
                }
            }

            key.ypos = line_buf;
            line_buf += 1;
        }
    }

    window.printw("\n");
    colwln!(&window, "---===========================---");
    window.printw("\n");
    logwln!(&window, r#"Use "Up" and "Down" to move, "Space" to edit and "Enter" to exit."#);

    let mut buf = String::new();
    loop {
        let cur_key = &key_sections[cur.section].keys[cur.key];
        let mut cur_val_str = String::new();
        if cur_key.key_type != ConfigKeyType::Button {
            cur_val_str = match cur_key.ptr.as_ref().unwrap() {
                ConfigKeyPointer::String(val) => unsafe {
                    format!("{}", &**val)
                },
                ConfigKeyPointer::Number(val) => unsafe {
                    format!("{}", **val)
                }
            };
        }

        if !edit {
            for section in &key_sections {
                for key in &section.keys {
                    colwmvaddstr!(&window, key.ypos, 0, " => ");
                }
            }

            window.attroff(pancurses::A_BOLD);
            window.attron(pancurses::ColorPair(5));
            window.mvaddstr(cur_key.ypos, 0, " >> ");
            window.attron(pancurses::A_BOLD);

            window.mv(0, 0);
            window.refresh();
        }

        match window.getch() {
            Some(pancurses::Input::Character('\u{1b}')) => {
                edit = false;
            },
            Some(pancurses::Input::KeyEnter) | Some(pancurses::Input::Character('\n')) => {
                if !edit {
                    break;
                } else {
                    match cur_key.ptr.as_ref().unwrap() {
                        ConfigKeyPointer::String(ptr) => unsafe {
                            **ptr = buf.clone();
                        },
                        ConfigKeyPointer::Number(ptr) => unsafe {
                            let digits: String = buf.clone().chars().filter(|c| c.is_digit(10)).collect();
                            **ptr = digits.parse::<u8>().unwrap();
                        }
                    }
                    edit = false;
                }
            },
            Some(pancurses::Input::Character(' ')) => {
                if !edit {
                    if cur_key.key_type == ConfigKeyType::Button {
                        // TODO: Initialize auto setup.
                    } else {
                        edit = true;
                        buf = cur_val_str.clone();
                    }
                } else {
                    buf.push(' ');
                }
            },
            Some(pancurses::Input::KeyUp) => {
                if !edit {
                    if cur.key == 0 {
                        if cur.section == 0 {
                            cur.section = key_sections.len() - 1;
                        } else {
                            cur.section -= 1;
                        }

                        cur.key = key_sections[cur.section].keys.len() - 1;
                    } else {
                        cur.key -= 1;
                    }
                }
            },
            Some(pancurses::Input::KeyDown) => {
                if !edit {
                    if cur.key == key_sections[cur.section].keys.len() - 1 {
                        if cur.section == key_sections.len() - 1 {
                            cur.section = 0;
                        } else {
                            cur.section += 1;
                        }

                        cur.key = 0;
                    } else {
                        cur.key += 1;
                    }
                }
            },
            Some(pancurses::Input::KeyBackspace) | Some(pancurses::Input::Character('\u{7f}')) => {
                buf.pop();
            },
            Some(pancurses::Input::Character(c)) => {
                buf.push(c);
            }
            _ => ()
        }

        if edit {
            window.mv(cur_key.ypos, cur_key.val_xpos());
            window.clrtoeol();
            match cur_key.key_type {
                ConfigKeyType::String => {
                    colwaddstr!(&window, "\x1b[0;39m{:?}", &buf);
                    window.mv(window.get_cur_y(), window.get_cur_x() - 1);
                },
                _ => {
                    colwaddstr!(&window, "\x1b[0;39m{}", &buf);
                }
            }
        }
    }

    pancurses::endwin();
    save_config(conf)?;
    successln!("Successfully saved the configuration.");
    Ok(())
}
