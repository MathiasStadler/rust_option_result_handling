fn main() {
    #[allow(unused_imports)]
    use eyre::eyre;
    #[allow(unused_imports)]
    use eyre::{EyreHandler, Report, Result, WrapErr};
    use serde::{Deserialize, Serialize};
    use std::fs;

    #[derive(Debug, Deserialize, Serialize)]
    struct Secrets {
        username: String,
        password: String,
    }

    fn get_secrets(s: &str) -> Result<Secrets> {
        let text = fs::read_to_string(s).wrap_err("Secrets file is missing.")?;
        let secrets: Secrets =
            serde_json::from_str(&text).wrap_err("Problem serializing secrets.")?;
        if secrets.username.chars().count() < 2 {
            return Err(eyre!("Username in secrets file is too short"));
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
    //Error - Problem serializing secrets
    let d = get_secrets("/tmp/secrets_short_username.json");
    _ = dbg!(d);
}

/*
export FILE_NAME=19_using_eyre.rs
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
