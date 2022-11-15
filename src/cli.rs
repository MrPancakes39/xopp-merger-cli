use std::process;

use crate::errors::ErrorHandler;
use crate::merge::merge_files;

pub fn run(args: Vec<String>) {
    let (input, output) = parse_args(&args).unwrap_or_else(|err| match err {
        ParseError::NeedHelp => display_help(),
        ParseError::NeedVersion => {
            version();
            process::exit(0);
        }
        _ => err.handle_error(),
    });
    merge_files(input, &output).unwrap_or_else(|err| {
        err.handle_error();
    });
}

#[derive(Debug)]
pub enum ParseError {
    NeedHelp,
    NeedVersion,
    NotEnoughArgs,
}

fn parse_args(args: &[String]) -> Result<(&[String], &String), ParseError> {
    // command help
    if args.len() == 2 {
        match &args[1][..] {
            "help" => return Err(ParseError::NeedHelp),
            "version" => return Err(ParseError::NeedVersion),
            _ => {}
        }
    }
    // min: command input1 input2 output
    if args.len() < 4 {
        return Err(ParseError::NotEnoughArgs);
    }

    // get input and output
    let n = args.len();
    let input = &args[1..n - 1];
    let output = &args[n - 1];

    Ok((input, output))
}

fn version() {
    println!(
        "xopp-merger {} ({})",
        include_str!("version.txt"),
        include_str!("commit.txt")
    );
}

fn display_help() -> ! {
    version();
    println!(
        r#"A small tool to merge multiple xournal notebooks.

USAGE:
    xopp-merger file1.xopp file2.xopp... output.xopp
    xopp-merger subcommand

ARGS:
    file1.xopp, file2.xopp...
        The notebooks to merge.

    output.xopp
        The path to save the merged files.

SUBCOMMANDS:
    help
        Print this help message.

    version
        Print version information."#
    );

    process::exit(0);
}
