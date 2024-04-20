use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

fn main() {
    #[derive(Debug, Deserialize, Serialize)]
    // FROM HERE
    // http://saidvandeklundert.net/learn/2021-09-01-rust-option-and-result/
    // search for => using other crates: anyhow
    struct Secrets {
        username: String,
        password: String,
    }

    fn get_secrets(s: &str) -> Result<Secrets> {
        let text = fs::read_to_string(s).context("Secrets file is missing.")?;
        let secrets: Secrets =
            serde_json::from_str(&text).context("Problem serializing secrets.")?;
        if secrets.password.chars().count() < 2 {
            return Err(anyhow!("Password in secrets file is too short"));
        }
        Ok(secrets)
    }
    // Ok
    let _a = get_secrets("/tmp/secrets.json");
    // Result NOT used
    _ = dbg!(_a);
    // Error Secrets file is missing
    let _b = get_secrets("missing_file.txt");
    _ = dbg!(_b);
    //Error - Problem serializing secrets
    let c = get_secrets("/tmp/invalid_json.txt");
    _ = dbg!(c);
}

/*
export FILE_NAME=18_first_anyhow.rs
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
