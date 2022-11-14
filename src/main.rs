mod merge;

use merge::{merge_files, MergeError};

fn main() -> Result<(), MergeError> {
    let list = vec!["src/example/notebook1.xopp", "src/example/notebook2.xopp"];
    merge_files(list, "src/example/output.xopp").unwrap();
    Ok(())
}
