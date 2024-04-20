#[derive(Debug)]
enum Example {
    This,
    That,
}

fn matcher(x: Example) {
    match x {
        Example::This => println!("We got This. {:?}", x),
        Example::That => println!("We got That."),
    }
}

fn main() {
    let _this = Example::This;
    let _that = Example::That;

    println!("Example::This contains: {:?}", _this);
    println!("Example::That contains: {:?}", _that);

    matcher(Example::This);
    matcher(Example::That);
}
/*
export FILE_NAME=02_match_enums.rs
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
