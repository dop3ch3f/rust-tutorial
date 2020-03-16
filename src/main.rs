// Document Storage Service as example
#[macro_use]
extern crate quick_error;

// quick-error features
// Macro: quick_error!
// Implements Error trait
// Easier Display trait implementation
// Easier From trait implementations
// Easily add context

use quick_error::ResultExt;
use std::fs::{File, OpenOptions};
use std::{io, result};

const MAX_DOCS_CREATES_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

quick_error! {
    #[derive(Debug)]
    pub enum DocumentServiceError {
        RateLimitExceeded {
            display("You have exceeded the allowed number of documents per minute")
        }
        Io(filename: String, cause: io::Error) {
            display("I/O error: {} for filename {}", cause, filename)
            context(filename: &'a str, cause: io::Error)
                -> (filename.to_string(), cause)
        }
    }
}

// Result type alias
pub type Result<T> = result::Result<T, DocumentServiceError>;

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATES_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .context(filename)?;

    Ok(file)
}

fn create_project(project_name: &str) -> Result<()> {
    create_document(&format!("{}-draft1", project_name))?;
    create_document(&format!("{}-draft2", project_name))?;
    create_document(&format!("{}-draft3", project_name))?;
    create_document(&format!("{}-draft4", project_name))?;
    Ok(())
}

fn main() {
    match create_project("new-project") {
        Ok(()) => println!("Project created successfully"),
        Err(e) => println!("Project creation failed: {}", e),
    }
}
