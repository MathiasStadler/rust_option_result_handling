// FROM HERE
// https://stackoverflow.com/questions/73505520/how-to-write-a-string-to-file

use std::fs::OpenOptions;
use std::io::Write;
fn main() {
    let valid_json_string = r#"{
        "name": "MyName",
        "age": 69
    }
    "#;
    // create valid file
    let mut valid_test_file = OpenOptions::new()
        .truncate(true)
        .create(true)
        .read(true)
        .write(true)
        .open("/tmp/valid_json.txt")
        .unwrap();
    //write valid json sting to file
    let eg = valid_test_file.write_all(valid_json_string.as_bytes());
    match eg {
        Ok(()) => println!("OK"),
        Err(e) => println!("{:?}", e),
    }
    //create invalid file
    #[allow(unused_variables)]
    let invalid_json_string = r#"{
        invalid_key_NOT_STRING : "MyName",
        "age": 69
    }
    "#;
    // invalid
    let mut invalid_test_file = OpenOptions::new()
        .truncate(true)
        // .create_new(true)
        .create(true)
        .read(true)
        .write(true)
        .open("/tmp/invalid_json.txt")
        .unwrap();

    //  write invalid json sting to file
    let eg = invalid_test_file.write_all(invalid_json_string.as_bytes());
    match eg {
        Ok(()) => println!("OK"),
        Err(e) => println!("{:?}", e),
    }

    //create secrets.json
    #[allow(unused_variables)]
    let secrets_json_string = r#"{
        "username": "username",
        "password": "password"
    }
    "#;
    // secrets
    let mut secrets_test_file = OpenOptions::new()
        .truncate(true)
        // .create_new(true)
        .create(true)
        .read(true)
        .write(true)
        .open("/tmp/secrets.json")
        .unwrap();

    //  write json sting to file
    let eg = secrets_test_file.write_all(secrets_json_string.as_bytes());
    match eg {
        Ok(()) => println!("OK"),
        Err(e) => println!("{:?}", e),
    }

    // create /tmp/secrets_short_username.json
    #[allow(unused_variables)]
    let secrets_short_username_json_string = r#"{
        "username": "u",
        "password": "password"
    }
    "#;
    // secrets
    let mut secrets_short_username_test_file = OpenOptions::new()
        .truncate(true)
        // .create_new(true)
        .create(true)
        .read(true)
        .write(true)
        .open("/tmp/secrets_short_username.json")
        .unwrap();

    //  write json sting to file
    let eg =
        secrets_short_username_test_file.write_all(secrets_short_username_json_string.as_bytes());
    match eg {
        Ok(()) => println!("OK"),
        Err(e) => println!("{:?}", e),
    }
}

/*
export FILE_NAME=16_generate_json_file.rs
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
