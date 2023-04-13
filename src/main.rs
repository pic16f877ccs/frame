use clap::{value_parser, Arg, ArgMatches, Command};
use colored::{Color, ColoredString, Colorize};
use std::error::{self, Error};
use std::fs;
use std::io::{self, Read, Write};
use std::iter;
use std::path::PathBuf;
use std::str::FromStr;
use terminal_size::{Width, terminal_size};

impl Default for Frame<'_> {
    fn default() -> Self {
        Self {
            top_left_corner: "┌",
            hor_top_line: "─",
            top_right_corner: "┐",
            vert_left: "│",
            vert_right: "│",
            bottom_left_corner: "└",
            bottom_right_corner: "┘",
            hor_bottom_line: "─",
            fill: " ",
            algn: Algn::Left,
            centr: 0,
            expand: 0,
            expand_width: 0,
            color: Color::Black,
        }
    }
}

#[derive(Debug)]
struct Frame<'a> {
    top_left_corner: &'a str,
    hor_top_line: &'a str,
    top_right_corner: &'a str,
    bottom_left_corner: &'a str,
    bottom_right_corner: &'a str,
    vert_left: &'a str,
    vert_right: &'a str,
    hor_bottom_line: &'a str,
    fill: &'a str,
    algn: Algn,
    centr: usize,
    expand: usize,
    expand_width: usize,
    color: Color,
}

impl<'b> Frame<'b> {
    fn frame_empty() -> Self {
        Self {
            top_left_corner: " ",
            hor_top_line: " ",
            top_right_corner: " ",
            bottom_left_corner: " ",
            bottom_right_corner: " ",
            vert_left: " ",
            vert_right: " ",
            hor_bottom_line: " ",
            ..Self::default()
        }
    }
    fn frame_double() -> Self {
        Self {
            top_left_corner: "╔",
            hor_top_line: "═",
            top_right_corner: "╗",
            bottom_left_corner: "╚",
            bottom_right_corner: "╝",
            vert_left: "║",
            vert_right: "║",
            hor_bottom_line: "═",
            ..Self::default()
        }
    }
    fn frame_hor_double() -> Self {
        Self {
            top_left_corner: "╒",
            hor_top_line: "═",
            top_right_corner: "╕",
            bottom_left_corner: "╘",
            bottom_right_corner: "╛",
            hor_bottom_line: "═",
            ..Self::default()
        }
    }

    fn frame_vert_double() -> Self {
        Self {
            top_left_corner: "╓",
            top_right_corner: "╖",
            bottom_left_corner: "╙",
            bottom_right_corner: "╜",
            vert_left: "║",
            vert_right: "║",
            ..Self::default()
        }
    }

    fn frame_torn() -> Self {
        Self {
            vert_left: "┆",
            vert_right: "┆",
            ..Self::default()
        }
    }

    fn frame_round() -> Self {
        Self {
            top_left_corner: "╭",
            top_right_corner: "╮",
            bottom_left_corner: "╰",
            bottom_right_corner: "╯",
            ..Self::default()
        }
    }

    fn frames(frame: &str) -> Self {
        match frame {
            "default" => Self::default(),
            "empty" => Self::frame_empty(),
            "double" => Self::frame_double(),
            "hor-double" => Self::frame_hor_double(),
            "vert-double" => Self::frame_vert_double(),
            "round" => Self::frame_round(),
            "torn" => Self::frame_torn(),
            _ => Self::default(),
        }
    }

    fn frame_text_algn(mut self, algn: &str) -> Self {
        self.algn = match algn {
            "left" => Algn::Left,
            "centr" => Algn::Centr,
            "right" => Algn::Right,
            _ => Algn::Left,
        };
        self
    }

    fn from_args(
        app: &'b ArgMatches,
        encode_arr: &'b mut EncodeArr,
    ) -> Result<Self, Box<dyn error::Error>> {
        let frame = if let Some(frame) = app.get_one::<String>("frame") {
            Self::frames(frame)
        } else {
            Self::frames("default")
        };

        let chr_right_bottom = &char::from_str(frame.bottom_right_corner)?;
        let chr_hor_bottom = &char::from_str(frame.hor_bottom_line)?;
        let chr_left_bottom = &char::from_str(frame.bottom_left_corner)?;
        let chr_vert_right = &char::from_str(frame.vert_right)?;
        let chr_vert_left = &char::from_str(frame.vert_left)?;
        let chr_right_top = &char::from_str(frame.top_right_corner)?;
        let chr_hor_top = &char::from_str(frame.hor_top_line)?;
        let chr_left_top = &char::from_str(frame.top_left_corner)?;
        let chr_fill = &char::from_str(frame.fill)?;

        Ok(Self {
            top_left_corner: if let Some(chr) = app.get_one::<char>("top_left") {
                chr
            } else {
                chr_left_top
            }
            .encode_utf8(&mut encode_arr.left_top),
            hor_top_line: if let Some(chr) = app.get_one::<char>("horizontal_top") {
                chr
            } else {
                chr_hor_top
            }
            .encode_utf8(&mut encode_arr.hor_top_line),
            top_right_corner: if let Some(chr) = app.get_one::<char>("top_right") {
                chr
            } else {
                chr_right_top
            }
            .encode_utf8(&mut encode_arr.right_top),
            vert_left: if let Some(chr) = app.get_one::<char>("vert_left") {
                chr
            } else {
                chr_vert_left
            }
            .encode_utf8(&mut encode_arr.vert_left),
            vert_right: if let Some(chr) = app.get_one::<char>("vert_right") {
                chr
            } else {
                chr_vert_right
            }
            .encode_utf8(&mut encode_arr.vert_right),
            bottom_left_corner: if let Some(chr) = app.get_one::<char>("bottom_left") {
                chr
            } else {
                chr_left_bottom
            }
            .encode_utf8(&mut encode_arr.left_bottom),
            hor_bottom_line: if let Some(chr) = app.get_one::<char>("horizontal_bottom") {
                chr
            } else {
                chr_hor_bottom
            }
            .encode_utf8(&mut encode_arr.hor_bottom_line),
            bottom_right_corner: if let Some(chr) = app.get_one::<char>("bottom_right") {
                chr
            } else {
                chr_right_bottom
            }
            .encode_utf8(&mut encode_arr.right_bottom),
            fill: if let Some(chr) = app.get_one::<char>("fill") {
                chr
            } else {
                chr_fill
            }
            .encode_utf8(&mut encode_arr.fill),
            color: if let Some(color) = app.get_one::<String>("color") {
                match color.as_str() {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "yellow" => Color::Yellow,
                    "blue" => Color::Blue,
                    "magenta" => Color::Magenta,
                    "cyan" => Color::Cyan,
                    "white" => Color::White,
                    _ => Color::Black,
                }
            } else {
                Color::Black
            },
            expand: if let Some(expand) = app.get_one::<u8>("expand") {
                *expand as usize
            } else {
                0
            },
            expand_width: if let Some(expand_width) = app.get_one::<u8>("expand_width") {
                *expand_width as usize
            } else {
                0
            },
            centr: if app.get_flag("frame_centr") { if let Some((Width(w), _)) = terminal_size() { w as usize } else { 0 } } else { 0 },
            ..frame.frame_text_algn(if let Some(string) = app.get_one::<String>("alignment") {
                string
            } else {
                "left"
            })
        })
    }

    fn frame_build<'a>(
        &'a self,
        text_buffer: &'a String,
    ) -> impl Iterator<Item = ColoredString> + '_ {
        let max_line_len = text_buffer.max_line_len((self.expand + self.expand_width) * 2);
        let centr = if self.centr > max_line_len { (self.centr - max_line_len) / 2 } else { 0 };

        let enlarge_line_iter = iter::repeat(" ".clear()).take(centr) 
            .chain(iter::once(self.vert_left.color(self.color)))
            .chain(iter::repeat(self.fill.color("default")).take(max_line_len))
            .chain(iter::once(self.vert_right.color(self.color)))
            .chain(iter::once("\n".clear()))
            .cycle()
            .take((max_line_len + 3 + centr) * self.expand);

        let top_half_frame_iter = iter::once("\n".color("default"))
            .chain(iter::repeat(" ".clear()).take(centr))
            .chain(iter::once(self.top_left_corner.color(self.color)))
            .chain(iter::repeat(self.hor_top_line.color(self.color)).take(max_line_len))
            .chain(iter::once(self.top_right_corner.color(self.color)))
            .chain(iter::once("\n".clear()))
            .chain(enlarge_line_iter.clone());

        let bottom_half_frame_iter = enlarge_line_iter
            .chain(iter::repeat(" ".clear()).take(centr))
            .chain(iter::once(self.bottom_left_corner.color(self.color)))
            .chain(iter::repeat(self.hor_bottom_line.color(self.color)).take(max_line_len))
            .chain(iter::once(self.bottom_right_corner.color(self.color)))
            .chain(iter::once("\n".clear()));

        let lines_buffer_iter = text_buffer.lines().flat_map(move |line| {
            let curr_line_len = line.chars().count();
            let algnment = match self.algn {
                Algn::Left => (0, max_line_len - curr_line_len),
                Algn::Centr => (
                    (max_line_len - curr_line_len) / 2,
                    (max_line_len - curr_line_len) - (max_line_len - curr_line_len) / 2,
                ),
                Algn::Right => (max_line_len - curr_line_len, 0),
            };

            let iter_top = iter::repeat(" ".clear()).take(centr)
                .chain(iter::once(self.vert_left.color(self.color)))
                .chain(iter::repeat(self.fill.color("default")).take(algnment.0));
            let iter_line = iter::once(line.color("default"));
            let iter_bottom = iter::repeat(self.fill.color("default"))
                .take(algnment.1)
                .chain(iter::once(self.vert_right.color(self.color)))
                .chain(iter::once("\n".color("default")));

            iter_top.chain(iter_line).chain(iter_bottom)
        });

        top_half_frame_iter
            .chain(lines_buffer_iter)
            .chain(bottom_half_frame_iter)
    }
}

#[derive(Default)]
struct EncodeArr {
    left_top: [u8; 4],
    hor_top_line: [u8; 4],
    right_top: [u8; 4],
    vert_left: [u8; 4],
    vert_right: [u8; 4],
    left_bottom: [u8; 4],
    right_bottom: [u8; 4],
    hor_bottom_line: [u8; 4],
    fill: [u8; 4],
}

trait MaxLineLen {
    fn max_line_len(&self, expand: usize) -> usize;
}

impl MaxLineLen for String {
    fn max_line_len(&self, expand: usize) -> usize {
        self.lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0)
            + expand
    }
}

#[derive(Debug)]
enum Algn {
    Left,
    Centr,
    Right,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut encode_arr = EncodeArr::default();
    let mut text_buffer = String::new();
    let app = app_commands();
    let frame = Frame::from_args(&app, &mut encode_arr)?;

    read_input(&app, &mut text_buffer)?;
    write_output(frame.frame_build(&text_buffer))?;

    Ok(())
}

fn read_input(app: &ArgMatches, text_buffer: &mut String) -> io::Result<()> {
    match app.get_one::<PathBuf>("file") {
        Some(path) => *text_buffer = fs::read_to_string(path)?,
        None => {
            std::io::stdin().lock().read_to_string(text_buffer)?;
        }
    }

    Ok(())
}

fn write_output<I>(iter: I) -> io::Result<()>
where
    I: Iterator,
    <I as IntoIterator>::Item: std::fmt::Display,
{
    let mut stdout = std::io::stdout().lock();
    for out_line in iter {
        write!(stdout, "{out_line}")?;
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
                .value_parser([
                    "empty",
                    "double",
                    "hor-double",
                    "vert-double",
                    "round",
                    "heavy",
                    "torn",
                ])
                .required(false),
        )
        .arg(
            Arg::new("frame_centr")
                .long("centered")
                .action(clap::ArgAction::SetTrue)
                .value_name("ALIGNMENT")
                //.num_args(0)
                .help("Display frame centered")
                .required(false),
        )
        .arg(
            Arg::new("alignment")
                .short('a')
                .long("algn")
                .num_args(1)
                .value_name("ALIGNMENT")
                .help("Frame text alingment")
                .value_parser(["left", "centr", "right"])
                .required(false),
        )
        .arg(
            Arg::new("expand")
                .long("expand")
                .num_args(1)
                .value_name("NUMBER")
                .value_parser(value_parser!(u8).range(1..255))
                .help("Enlarge frame")
                .required(false),
        )
        .arg(
            Arg::new("expand_width")
                .long("expand-width")
                .num_args(1)
                .value_name("NUMBER")
                .value_parser(value_parser!(u8).range(1..255))
                .help("Enlarge frame width")
                .required(false),
        )
        .arg(
            Arg::new("fill")
                .long("fill")
                .num_args(1)
                .value_name("CHARACTER")
                .value_parser(value_parser!(char))
                .default_value(" ")
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
            Arg::new("color")
                .short('c')
                .long("color")
                .value_name("COLOR")
                .help("Displays a text frame in the specified color")
                .value_parser([
                    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
                ])
                .default_value("black")
                .required(false),
        )
        .arg(
            Arg::new("file")
                .value_parser(value_parser!(PathBuf))
                .index(1),
        )
        .get_matches()
}
