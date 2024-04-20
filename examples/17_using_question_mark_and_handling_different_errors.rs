use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

// Debug allows us to print the struct.
// Deserialize and Serialize adds decoder and encoder capabilities to the struct.
#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: usize,
}

fn file_to_json(s: &str) -> Result<Person, Box<dyn Error>> {
    let text = fs::read_to_string(s)?;
    let marie: Person = serde_json::from_str(&text)?;
    Ok(marie)
}

fn main() {
    // _ <- underline avoid err
    // warning: unused  that must be used dbg!
    // marker with underline
    let _x = file_to_json("/tmp/valid_json.txt");
    let _y = file_to_json("/tmp/invalid_json.txt");
    let _z = file_to_json("/tmp/non_existing_file.txt");

    _ = dbg!(_x);
    _ = dbg!(_y);
    _ = dbg!(_z);
}

// # create test json files before run => 16_generate_json_file.rs
// ls -l /tmp/*json.txt

/*
export FILE_NAME=17_using_question_mark_and_handling_different_errors.rs
export FILE_DIR_NAME=examples
git add $FILE_DIR_NAME/$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
# cargo install --list
# cargo update --workspace
cargo clippy --fix
cargo clippy --fix --examples
# cargo check --verbose
# cargo check --verbose --examples
cargo check
cargo check --examples
cargo fmt -- --emit=files
git commit --all --message="-> Add AFTER housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
cargo run --example $(echo $FILE_NAME | cut -d . -f 1)
*/
