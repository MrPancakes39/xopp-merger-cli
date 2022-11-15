mod merge;

use merge::merge_files;
use std::process;

pub fn run(args: Vec<String>) {
    let (input, output) = parse_args(&args).unwrap_or_else(|err| {
        err.handle_error();
    });
    merge_files(input, &output).unwrap();
}

#[derive(Debug)]
enum ParseError {
    NeedHelp,
    NotEnoughArgs,
}

fn parse_args(args: &[String]) -> Result<(&[String], &String), ParseError> {
    // command help
    if args.len() == 2 && args[1] == "help" {
        return Err(ParseError::NeedHelp);
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

trait ErrorHandler {
    fn handle_error(&self) -> !;
}

impl ErrorHandler for ParseError {
    fn handle_error(&self) -> ! {
        println!("{:?}", self);
        process::exit(1);
    }
}
