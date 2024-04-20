// FROM HERE
// https://github.com/saidvandeklundert/LearningRust/blob/master/blog/option-and-result/src/main.rs

fn main() {
    let something: Option<&str> = Some("Something");
    #[allow(clippy::unnecessary_literal_unwrap)]
    let unwrapped = something.unwrap(); // "Something"
    println!("{:?}\n{:?}", something, unwrapped);
    let _nothing: Option<&str> = None;
    // uncommenting the next line will cause a panic:
    //nothing.unwrap();
}

/*
export FILE_NAME=05_option_unwrap_two.rs
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
