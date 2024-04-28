use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

use chrono::Local;

fn test_on_stdout(args: &str, result: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(args.split(' '))
        .assert()
        .success()
        .stdout(predicate::str::contains(result));

    Ok(())
}

#[test]
fn test_default_args() -> Result<(), Box<dyn std::error::Error>> {
    let d = format!("{}", Local::now().naive_local().date().format("%Y%m"));

    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.assert().success().stdout(predicate::str::contains(d));

    Ok(())
}

#[test]
fn test_date_valid_default() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427", "202404")
}

#[test]
fn test_date_valid_day() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427 -td", "20240427")
}

#[test]
fn test_date_valid_month() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427 -tm", "202404")
}

#[test]
fn test_date_valid_year() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427 -ty", "2024")
}

#[test]
fn test_date_valid_day_newline() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427 -td -n", "20240427\n")
}

#[test]
fn test_date_valid_day_prefix() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427 -td -p LAB_", "LAB_20240427")
}

#[test]
fn test_date_valid_day_suffix() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427 -td -x _rel", "20240427_rel")
}

#[test]
fn test_date_valid_day_prefix_and_suffix() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427 -td -p LAB- -x _rel", "LAB-20240427_rel")
}

#[test]
fn test_date_valid_day_repeat() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427 -td -r3 -o1", "20240427\n20240428\n20240429\n")
}

#[test]
fn test_date_valid_day_offset() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427 -td -o 1", "20240428")
}

#[test]
fn test_date_valid_day_offset_negative() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("-d 20240427 -td -o -1", "20240426")
}

#[test]
fn test_markdown_help() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("--markdown-help", "markdown-help")
}
