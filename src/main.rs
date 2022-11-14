mod merge;

use anyhow::Result;
use merge::merge_files;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let (input, output) = parse_args(&args).unwrap();
    println!("input: {:?}", input);
    println!("output: {:?}", output);
    merge_files(input, &output).unwrap();
    Ok(())
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
