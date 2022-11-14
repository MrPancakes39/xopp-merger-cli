mod cli;
mod merge;

use cli::*;
use merge::merge_files;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (input, output) = parse_args(&args).unwrap_or_else(|err| {
        handle_error(err);
        process::exit(1);
    });
    merge_files(input, &output).unwrap();
}
