use crate::errors::ErrorHandler;
use crate::merge::merge_files;

pub fn run(args: Vec<String>) {
    let (input, output) = parse_args(&args).unwrap_or_else(|err| {
        err.handle_error();
    });
    merge_files(input, &output).unwrap_or_else(|err| {
        err.handle_error();
    });
}

#[derive(Debug)]
pub enum ParseError {
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

fn version() {
    println!(
        "{} {} ({})",
        include_str!("name.txt"),
        include_str!("version.txt"),
        include_str!("commit.txt")
    );
}
