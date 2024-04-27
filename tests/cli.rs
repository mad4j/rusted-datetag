use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

use chrono::Local;

#[test]
fn test_default_args() -> Result<(), Box<dyn std::error::Error>> {
    let d = format!("{}", Local::now().naive_local().date().format("%Y%m"));

    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.assert().success().stdout(predicate::str::contains(d));

    Ok(())
}

#[test]
fn test_date_valid_default() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(["-d", "20240427"])
        .assert()
        .success()
        .stdout(predicate::str::contains("202404"));

    Ok(())
}

#[test]
fn test_date_valid_day() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(["-d", "20240427", "-t", "d"])
        .assert()
        .success()
        .stdout(predicate::str::contains("20240427"));

    Ok(())
}

#[test]
fn test_date_valid_month() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(["-d", "20240427", "-t", "m"])
        .assert()
        .success()
        .stdout(predicate::str::contains("202404"));

    Ok(())
}

#[test]
fn test_date_valid_year() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(["-d", "20240427", "-t", "y"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2024"));

    Ok(())
}

#[test]
fn test_date_valid_day_newline() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(["-d", "20240427", "-t", "d", "-n"])
        .assert()
        .success()
        .stdout(predicate::str::contains("20240427\n"));

    Ok(())
}

#[test]
fn test_date_valid_day_prefix() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(["-d", "20240427", "-t", "d", "-p", "LAB_"])
        .assert()
        .success()
        .stdout(predicate::str::contains("LAB_20240427"));

    Ok(())
}

#[test]
fn test_date_valid_day_repeat() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(["-d", "20240427", "-t", "d", "-r", "3", "-o", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("20240427\n20240428\n20240429\n"));

    Ok(())
}

#[test]
fn test_date_valid_day_offset() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(["-d", "20240427", "-t", "d", "-o", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("20240428"));

    Ok(())
}

#[test]
fn test_markdown_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(["--markdown-help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("markdown-help"));

    Ok(())
}
