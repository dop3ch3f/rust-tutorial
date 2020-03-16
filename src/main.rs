// Document Storage Service as example
#[macro_use]
extern crate error_chain;

// error chain features
// macro error_chain
// Easier display trait implementation
// easier from trait implementations
// implements error trait
// opinionated type definitions without boilerplate
// result type alias
// add context through a chain of errors
// Backtrace support

use std::fs::{File, OpenOptions};

const MAX_DOCS_CREATES_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

pub mod errors {
    error_chain! {
        errors {
            RateLimitExceeded {
                display("Maximum files")
            }
        }
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use errors::*;

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATES_PER_MINUTE {
        bail!(ErrorKind::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .chain_err(|| format!("could not open {}", filename))?;

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
        Err(e) => {
            println!("Project creation failed: {}", e);
            for e in e.iter().skip(1) {
                println!("Caused by: {}", e)
            }
            if let Some(backtrace) = e.backtrace() {
                println!("backtrace: {:?}", backtrace);
            }
        }
    }
}
