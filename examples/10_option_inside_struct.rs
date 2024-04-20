#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    age: Option<i32>,
}

fn main() {
    let marie = Person {
        name: String::from("Marie"),
        age: Some(2),
    };

    let jan = Person {
        name: String::from("Jan"),
        age: None,
    };
    println!("{:?}\n{:?}", marie, jan);
}

/*
export FILE_NAME=10_option_inside_struct.rs
export FILE_DIR_NAME=examples
git add $FILE_DIR_NAME/$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
# cargo install --list
cargo update --workspace
cargo clippy --fix
cargo clippy --fix --examples
cargo check --verbose
cargo check --verbose --examples
cargo fmt -- --emit=files
git commit --all --message="-> Add AFTER housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
cargo run --example $(echo $FILE_NAME | cut -d . -f 1)
*/
