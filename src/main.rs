// Custom Error types

// Solution 1: Returning a Box<Error> trait object
// use std::env;
// use std::error::Error;
// fn num_threads() -> Result<usize, Box<dyn Error>> {
//     let s = env::var("NUM_THREADS")?;
//     let n: usize = s.parse()?;
//     Ok(n + 1)
// }

// fn run_application() -> Result<(), Box<dyn Error>> {
//     let num = num_threads()?;
//     println!("the number of threads is {}", num);
//     Ok(())
// }

// fn main() {
//     if let Err(e) = run_application() {
//         panic!("an error occured: {}", e);
//     }
// }

// Downside
// - Can't inspect the error type in code
// - Can't decide to handle different errors differently

// Solution 2: Defining a custom error type

// Document Storage Service as example

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

const MAX_DOCS_CREATES_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

#[derive(Debug)]
pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),
}

impl Error for DocumentServiceError {}

impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => write!(f, "Max number of docs per minute"),
            Io(ref io) => write!(f, "I/O error: {}", io),
        }
    }
}

impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

// Result type alias
use std::result;

pub type Result<T> = result::Result<T, DocumentServiceError>;

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATES_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

fn main() {}
