#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use clap::{value_parser, Arg, ArgMatches, Command};
use colored::{Color, Colorize};
use std::error::Error;
use std::fs;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;

struct Frame {
    frame_variants: [char; 8],
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            frame_variants: ['┌', '┐', '─', '─', '│', '│', '└', '┘'],
        }
    }
}

impl Frame {
    fn duble() -> Self {
        Self {
            frame_variants: ['╔', '╗', '═', '═', '║', '║', '╚', '╝'],
        }
    }

    fn round() -> Self {
        Self {
            frame_variants: ['╭', '╮', '─', '─', '│', '│', '╰', '╯'],
        }
    }
    fn heavy() -> Self {
        Self {
            frame_variants: ['┏', '┓', '━', '━', '┃', '┃', '┗', '┛'],
        }
    }

    fn torn() -> Self {
        Self {
            frame_variants: ['', '', '', '', '¦', '¦', '', ''],
        }
    }

    fn frame_variants(app: &ArgMatches) -> Self {
        if let Some(variants) = app.get_one::<String>("frame") {
            match variants.as_str() {
                "duble" => Self::duble(),
                "round" => Self::round(),
                "heavy" => Self::heavy(),
                "torn" => Self::torn(),
                _ => Self::default(),
            }
        } else {
            Self::default()
        }
    }
    fn frame_custom(&mut self, app: &ArgMatches) -> &Self {
        self.set_hor_upper(*app.get_one("hor-upper").unwrap_or(&self.get_hor_top()));
        self.set_hor_lower(*app.get_one("hor-lower").unwrap_or(&self.get_hor_buttom()));
        self.set_vert_left(*app.get_one("vert-left").unwrap_or(&self.get_vert_left()));
        self.set_vert_right(*app.get_one("vert-right").unwrap_or(&self.get_vert_right()));
        self.set_top_left(*app.get_one("top-left").unwrap_or(&self.get_top_left()));
        self.set_top_right(*app.get_one("top-right").unwrap_or(&self.get_top_right()));
        self.set_bottom_left(
            *app.get_one("bottom-left")
                .unwrap_or(&self.get_bottom_left()),
        );
        self.set_bottom_right(
            *app.get_one("bottom-right")
                .unwrap_or(&self.get_bottom_right()),
        );
        self
    }

    fn get_top_left(&self) -> char {
        self.frame_variants[0]
    }
    fn get_top_right(&self) -> char {
        self.frame_variants[1]
    }
    fn get_hor_top(&self) -> char {
        self.frame_variants[2]
    }
    fn get_hor_buttom(&self) -> char {
        self.frame_variants[3]
    }
    fn get_vert_left(&self) -> char {
        self.frame_variants[4]
    }
    fn get_vert_right(&self) -> char {
        self.frame_variants[5]
    }
    fn get_bottom_left(&self) -> char {
        self.frame_variants[6]
    }
    fn get_bottom_right(&self) -> char {
        self.frame_variants[7]
    }

    fn set_top_left(&mut self, c: char) -> &Self {
        self.frame_variants[0] = c;
        self
    }
    fn set_top_right(&mut self, c: char) -> &Self {
        self.frame_variants[1] = c;
        self
    }
    fn set_hor_upper(&mut self, c: char) -> &Self {
        self.frame_variants[2] = c;
        self
    }
    fn set_hor_lower(&mut self, c: char) -> &Self {
        self.frame_variants[3] = c;
        self
    }
    fn set_vert_left(&mut self, c: char) -> &Self {
        self.frame_variants[4] = c;
        self
    }
    fn set_vert_right(&mut self, c: char) -> &Self {
        self.frame_variants[5] = c;
        self
    }
    fn set_bottom_left(&mut self, c: char) -> &Self {
        self.frame_variants[6] = c;
        self
    }
    fn set_bottom_right(&mut self, c: char) -> &Self {
        self.frame_variants[7] = c;
        self
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = app_commands();
    let mut frame = Frame::frame_variants(&app);
    frame.frame_custom(&app);
    let mut buff = String::new();

    match app.get_one::<PathBuf>("file") {
        Some(path) => buff = fs::read_to_string(path)?,
        None => {
            stdin().read_to_string(&mut buff)?;
        }
    }

    let expand = *app.get_one("expand").unwrap_or(&0);
    let mut max_line_len: usize = match buff.lines().map(|line| line.chars().count()).max() {
        Some(value) => value + expand * 2,
        None => return Ok(()),
    };

    let color = if let Some(color) = app.get_one::<String>("color") {
        match color.as_str() {
            "red" => "red",
            "green" => "green",
            "yellow" => "yellow",
            "blue" => "blue",
            "magenta" => "magenta",
            "cyan" => "cyan",
            "white" => "white",
            _ => "black",
        }
    } else {
        "black"
    };

    let hor_top_line = String::from(frame.get_hor_top())
        .repeat(max_line_len)
        .color(color);
    let hor_buttom_line = String::from(frame.get_hor_buttom())
        .repeat(max_line_len)
        .color(color);
    let vrt_left = String::from(frame.get_vert_left()).color(color);
    let vrt_right = String::from(frame.get_vert_right()).color(color);

    let hor_line = format!(
        "{vrt_left}{hor_line}{vrt_right}\n",
        hor_line = " ".repeat(max_line_len)
    )
    .repeat(expand);

    let buff_line = format!(
        "{top_left}{hor_top_line}{top_right}\n{hor_line}{buff}{hor_line}{bottom_left}{hor_buttom_line}{bottom_right}\n",
        top_left = frame.get_top_left().to_string().color(color),
        top_right = frame.get_top_right().to_string().color(color),
        bottom_left = frame.get_bottom_left().to_string().color(color),
        bottom_right = frame.get_bottom_right().to_string().color(color),
    );

    for current_line in buff_line.lines() {
        let current_line_len = current_line.chars().count();

        let out_line = if max_line_len < current_line_len {
            format!("{current_line}\n")
        } else {
            let mut left = 0;
            let line_len = max_line_len - current_line_len;

            if let Some(aling) = app.get_one::<String>("alignment") {
                match aling.as_str() {
                    "centr" => left = line_len / 2,
                    "right" => left = line_len,
                    _ => left = 0,
                }
            }

            format!(
                "{vrt_left}{left_fill}{current_line}{right_fill}{vrt_right}\n",
                left_fill = " ".repeat(left),
                right_fill = " ".repeat(line_len - left),
            )
        };

        std::io::stdout().write_all(out_line.as_bytes())?;
    }

    Ok(())
}

fn app_commands() -> ArgMatches {
    Command::new("frame")
        .about("    frame for text")
        .author("    by PIC16F877ccs")
        .args_override_self(true)
        .arg(
            Arg::new("frame")
                .short('f')
                .long("frame")
                .num_args(1)
                .value_name("VARIANTS")
                .help("Text frame variants")
                .value_parser(["duble", "round", "heavy", "torn"])
                //.hide_possible_values(true)
                .required(false),
        )
        .arg(
            Arg::new("alignment")
                .short('a')
                .long("align")
                .num_args(1)
                .value_name("ALINGMENT")
                .help("Alingment text in frame")
                .value_parser(["centr", "right"])
                //.hide_possible_values(true)
                .required(false),
        )
        .arg(
            Arg::new("expand")
                //.short('e')
                .long("expand")
                .num_args(1)
                .value_name("NUMPER")
                .value_parser(value_parser!(usize))
                .help("Expand the frame")
                .required(false),
        )
        .arg(
            Arg::new("top-left")
                .short('S')
                .long("top-left")
                .num_args(1)
                .value_name("CHARACTER")
                .value_parser(value_parser!(char))
                .help("Sets the top left corner")
                .required(false),
        )
        .arg(
            Arg::new("top-right")
                .short('E')
                .long("top-right")
                .help("Sets the top right corner")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("hor-upper")
                .short('H')
                .long("hor-upper")
                .help("Sets the view of horizontal upper line")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("hor-lower")
                .long("hor-lower")
                .help("Sets the view of horizontal lower line")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("vert-left")
                .short('V')
                .long("vert-left")
                .help("Sets the view of vertical left line")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("vert-right")
                .long("vert-right")
                .help("Sets the view of vertical right line")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("bottom-left")
                .short('s')
                .long("bottom-left")
                .value_name("CHARACTER")
                .help("Sets the bottom left corner")
                .value_parser(value_parser!(char))
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("bottom-right")
                .short('e')
                .long("bottom-right")
                .value_name("CHARACTER")
                .help("Sets the bottom right corner")
                .value_parser(value_parser!(char))
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("color")
                .short('c')
                .long("color")
                .value_name("COLOR")
                .help("Displays a text frame in the specified color")
                .value_parser(["red", "green", "yellow", "blue", "magenta", "cyan", "white"])
                .hide_default_value(true)
                .hide_possible_values(true)
                .required(false),
        )
        .arg(
            Arg::new("file")
                .value_parser(value_parser!(PathBuf))
                .index(1),
        )
        .get_matches()
}
