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
use fancy_regex::Regex;

pub fn color_string(string: &str) -> String {
    let mut result = string.to_string();
    let rules: Vec<(&str, &str)> = vec![
        (r"[+]", "\x1b[1;36m${0}\x1b[1;37m"),
        (r"[:/=]", "\x1b[1;32m${0}\x1b[1;37m"),
        (r"[,\-|]", "\x1b[0;32m${0}\x1b[1;37m"),
        (r"[*]", "\x1b[1;31m${0}\x1b[1;37m"),
        ("\"([^\"]+)\"", "\x1b[1;32m\"\x1b[0;37m${1}\x1b[1;32m\"\x1b[1;37m"),
        (r"'([^']+)'", "\x1b[0;32m'\x1b[0;37m${1}\x1b[0;32m'\x1b[1;37m"),
        (r"\[([^\[\]]+)\]", "\x1b[1;32m[\x1b[0;37m${1}\x1b[1;32m]\x1b[1;37m"),
        (r"\(([^\(\)]+)\)", "\x1b[0;32m(\x1b[0;37m${1}\x1b[0;32m)\x1b[1;37m"),
        (r"<([^<>]+)>", "\x1b[1;32m<\x1b[0;37m${1}\x1b[0;32m>\x1b[1;37m"),
        ("\x1b\\[.*m=\x1b\\[.*m>", "\x1b[1;36m=>\x1b[1;37m")
    ];

    for tuple in rules {
        let re = Regex::new(tuple.0).unwrap();
        result = re.replace_all(result.as_str(), tuple.1).to_string();
    }

    result
}

#[macro_export]
macro_rules! col {
    ($fmt:expr) => ({ print!("\x1b[1;37m{}", $crate::color::color_string($fmt)); });
    ($fmt:expr, $($arg:tt)*) => ({ print!("\x1b[1;37m{}", $crate::color::color_string(format!($fmt, $($arg)*).as_str())); });
}

#[macro_export]
macro_rules! colln {
    ($fmt:expr) => ({ col!(format!("{}\n", $fmt).as_str()); });
    ($fmt:expr, $($arg:tt)*) => ({ col!(format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str()); });
}

#[macro_export]
macro_rules! log {
    ($fmt:expr) => ({ print!(" \x1b[1;36m=>\x1b[1;37m {}", $crate::color::color_string($fmt)); });
    ($fmt:expr, $($arg:tt)*) => ({ print!("\x1b[1;36m=>\x1b[1;37m {}", $crate::color::color_string(format!($fmt, $($arg)*).as_str())); });
}

#[macro_export]
macro_rules! logln {
    ($fmt:expr) => ({ log!(format!("{}\n", $fmt).as_str()); });
    ($fmt:expr, $($arg:tt)*) => ({ log!(format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str()); });
}

#[macro_export]
macro_rules! err {
    ($fmt:expr) => ({ print!(" \x1b[1;31m=>\x1b[1;37m {}", $crate::color::color_string($fmt)); });
    ($fmt:expr, $($arg:tt)*) => ({ print!("\x1b[1;31m=>\x1b[1;37m {}", $crate::color::color_string(format!($fmt, $($arg)*).as_str())); });
}

#[macro_export]
macro_rules! errln {
    ($fmt:expr) => ({ err!(format!("{}\n", $fmt).as_str()); });
    ($fmt:expr, $($arg:tt)*) => ({ err!(format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str()); });
}

#[macro_export]
macro_rules! success {
    ($fmt:expr) => ({ print!(" \x1b[1;32m=>\x1b[1;37m {}", $crate::color::color_string($fmt)); });
    ($fmt:expr, $($arg:tt)*) => ({ print!("\x1b[1;32m=>\x1b[1;37m {}", $crate::color::color_string(format!($fmt, $($arg)*).as_str())); });
}

#[macro_export]
macro_rules! successln {
    ($fmt:expr) => ({ success!(format!("{}\n", $fmt).as_str()); });
    ($fmt:expr, $($arg:tt)*) => ({ success!(format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str()); });
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr) => ({ print!(" \x1b[1;33m=>\x1b[1;37m {}", $crate::color::color_string($fmt)); });
    ($fmt:expr, $($arg:tt)*) => ({ print!("\x1b[1;33m=>\x1b[1;37m {}", $crate::color::color_string(format!($fmt, $($arg)*).as_str())); });
}

#[macro_export]
macro_rules! warnln {
    ($fmt:expr) => ({ warn!(format!("{}\n", $fmt).as_str()); });
    ($fmt:expr, $($arg:tt)*) => ({ warn!(format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str()); });
}
