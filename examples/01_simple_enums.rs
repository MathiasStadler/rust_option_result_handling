fn main() {
    #[derive(Debug)]
    enum Example {
        This,
        That,
    }

    let _this = Example::This;
    let _that = Example::That;

    println!("this => {:?}", _this);
    println!("that => {:?}", _that);
}

/*
export FILE_NAME=01_simple_enums.rs
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
