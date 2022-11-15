use crate::cli::ParseError;
use crate::merge::MergeError;
use std::process;

pub trait ErrorHandler {
    fn handle_error(&self) -> !;
}

impl ErrorHandler for ParseError {
    fn handle_error(&self) -> ! {
        eprint!("[xopp-merger error]: ");
        match self {
            Self::NeedHelp | Self::NeedVersion => {}
            Self::NotEnoughArgs => eprintln!("Not enough input arguments."),
        }
        process::exit(1);
    }
}

impl ErrorHandler for MergeError {
    fn handle_error(&self) -> ! {
        println!("{:?}", self);
        process::exit(1);
    }
}
