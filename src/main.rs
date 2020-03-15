// Seperating bugs from recoverable errors

// Panic
// - bad state that shouldn't happen
// - no recovery
// - bug that a programmer must fix

// Result
// - Bad state that might happen in normal use
// - recovery is a possiblity
// - end user might be able to fix

// Advantages from rusts error handling
// - loud failures
// - fail fasts (prevents hacking and easy debugs)
// - enforce assumptions (the expect method helps to easily panic with a value on methods that return a type result and option)
//   let config = File::open("config.toml").expect("Config file not created on startup")

// - possibility of errors is clear. once you see a result it can fail
// - compiler forces handling

// // in contrast with go
// // return (success value, error value) not one or another
// // check wether the error value is nil after each function call
// // no compiler enforcement

// Disadvantages
// -it can be annoying to always return errors
// - use expect a lot
