use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug)]
pub enum MergeError {
    LengthError,
    IOError(io::Error),
    FormatError,
}

impl From<io::Error> for MergeError {
    fn from(err: io::Error) -> Self {
        MergeError::IOError(err)
    }
}

pub fn merge_files(file_list: Vec<&str>, path: &str) -> Result<(), MergeError> {
    if file_list.len() < 2 {
        return Err(MergeError::LengthError);
    }

    let mut merged = String::new();
    {
        let mut byte_buffer: Vec<u8> = Vec::new();
        let mut char_buffer: String = String::new();
        for (i, file) in file_list.into_iter().enumerate() {
            // read the file as bytes
            byte_buffer.clear();
            let mut f = File::open(file)?;
            f.read_to_end(&mut byte_buffer)?;

            // decompress the file to xml
            char_buffer.clear();
            let mut d = GzDecoder::new(&byte_buffer[..]);
            d.read_to_string(&mut char_buffer)?;

            if i == 0 {
                // find the end of first file
                let n = char_buffer.find("</xournal>");
                if n.is_none() {
                    return Err(MergeError::FormatError);
                }
                merged = (&char_buffer[..n.unwrap()]).to_string(); // without end tag
            } else {
                // find the pages  of file
                let st = char_buffer.find("<page");
                let en = char_buffer.find("</xournal>");
                if st.is_none() || en.is_none() {
                    return Err(MergeError::FormatError);
                }
                merged.push_str(&char_buffer[st.unwrap()..en.unwrap()])
            }
        }
        merged.push_str("</xournal>"); // adds end tag back
    }

    // compress xml string
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(merged.as_bytes())?;
    let compressed = e.finish()?;

    // save it
    let mut f = File::create(path)?;
    f.write_all(&compressed[..])?;

    Ok(())
}
