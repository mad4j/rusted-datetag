use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

use chrono::Local;

#[test]
fn test_default_args() -> Result<(), Box<dyn std::error::Error>> {
    let d = format!("{}", Local::now().naive_local().date().format("%Y%m"));

    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.assert().success().stdout(predicate::str::contains(d));

    Ok(())
}
