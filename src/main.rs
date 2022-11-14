use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug)]
enum MergeError {
    LengthError,
    IOError(io::Error),
    FormatError,
}

impl From<io::Error> for MergeError {
    fn from(err: io::Error) -> Self {
        MergeError::IOError(err)
    }
}

fn merge_files(file_list: Vec<&str>, path: &str) -> Result<(), MergeError> {
    if file_list.len() < 2 {
        return Err(MergeError::LengthError);
    }

    let mut merged = String::new();
    for (i, file) in file_list.into_iter().enumerate() {
        // read the file as bytes
        let mut f = File::open(file)?;
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf)?;

        // decompress the file to xml
        let mut d = GzDecoder::new(&buf[..]);
        let mut s = String::new();
        d.read_to_string(&mut s)?;

        if i == 0 {
            // find the end of first file
            let n = s.find("</xournal>");
            if n.is_none() {
                return Err(MergeError::FormatError);
            }
            merged = (&s[..n.unwrap()]).to_string(); // without end tag
        } else {
            // find the pages  of file
            let st = s.find("<page");
            let en = s.find("</xournal>");
            if st.is_none() || en.is_none() {
                return Err(MergeError::FormatError);
            }
            merged.push_str(&s[st.unwrap()..en.unwrap()])
        }
    }
    merged.push_str("</xournal>"); // adds end tag back

    // compress xml string
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(merged.as_bytes())?;
    let compressed = e.finish()?;

    // save it
    let mut f = File::create(path)?;
    f.write_all(&compressed[..])?;

    Ok(())
}

fn main() -> Result<(), MergeError> {
    let list = vec!["src/data/notebook1.xopp", "src/data/notebook2.xopp"];
    merge_files(list, "src/data/output.xopp").unwrap();
    Ok(())
}
