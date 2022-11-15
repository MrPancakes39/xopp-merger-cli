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
            Self::PathError(path) => eprintln!("Path doesn't exist: {}", path),
        }
        process::exit(1);
    }
}

impl ErrorHandler for MergeError {
    fn handle_error(&self) -> ! {
        eprint!("[xopp-merger error]: ");
        match self {
            Self::LengthError => eprintln!("Not enough input files."),
            Self::IOError(err) => {
                eprintln!("{}.", err);
                eprintln!("                     {}.", err.root_cause())
            }
            Self::FormatError(file) => {
                eprintln!("File '{file}' is not in the right format.")
            }
        }
        process::exit(1);
    }
}
