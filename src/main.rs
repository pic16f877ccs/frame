#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use clap::{arg, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use colored::{Color, Colorize};
use std::io::{self, prelude::*};

fn main() {
    app_commands();

    println!("Hello, world!");
}

fn app_commands() -> ArgMatches {
    Command::new("frame")
        .about("    frame for text")
        .author("    by PIC16F877ccs")
        .args_override_self(true)
        .arg(
            arg!(-f  --frame <BOOL>        "Displays text in a frame")
                .number_of_values(1)
                .value_parser(value_parser!(bool))
                .default_missing_value("true")
                .required(false),
        )
        .arg(
            Arg::new("top-left")
                .short('S')
                .long("top-left")
                .help("Sets the top left corner")
                .value_parser(value_parser!(char))
                .number_of_values(1)
                .required(false),
        )
        .arg(
            Arg::new("top-right")
                .short('E')
                .long("top-right")
                .help("Sets the top right corner")
                .value_parser(value_parser!(char))
                .number_of_values(1)
                .required(false),
        )
        .arg(
            arg!(-H  --horizontal <STRING>        "Sets the view of horizontal line")
                .value_parser(value_parser!(char))
                .number_of_values(1)
                .required(false),
        )
        .arg(
            arg!(-V  --vertical <STRING>       "Sets the view of vertical line")
                .value_parser(value_parser!(char))
                .number_of_values(1)
                .required(false),
        )
        .arg(
            Arg::new("bottom-left")
                .short('s')
                .long("bottom-left")
                .help("Sets the bottom left corner")
                .value_parser(value_parser!(char))
                .number_of_values(1)
                .required(false),
        )
        .arg(
            Arg::new("bottom-right")
                .short('e')
                .long("bottom-right")
                .help("Sets the bottom right corner")
                .value_parser(value_parser!(char))
                .number_of_values(1)
                .required(false),
        )
        .arg(
            arg!(-c  --color <STRING>        "Displays a text frame in the specified color")
                .number_of_values(1)
                .value_parser([
                    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
                ])
                .default_value("black")
                .hide_default_value(true)
                .hide_possible_values(true)
                .required(false),
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
