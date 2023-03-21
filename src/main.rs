#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use clap::{arg, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use colored::{Color, Colorize};
use std::error::Error;
use std::fs;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let app = app_commands();
    let mut buff = String::new();
    let mut head_line = String::new();
    let variants = app.get_one::<String>("frame").unwrap();

    let variants_index: usize = match variants.as_str() {
            "duble" => 1,
            "round" => 2,
            "default" => 0,
            &_ => todo!(),
    };

    let frame_variants = [
        ['─', '│', '┌', '┐', '└', '┘'],
        ['═', '║', '╔', '╗', '╚', '╝'],
        ['─', '│', '╭', '╮', '╰', '╯'],
    ];
    let hor_line: char = *app
        .get_one("horizontal")
        .unwrap_or(&frame_variants[variants_index][0]);
    let vrt_line: char = *app
        .get_one("vertical")
        .unwrap_or(&frame_variants[variants_index][1]);
    let top_left: char = *app
        .get_one("top-left")
        .unwrap_or(&frame_variants[variants_index][2]);
    let top_right: char = *app
        .get_one("top-right")
        .unwrap_or(&frame_variants[variants_index][3]);
    let bottom_left: char = *app
        .get_one("bottom-left")
        .unwrap_or(&frame_variants[variants_index][4]);
    let bottom_right: char = *app
        .get_one("bottom-right")
        .unwrap_or(&frame_variants[variants_index][5]);

    //println!("In file: {:?}", app.get_one::<PathBuf>("file").unwrap());
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
        head_line.push(hor_line);
    }

    std::io::stdout().write_all(format!("{top_left}{head_line}{top_right}\n").as_bytes())?;

    for current_line in buff.lines() {
        let mut current_fill = String::new();

        for _ in 0..(max_line_len - current_line.chars().count()) {
            current_fill.push(' ');
        }

        std::io::stdout()
            .write_all(format!("{vrt_line}{current_line}{current_fill}{vrt_line}\n").as_bytes())?;
    }

    std::io::stdout().write_all(format!("{bottom_left}{head_line}{bottom_right}\n").as_bytes())?;

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
                .default_value("default")
                .value_parser(["default", "duble", "round"])
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
