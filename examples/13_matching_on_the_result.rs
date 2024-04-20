fn check_length(s: &str, min: usize) -> Result<&str, String> {
    if s.chars().count() >= min {
        Ok(s)
    } else {
        Err(format!("'{}' is not long enough!", s))
    }
}

fn main() {
    // let a = check_length("some str", 5);
    // let b = check_length("another str", 300);
    // dbg!(a); // Ok("some str",)
    // dbg!(b); // Err("'another str' is not long enough!",)

    // instead /w match
    //Ok

    let func_return = check_length("some str", 5);
    let a_str = match func_return {
        Ok(a_str) => a_str,
        Err(error) => panic!("Err => Problem running 'check_length':\n {:?}", error),
        // after check_length
    };
    println!("Length is Ok -> this str is long enough! => {} <=", a_str);

    // instead /w match
    // Err
    let func_return = check_length("some string literal", 100);
    #[allow(unused_variables)]
    let _a_str = match func_return {
        Ok(a_str) => a_str,
        Err(error) => panic!("Err => Problem running 'check_length':\n {:?}", error),
    };
    // thread 'main' panicked at 'Problem running 'check_length':
    // "'some string literal' is not long enough!"'
} //end of fn main

/*
export FILE_NAME=13_matching_on_the_result.rs
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
