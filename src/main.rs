use std::fs::{read_dir, DirEntry};

pub use std::format as f;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

type R<T> = core::result::Result<T, Error>;
pub struct W<T>(pub T);

impl TryFrom<W<&DirEntry>> for String {
    type Error = Error;

    fn try_from(value: W<&DirEntry>) -> R<String> {
        value
            .0
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| Error::Generic(f!("invalid path {:?}", value.0)))
    }
}

fn main() -> R<()> {
    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{:}", entry);
    }

    Ok(())
}
