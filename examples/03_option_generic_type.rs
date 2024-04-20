#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let a_str: Option<&str> = Some("a str");
    let a_string: Option<String> = Some(String::from("a String"));
    let a_float: Option<f64> = Some(1.1);
    let a_vec: Option<Vec<i32>> = Some(vec![0, 1, 2, 3]);

    let marie = Person {
        name: String::from("Marie"),
        age: 2,
    };

    let a_person: Option<Person> = Some(marie);
    let maybe_someone: Option<Person> = None;

    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        a_str, a_string, a_float, a_vec, a_person, maybe_someone
    );
}

/*
export FILE_NAME=03_option_generic_type.rs
export FILE_DIR_NAME=examples
git add $FILE_DIR_NAME/$FILE_NAME
git commit --all --message="add BEFORE housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
cargo clippy --fix
cargo clippy --fix --examples
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
cargo run --example $(echo $FILE_NAME | cut -d . -f 1)
*/
