mod merge;

use merge::{merge_files, MergeError};

fn main() -> Result<(), MergeError> {
    let list = vec!["src/data/notebook1.xopp", "src/data/notebook2.xopp"];
    merge_files(list, "src/data/output.xopp").unwrap();
    Ok(())
}
