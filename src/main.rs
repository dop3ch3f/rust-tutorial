// How to return Result

use std::time;

fn save_status(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("status text is too long");
    }
// How to use ?
    let record = save_to_database(text)?;

    Ok(record.id)

}

fn save_to_database(text: &str) -> Result<StatusRecord, &'static str> {
    // fake implementation that always fails
    Err("db unavailable")
}

struct StatusRecord {
    id: i64,
    text: String,
    created_at: std::time::Instant,
}

// Where ? is valid
// - it must be called in methods that return a result or an option so you can annotate methods that didnt initially to let the code pass
// Recent features involving ?

// - you can use ? on Option Values
// - same but instead return Some or None
// - can use ? to return in main because now main can return a value just anotate the return type
// - you can use question mark in tests the same way
