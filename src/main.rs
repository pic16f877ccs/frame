#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use clap::{arg, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use colored::{Color, Colorize};
use std::io::{self, prelude::*};
use std::fs;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = app_commands();
    let mut buff = String::new();

    //println!("Hello, world!");
    println!("In file: {:?}", app.get_one::<PathBuf>("file").unwrap());
    match app.get_one::<PathBuf>("file") {
        Some(path) => {buff = fs::read_to_string(path)?}
        None => {stdin().read_to_string(&mut buff)?;}
    }
    println!("Print path: {}", buff);
    Ok(())
}

fn app_commands() -> ArgMatches {
    Command::new("frame")
        .about("    frame for text")
        .author("    by PIC16F877ccs")
        .args_override_self(true)
        .arg(
            Arg::new("frame")
                .num_args(0)
                .short('f')
                .long("frame")
                .help("Displays text in a frame")
                .required(false),
        )
        .arg(
            Arg::new("top-left")
                .short('S')
                .long("top-left")
                .help("Sets the top left corner")
                .value_parser(value_parser!(char))
                .value_name("CHARACTER")
                .num_args(1)
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
                .index(1)
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
