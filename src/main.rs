#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use clap::{value_parser, Arg, ArgMatches, Command};
use colored::Colorize;
use std::error::{self, Error};
use std::fs;
use std::io::{stdin, Read, Write};
use std::iter;
use std::path::PathBuf;

struct Frame<'a> {
    frame_variants: [char; 10],
    max_line_len: usize,
    expand: usize,
    expand_width: usize,
    color: &'a str,
    fill: char,
    hor_top_line: String,
    hor_bottom_line: String,
    line_buff: String,
}

impl Default for Frame<'_> {
    fn default() -> Self {
        Self {
            frame_variants: ['┌', '┐', '─', '─', '│', '│', '└', '┘', '├', '┤'],
            max_line_len: 0,
            expand: 0,
            expand_width: 0,
            fill: ' ',
            color: "",
            hor_top_line: String::new(),
            hor_bottom_line: String::new(),
            line_buff: String::new(),
        }
    }
}

impl Frame<'_> {
    fn new() -> Self {
        Self::default()
    }

    fn read(&mut self, app: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
        self.variants(&app);
        self.custom(&app);
        match app.get_one::<PathBuf>("file") {
            Some(path) => self.line_buff = fs::read_to_string(path)?,
            None => {
                stdin().read_to_string(&mut self.line_buff)?;
            }
        }

        self.expand = *(app.get_one("expand").unwrap_or(&0u8)) as usize;
        self.expand_width = *(app.get_one("expand_width").unwrap_or(&0u8)) as usize;
        self.max_line_len = self
            .line_buff
            .lines()
            .map(|line| line.chars().count())
            .max()
            .ok_or("unknown maximum line length")?
            + self.expand * 2
            + self.expand_width * 2;

        self.hor_top_line = format!(
            "{top_left}{hor_top}{top_right}",
            top_left = self.get_top_left(),
            hor_top = self.get_hor_top().to_string().repeat(self.max_line_len),
            top_right = self.get_top_right()
        );

        self.hor_bottom_line = format!(
            "{bottom_left}{hor_bottom}{bottom_right}",
            bottom_left = self.get_bottom_left(),
            hor_bottom = self.get_hor_buttom().to_string().repeat(self.max_line_len),
            bottom_right = self.get_bottom_right()
        );

        self.color = if let Some(color) = app.get_one::<String>("color") {
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

        Ok(())
    }

    fn duble(&mut self) {
        for (i, elem) in self.frame_variants.iter_mut().enumerate() {
            *elem = ['╔', '╗', '═', '═', '║', '║', '╚', '╝', '╠', '╣'][i];
        }
    }

    fn round(&mut self) {
        for (i, elem) in self.frame_variants.iter_mut().enumerate() {
            *elem = ['╭', '╮', '─', '─', '│', '│', '╰', '╯', '├', '┤'][i];
        }
    }

    fn heavy(&mut self) {
        for (i, elem) in self.frame_variants.iter_mut().enumerate() {
            *elem = ['┏', '┓', '━', '━', '┃', '┃', '┗', '┛', '┣', '┫'][i];
        }
    }

    fn torn(&mut self) {
        for (i, elem) in self.frame_variants.iter_mut().enumerate() {
            *elem = ['┏', '┓', '━', '━', '╏', '╏', '┗', '┛', '┣', '┫'][i];
        }
    }

    fn variants(&mut self, app: &ArgMatches) {
        if let Some(variants) = app.get_one::<String>("frame") {
            match variants.as_str() {
                "duble" => {
                    self.duble();
                }
                "round" => {
                    self.round();
                }
                "heavy" => {
                    self.heavy();
                }
                "torn" => {
                    self.torn();
                }
                _ => (),
            }
        } else {
            ()
        }
    }

    fn custom(&mut self, app: &ArgMatches) {
        self.set_hor_top(*app.get_one("horizontal_top").unwrap_or(&self.get_hor_top()));
        self.set_hor_bottom(
            *app.get_one("horizontal_bottom")
                .unwrap_or(&self.get_hor_buttom()),
        );
        self.set_vert_left(*app.get_one("vert_left").unwrap_or(&self.get_vert_left()));
        self.set_vert_right(*app.get_one("vert_right").unwrap_or(&self.get_vert_right()));
        self.set_top_left(*app.get_one("top_left").unwrap_or(&self.get_top_left()));
        self.set_top_right(*app.get_one("top_right").unwrap_or(&self.get_top_right()));
        self.set_fill(*app.get_one("fill").unwrap_or(&self.get_fill()));
        self.set_bottom_left(
            *app.get_one("bottom_left")
                .unwrap_or(&self.get_bottom_left()),
        );
        self.set_bottom_right(
            *app.get_one("bottom_right")
                .unwrap_or(&self.get_bottom_right()),
        );
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

    fn get_vert_hor_left(&self) -> char {
        self.frame_variants[8]
    }

    fn get_vert_hor_right(&self) -> char {
        self.frame_variants[9]
    }

    fn get_fill(&self) -> char {
        self.fill
    }

    fn set_fill(&mut self, c: char) {
        self.fill = c;
    }

    fn set_top_left(&mut self, c: char) -> &Self {
        self.frame_variants[0] = c;
        self
    }

    fn set_top_right(&mut self, c: char) -> &Self {
        self.frame_variants[1] = c;
        self
    }

    fn set_hor_top(&mut self, c: char) -> &Self {
        self.frame_variants[2] = c;
        self
    }

    fn set_hor_bottom(&mut self, c: char) -> &Self {
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

    fn set_vert_hor_left(&mut self, c: char) -> &Self {
        self.frame_variants[8] = c;
        self
    }

    fn set_vert_hor_right(&mut self, c: char) -> &Self {
        self.frame_variants[9] = c;
        self
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = app_commands();
    let mut frame = Frame::new();
    frame.read(&app)?;

    let top_new_line = (0..1).map(|_| '\n').next();
    let vrt_left = (0..1).map(|_| frame.get_vert_left()).next();
    let expand_line = (0..1).into_iter().map(|_| frame.fill).next();
    let vrt_right = (0..1).map(|_| frame.get_vert_right()).next();
    let expand_new_line = (0..1).map(|_| '\n').next();

    let expand_line = vrt_left
        .into_iter()
        .chain(expand_line.into_iter().cycle().take(frame.max_line_len))
        .chain(vrt_right.into_iter())
        .chain(expand_new_line.into_iter());

    let hor_top_lin = iter::repeat_with(|| frame.get_top_left())
        .take(1)
        .chain(iter::repeat_with(|| frame.get_hor_top()).take(frame.max_line_len))
        .chain(iter::repeat_with(|| frame.get_top_right()).take(1))
        .chain(iter::repeat_with(|| '\n').take(1))
        .chain(
            expand_line
                .cycle()
                .take((frame.max_line_len + 3) * frame.expand),
        )
        .collect::<String>();

    let vrt_left = String::from(frame.get_vert_left()).color(frame.color);
    let vrt_right = String::from(frame.get_vert_right()).color(frame.color);
    let hor_line = format!(
        "{hor_line}\n",
        hor_line = String::from(frame.fill).repeat(frame.max_line_len)
    )
    .repeat(frame.expand);
    let empty_line = format!(
        "{vrt_hor_left}{space_line}{vrt_hor_right}",
        space_line = String::from(frame.get_hor_top())
            .repeat(frame.max_line_len)
            .color(frame.color),
        vrt_hor_left = String::from(frame.get_vert_hor_left()).color(frame.color),
        vrt_hor_right = String::from(frame.get_vert_hor_right()).color(frame.color),
    );

    frame.line_buff = format!(
        //"{hor_top_line}{hor_line}{buff}{hor_line}{hor_bottom_line}\n",
        "{hor_top_lin}{buff}{hor_line}{hor_bottom_line}\n",
        //hor_top_line = frame.hor_top_line.color(frame.color),
        buff = frame.line_buff,
        hor_bottom_line = frame.hor_bottom_line.color(frame.color),
    );

    for current_line in frame.line_buff.lines() {
        let current_line_len = current_line.chars().count();

        let out_line = if frame.max_line_len < current_line_len {
            format!("{current_line}\n")
        } else if (current_line_len == 0) && app.get_flag("blank_line") {
            format!("{empty_line}\n")
        } else {
            let mut left = 0;
            let line_len = frame.max_line_len - current_line_len;

            if let Some(aling) = app.get_one::<String>("alignment") {
                match aling.as_str() {
                    "centr" => left = line_len / 2,
                    "right" => left = line_len,
                    _ => left = 0,
                }
            }

            format!(
                "{vrt_left}{left_fill}{current_line}{right_fill}{vrt_right}\n",
                left_fill = String::from(frame.fill).repeat(left),
                right_fill = String::from(frame.fill).repeat(line_len - left),
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
                .required(false),
        )
        .arg(
            Arg::new("alignment")
                .short('a')
                .long("align")
                .num_args(1)
                .value_name("ALINGMENT")
                .help("Frame text alingment")
                .value_parser(["centr", "right"])
                //.hide_possible_values(true)
                .required(false),
        )
        .arg(
            Arg::new("expand")
                .long("expand")
                .num_args(1)
                .value_name("NUMBER")
                .value_parser(value_parser!(u8).range(1..100))
                .help("Enlarge frame")
                .required(false),
        )
        .arg(
            Arg::new("expand_width")
                .long("expand-width")
                .num_args(1)
                .value_name("NUMBER")
                .value_parser(value_parser!(u8).range(1..100))
                .help("Enlarge frame width")
                .required(false),
        )
        .arg(
            Arg::new("fill")
                .long("fill")
                .num_args(1)
                .value_name("CHARACTER")
                .value_parser(value_parser!(char))
                .help("Sets the fill character")
                .required(false),
        )
        .arg(
            Arg::new("top_left")
                .short('S')
                .long("top-left")
                .num_args(1)
                .value_name("CHARACTER")
                .value_parser(value_parser!(char))
                .help("Sets the top left corner")
                .required(false),
        )
        .arg(
            Arg::new("top_right")
                .short('E')
                .long("top-right")
                .help("Sets the top right corner")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("horizontal_top")
                .short('H')
                .long("hor-top")
                .help("Sets the view of horizontal top line")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("horizontal_bottom")
                .long("hor-bottom")
                .help("Sets the view of horizontal bottom line")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("vert_left")
                .short('V')
                .long("vert-left")
                .help("Sets the view of vertical left line")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("vert_right")
                .long("vert-right")
                .help("Sets the view of vertical right line")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("bottom_left")
                .short('s')
                .long("bottom-left")
                .value_name("CHARACTER")
                .help("Sets the bottom left corner")
                .value_parser(value_parser!(char))
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("bottom_right")
                .short('e')
                .long("bottom-right")
                .value_name("CHARACTER")
                .help("Sets the bottom right corner")
                .value_parser(value_parser!(char))
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("blank_line")
                .short('b')
                .long("blank")
                .action(clap::ArgAction::SetTrue)
                .help("Insert into blank line")
                .num_args(0)
                .required(false),
        )
        .arg(
            Arg::new("color")
                .short('c')
                .long("color")
                .value_name("COLOR")
                .help("Displays a text frame in the specified color")
                .value_parser(["red", "green", "yellow", "blue", "magenta", "cyan", "white"])
                .required(false),
        )
        .arg(
            Arg::new("file")
                .value_parser(value_parser!(PathBuf))
                .index(1),
        )
        .get_matches()
}
