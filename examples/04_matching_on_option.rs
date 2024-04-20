pub fn main() {
    let something: Option<&str> = Some("a String"); // Some("a String")
    let nothing: Option<&str> = None; // None

    match something {
        Some(text) => println!("We go something: {}", text),
        None => println!("We got nothing."),
    }

    match nothing {
        Some(something_else) => println!("We go something: {}", something_else),
        None => println!("We got nothing"),
    }
}

// cargo fmt -- --emit=files ./examples/04_matching_on_option.rs
// cargo run --example 04_matching_on_option
