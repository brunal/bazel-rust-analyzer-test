// This line will report "unused import: `std::fmt::Error`".
// However, when removing it, the report will stay until there is another
// diagnostic to report (either a syntax error or a build diagnostic).
use std::fmt::Error;

fn main() {
    println!("hello world");
}
