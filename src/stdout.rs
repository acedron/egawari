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
use fancy_regex::{Regex, Captures};
use pancurses;

/// Colors the string using ANSI escape codes according to some rules.
/// 
/// ## Example
/// 
/// ```rust
/// println!("{}", stdout::color_str_escape(" => 'Hi!'"));
/// ```
pub fn color_str_escape(string: &str) -> String {
    let mut result = string.to_string();

    // Basic regular expressions and replacements.
    let rules: Vec<(&str, &str)> = vec![
        // Characters
        (r#"[+]+"#, "\x1b[1;36m${0}\x1b[1;39m"),
        (r#"[:/=]+"#, "\x1b[1;32m${0}\x1b[1;39m"),
        (r#"[,\-|]+"#, "\x1b[0;32m${0}\x1b[1;39m"),
        (r#"[*]+"#, "\x1b[1;31m${0}\x1b[1;39m"),
        (r#"[{}]+"#, "\x1b[1;35m${0}\x1b[1;39m"),

        // Exceptions
        ("\x1b\\[\\d*;\\d+m=\x1b\\[\\d*;\\d+m>", "\x1b[1;36m=>\x1b[1;39m")
    ];
    for tuple in rules {
        let re = Regex::new(tuple.0).unwrap();
        result = re.replace_all(result.as_str(), tuple.1).to_string();
    }
    
    // The surrounding characters rules.
    let surrounding: Vec<(&str, &str)> = vec![
        (r#"([\[])(?:(?=(\\?))\2.)*?([\]])"#, "\x1b[1;32m"),
        (r#"([\(])(?:(?=(\\?))\2.)*?([\)])"#, "\x1b[0;32m"),
        (r#"(["])(?:(?=(\\?))\2.)*?(["])"#, "\x1b[1;32m"),
        (r#"(['])(?:(?=(\\?))\2.)*?(['])"#, "\x1b[0;32m"),
        (r#"([<])(?:(?=(\\?))\2.)*?([>])"#, "\x1b[1;32m")
    ];
    // Color the surrounding colors and remove the color between them.
    for tuple in surrounding {
        let re = Regex::new(tuple.0).unwrap();
        result = re.replace_all(result.as_str(), |caps: &Captures| {
            let buf = &mut caps[0].chars();
            buf.next();
            buf.next_back();
            format!("{}{}\x1b[0;39m{}{}{}\x1b[1;39m", tuple.1, &caps[1], buf.as_str().replace("\x1b[1;39m", "\x1b[0;39m"), tuple.1, &caps[3])
        }).to_string();
    }

    // The surrounding character escapes.
    let sur_escape: Vec<(&str, &str)> = vec![
        (r#"\\([\[\]"<>])"#, "\x1b[1;32m${1}\x1b[1;39m"),
        (r#"\\([\(\)'])"#, "\x1b[0;32m${1}\x1b[1;39m")
    ];
    // Delete the escape character if the surrounding character was escaped.
    for tuple in sur_escape {
        let re = Regex::new(tuple.0).unwrap();
        result = re.replace_all(result.as_str(), tuple.1).to_string();
    }

    result
}

/// Initializes a curses window with colors using `pancurses`.
/// 
/// ## Example
/// 
/// ```rust
/// let window = stdout::init_curses_wcolors();
/// ```
pub fn init_curses_wcolors() -> pancurses::Window {
    let window = pancurses::initscr();

    pancurses::use_default_colors();
    pancurses::start_color();
    pancurses::init_pair(0, pancurses::COLOR_BLACK, -1);
    pancurses::init_pair(1, pancurses::COLOR_RED, -1);
    pancurses::init_pair(2, pancurses::COLOR_GREEN, -1);
    pancurses::init_pair(3, pancurses::COLOR_YELLOW, -1);
    pancurses::init_pair(4, pancurses::COLOR_BLUE, -1);
    pancurses::init_pair(5, pancurses::COLOR_MAGENTA, -1);
    pancurses::init_pair(6, pancurses::COLOR_CYAN, -1);
    pancurses::init_pair(7, pancurses::COLOR_WHITE, -1);
    pancurses::init_pair(9, -1, -1);

    window.attron(pancurses::A_COLOR);
    window.attron(pancurses::ColorPair(9));
    window
}

/// Converts the ANSI escape colored string to a sequence of
/// curses attributes and `printw` methods and runs them.
/// 
/// ## Example
/// 
/// ```rust
/// extern crate pancurses;
/// 
/// let window = pancurses::initscr();
/// stdout::escaped_to_printw(&window, "\x1b[1;32mHi!")
/// ```
pub fn escaped_to_printw(window: &pancurses::Window, escaped: String) {
    window.attron(pancurses::ColorPair(9));
    window.attron(pancurses::A_BOLD);

    let re = Regex::new(r"(?<=\[)\d*;\d+(?=m)").unwrap();
    for s in escaped.split("\x1b") {
        let mat = re.find(s).unwrap();
        let mut chars = s.chars();

        if let Some(mat) = mat {
            if mat.start() > 1 {
                continue;
            }

            let colors: Vec<&str> = mat.as_str().split(";").collect();
            match colors[0] {
                "1" => { window.attron(pancurses::A_BOLD); },
                _ => { window.attroff(pancurses::A_BOLD); }
            }

            let mut color_chars = colors[1].chars();
            color_chars.next();
            let color = color_chars.as_str();
            if color != "" {
                window.attron(pancurses::ColorPair(color.parse::<u8>().unwrap()));
            }
            
            chars.nth(mat.end());
        }

        window.printw(chars.as_str());
    }

    window.attron(pancurses::ColorPair(9));
    window.attroff(pancurses::A_BOLD);
}

//
// Macro rules that automatically prints to stdout after
// coloring the string using ANSI escape sequences using
// `color_str_escape` function above.
//

#[macro_export]
macro_rules! col {
    ($fmt:expr) => ({
        print!("\x1b[1;39m{}\x1b[;m", $crate::stdout::color_str_escape($fmt));
    });

    ($fmt:expr, $($arg:tt)*) => ({
        print!("\x1b[1;39m{}\x1b[;m", $crate::stdout::color_str_escape(format!($fmt, $($arg)*).as_str()));
    });
}

#[macro_export]
macro_rules! colln {
    ($fmt:expr) => ({
        col!(format!("{}\n", $fmt).as_str());
    });

    ($fmt:expr, $($arg:tt)*) => ({
        col!(format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str());
    });
}

#[macro_export]
macro_rules! log {
    ($fmt:expr) => ({
        print!(" \x1b[1;36m=>\x1b[1;39m {}\x1b[;m", $crate::stdout::color_str_escape($fmt));
    });

    ($fmt:expr, $($arg:tt)*) => ({
        print!("\x1b[1;36m=>\x1b[1;39m {}\x1b[;m", $crate::stdout::color_str_escape(format!($fmt, $($arg)*).as_str()));
    });
}

#[macro_export]
macro_rules! logln {
    ($fmt:expr) => ({
        log!(format!("{}\n", $fmt).as_str());
    });

    ($fmt:expr, $($arg:tt)*) => ({
        log!(format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str());
    });
}

#[macro_export]
macro_rules! err {
    ($fmt:expr) => ({
        print!(" \x1b[1;31m=>\x1b[1;39m {}\x1b[;m", $crate::stdout::color_str_escape($fmt));
    });

    ($fmt:expr, $($arg:tt)*) => ({
        print!("\x1b[1;31m=>\x1b[1;39m {}\x1b[;m", $crate::stdout::color_str_escape(format!($fmt, $($arg)*).as_str()));
    });
}

#[macro_export]
macro_rules! errln {
    ($fmt:expr) => ({
        err!(format!("{}\n", $fmt).as_str());
    });

    ($fmt:expr, $($arg:tt)*) => ({
        err!(format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str());
    });
}

#[macro_export]
macro_rules! success {
    ($fmt:expr) => ({
        print!(" \x1b[1;32m=>\x1b[1;39m {}\x1b[;m", $crate::stdout::color_str_escape($fmt));
    });

    ($fmt:expr, $($arg:tt)*) => ({
        print!("\x1b[1;32m=>\x1b[1;39m {}\x1b[;m", $crate::stdout::color_str_escape(format!($fmt, $($arg)*).as_str()));
    });
}

#[macro_export]
macro_rules! successln {
    ($fmt:expr) => ({
        success!(format!("{}\n", $fmt).as_str());
    });

    ($fmt:expr, $($arg:tt)*) => ({
        success!(format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str());
    });
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr) => ({
        print!(" \x1b[1;33m=>\x1b[1;39m {}\x1b[;m", $crate::stdout::color_str_escape($fmt));
    });

    ($fmt:expr, $($arg:tt)*) => ({
        print!("\x1b[1;33m=>\x1b[1;39m {}\x1b[;m", $crate::stdout::color_str_escape(format!($fmt, $($arg)*).as_str()));
    });
}

#[macro_export]
macro_rules! warnln {
    ($fmt:expr) => ({
        warn!(format!("{}\n", $fmt).as_str());
    });

    ($fmt:expr, $($arg:tt)*) => ({
        warn!(format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str());
    });
}

//
// Macro rules that automatically print to a curses window after
// coloring the string using ColorPair attribute sequences using
// `color_str_escape` and `escaped_to_printw` function above.
//

#[macro_export]
macro_rules! colw {
    ($window:expr, $fmt:expr) => ({
        $crate::stdout::escaped_to_printw($window, $crate::stdout::color_str_escape($fmt));
    });

    ($window:expr, $fmt:expr, $($arg:tt)*) => ({
        $crate::stdout::escaped_to_printw($window, $crate::stdout::color_str_escape(format!($fmt, $($arg)*).as_str()));
    });
}

#[macro_export]
macro_rules! colwln {
    ($window:expr, $fmt:expr) => ({
        colw!($window, format!("{}\n", $fmt).as_str());
    });

    ($window:expr, $fmt:expr, $($arg:tt)*) => ({
        colw!($window, format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str());
    });
}

#[macro_export]
macro_rules! logw {
    ($window:expr, $fmt:expr) => ({
        $crate::stdout::escaped_to_printw($window, format!(" \x1b[1;36m=>\x1b[1;39m {}", $crate::stdout::color_str_escape($fmt)));
    });

    ($window:expr, $fmt:expr, $($arg:tt)*) => ({
        $crate::stdout::escaped_to_printw($window, format!(" \x1b[1;36m=>\x1b[1;39m {}", $crate::stdout::color_str_escape(format!($fmt, $($arg)*).as_str())));
    });
}

#[macro_export]
macro_rules! logwln {
    ($window:expr, $fmt:expr) => ({
        logw!($window, format!("{}\n", $fmt).as_str());
    });

    ($window:expr, $fmt:expr, $($arg:tt)*) => ({
        logw!($window, format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str());
    });
}

#[macro_export]
macro_rules! errw {
    ($window:expr, $fmt:expr) => ({
        $crate::stdout::escaped_to_printw($window, format!(" \x1b[1;31m=>\x1b[1;39m {}", $crate::stdout::color_str_escape($fmt)));
    });

    ($window:expr, $fmt:expr, $($arg:tt)*) => ({
        $crate::stdout::escaped_to_printw($window, format!(" \x1b[1;31m=>\x1b[1;39m {}", $crate::stdout::color_str_escape(format!($fmt, $($arg)*).as_str())));
    });
}

#[macro_export]
macro_rules! errwln {
    ($window:expr, $fmt:expr) => ({
        errw!($window, format!("{}\n", $fmt).as_str());
    });

    ($window:expr, $fmt:expr, $($arg:tt)*) => ({
        errw!($window, format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str());
    });
}

#[macro_export]
macro_rules! successw {
    ($window:expr, $fmt:expr) => ({
        $crate::stdout::escaped_to_printw($window, format!(" \x1b[1;32m=>\x1b[1;39m {}", $crate::stdout::color_str_escape($fmt)));
    });

    ($window:expr, $fmt:expr, $($arg:tt)*) => ({
        $crate::stdout::escaped_to_printw($window, format!(" \x1b[1;32m=>\x1b[1;39m {}", $crate::stdout::color_str_escape(format!($fmt, $($arg)*).as_str())));
    });
}

#[macro_export]
macro_rules! successwln {
    ($window:expr, $fmt:expr) => ({
        successw!($window, format!("{}\n", $fmt).as_str());
    });

    ($window:expr, $fmt:expr, $($arg:tt)*) => ({
        successw!($window, format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str());
    });
}

#[macro_export]
macro_rules! warnw {
    ($window:expr, $fmt:expr) => ({
        $crate::stdout::escaped_to_printw($window, format!(" \x1b[1;33m=>\x1b[1;39m {}", $crate::stdout::color_str_escape($fmt)));
    });

    ($window:expr, $fmt:expr, $($arg:tt)*) => ({
        $crate::stdout::escaped_to_printw($window, format!(" \x1b[1;33m=>\x1b[1;39m {}", $crate::stdout::color_str_escape(format!($fmt, $($arg)*).as_str())));
    });
}

#[macro_export]
macro_rules! warnwln {
    ($window:expr, $fmt:expr) => ({
        warnw!($window, format!("{}\n", $fmt).as_str());
    });

    ($window:expr, $fmt:expr, $($arg:tt)*) => ({
        warnw!($window, format!("{}\n", format!($fmt, $($arg)*).as_str()).as_str());
    });
}
