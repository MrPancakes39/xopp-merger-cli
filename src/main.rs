mod cli;
mod merge;

use cli::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}
