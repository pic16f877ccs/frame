#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use clap::{arg, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use colored::{Color, Colorize};
use std::error::Error;
use std::fs;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;

struct Frame {
    frame_variants: [char; 6],
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            frame_variants: ['┌', '┐', '─', '│', '└', '┘'],
        }
    }
}

impl Frame {
    fn duble() -> Self {
        Self {
            frame_variants: ['╔', '╗', '═', '║', '╚', '╝'],
        }
    }

    fn round() -> Self {
        Self {
            frame_variants: ['╭', '╮', '─', '│', '╰', '╯'],
        }
    }

    fn frame_variants(app: &ArgMatches) -> Self {
        if let Some(variants) = app.get_one::<String>("frame") {
            match variants.as_str() {
                "duble" => Self::duble(),
                "round" => Self::round(),
                _ => Self::default(),
            }
        } else {
            Self::default()
        }
    }
    fn frame_custom(&mut self, app: &ArgMatches) -> &Self {
        self.set_horizontal(*app.get_one("horizontal").unwrap_or(&self.get_horizontal()));
        self.set_vertical(*app.get_one("vertical").unwrap_or(&self.get_vertical()));
        self.set_top_left(*app.get_one("top-left").unwrap_or(&self.get_top_left()));
        self.set_top_right(*app.get_one("top-right").unwrap_or(&self.get_top_right()));
        self.set_bottom_left(*app.get_one("bottom-left").unwrap_or(&self.get_bottom_left()));
        self.set_bottom_right(*app.get_one("bottom-right").unwrap_or(&self.get_bottom_right()));
        self
    }

    fn get_top_left(&self) -> char {
        self.frame_variants[0]
    }
    fn get_top_right(&self) -> char {
        self.frame_variants[1]
    }
    fn get_horizontal(&self) -> char {
        self.frame_variants[2]
    }
    fn get_vertical(&self) -> char {
        self.frame_variants[3]
    }
    fn get_bottom_left(&self) -> char {
        self.frame_variants[4]
    }
    fn get_bottom_right(&self) -> char {
        self.frame_variants[5]
    }

    fn set_top_left(&mut self, c: char) -> &Self {
        self.frame_variants[0] = c;
        self
    }
    fn set_top_right(&mut self, c: char) -> &Self {
        self.frame_variants[1] = c;
        self
    }
    fn set_horizontal(&mut self, c: char) -> &Self {
        self.frame_variants[2] = c;
        self
    }
    fn set_vertical(&mut self, c: char) -> &Self {
        self.frame_variants[3] = c;
        self
    }
    fn set_bottom_left(&mut self, c: char) -> &Self {
        self.frame_variants[4] = c;
        self
    }
    fn set_bottom_right(&mut self, c: char) -> &Self {
        self.frame_variants[5] = c;
        self
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = app_commands();
    let mut frame = Frame::frame_variants(&app);
    let mut buff = String::new();
    let mut head_line = String::new();
    frame.frame_custom(&app);

    //frame.set_horizontal(*app.get_one("horizontal").unwrap_or(&frame.get_horizontal()));
    //frame.set_vertical(*app.get_one("vertical").unwrap_or(&frame.get_vertical()));
    //frame.set_top_left(*app.get_one("top-left").unwrap_or(&frame.get_top_left()));
    //frame.set_top_right(*app.get_one("top-right").unwrap_or(&frame.get_top_right()));
    //frame.set_bottom_left(*app.get_one("bottom-left").unwrap_or(&frame.get_bottom_left()));
    //frame.set_bottom_right(*app.get_one("bottom-right").unwrap_or(&frame.get_bottom_right()));

    match app.get_one::<PathBuf>("file") {
        Some(path) => buff = fs::read_to_string(path)?,
        None => {
            stdin().read_to_string(&mut buff)?;
        }
    }

    let max_line_len: usize = match buff.lines().map(|line| line.chars().count()).max() {
        Some(value) => value,
        None => return Ok(()),
    };

    for _ in 0..max_line_len {
        head_line.push(frame.get_horizontal());
    }

    std::io::stdout().write_all(
        format!(
            "{}{head_line}{}\n",
            frame.get_top_left(),
            frame.get_top_right()
        )
        .as_bytes(),
    )?;

    for current_line in buff.lines() {
        let mut current_fill = String::new();

        for _ in 0..(max_line_len - current_line.chars().count()) {
            current_fill.push(' ');
        }

        std::io::stdout().write_all(
            format!(
                "{vrt}{current_line}{current_fill}{vrt}\n",
                vrt = frame.get_vertical()
            )
            .as_bytes(),
        )?;
    }

    std::io::stdout().write_all(
        format!(
            "{}{head_line}{}\n",
            frame.get_bottom_left(),
            frame.get_bottom_right()
        )
        .as_bytes(),
    )?;

    //println!("In file: {:?}", app.get_one::<PathBuf>("file").unwrap());
    //println!("Max line length: {}", max_line_len);
    //println!("Long flag --frame: {}", app.get_flag("frame"));
    //println!("Print path: {}", buff);
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
                //.default_value("default")
                //.value_parser(["default", "duble", "round"])
                .value_parser(["duble", "round"])
                .hide_possible_values(true)
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
            Arg::new("horizontal")
                .short('H')
                .long("horizontal")
                .help("Sets the view of horizontal line")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("vertical")
                .short('V')
                .long("vertical")
                .help("Sets the view of vertical line")
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
                .value_parser([
                    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
                ])
                .default_value("black")
                .hide_default_value(true)
                .hide_possible_values(true)
                .required(false),
        )
        .arg(
            Arg::new("file")
                .value_parser(value_parser!(PathBuf))
                .index(1),
        )
        //.arg(
        //    arg!(-p  --position <NUMBER>      "Select start and end characters")
        //        .value_parser(value_parser!(usize))
        //        .number_of_values(2)
        //        .use_value_delimiter(true)
        //        .required(false),
        //)
        //.arg(
        //    arg!(-e  --exclude <STRING>        "Exclude chars filter")
        //        .action(ArgAction::Append)
        //        .required(false),
        //)
        .get_matches()
}
