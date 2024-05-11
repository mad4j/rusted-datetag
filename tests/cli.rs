use assert_cmd::prelude::*;
use assert_fs::prelude::*;
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

fn test_on_stderr(args: &str, result: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("datetag")?;

    cmd.args(args.split(' '))
        .assert()
        .failure()
        .stderr(predicate::str::contains(result));

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
fn test_file_valid_default() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let date = format!("{}", Local::now().naive_local().date().format("%Y%m"));
    let args = format!("-f {}", file.path().to_str().unwrap_or_default());

    test_on_stdout(&args, &date)
}

#[test]
fn test_file_invalid_default() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stderr("-f fake.txt", "Error")
}

#[test]
fn test_date_valid_default() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427", "202404")
}

#[test]
fn test_date_valid_day() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td", "20240427")
}

#[test]
fn test_date_valid_month() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -tm", "202404")
}

#[test]
fn test_date_valid_year() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -ty", "2024")
}

#[test]
fn test_date_valid_day_dot() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -s dot", "2024.04.27")
}

#[test]
fn test_date_valid_month_dot() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -tm -s dot", "2024.04")
}

#[test]
fn test_date_valid_year_dot() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -ty -s dot", "2024")
}

#[test]
fn test_date_valid_day_dash() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -s dash", "2024-04-27")
}

#[test]
fn test_date_valid_month_dash() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -tm -s dash", "2024-04")
}

#[test]
fn test_date_valid_year_dash() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -ty -s dash", "2024")
}

#[test]
fn test_date_valid_day_colon() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -s colon", "2024:04:27")
}

#[test]
fn test_date_valid_month_colon() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -tm -s colon", "2024:04")
}

#[test]
fn test_date_valid_year_colon() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -ty -s colon", "2024")
}

#[test]
fn test_date_valid_day_slash() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -s slash", "2024/04/27")
}

#[test]
fn test_date_valid_month_slash() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -tm -s slash", "2024/04")
}

#[test]
fn test_date_valid_year_slash() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -ty -s slash", "2024")
}

#[test]
fn test_date_valid_day_newline() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -n", "20240427\n")
}

#[test]
fn test_date_valid_day_prefix() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -p LAB_", "LAB_20240427")
}

#[test]
fn test_date_valid_day_suffix() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -x _rel", "20240427_rel")
}

#[test]
fn test_date_valid_day_prefix_and_suffix() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -p LAB- -x _rel", "LAB-20240427_rel")
}

#[test]
fn test_date_valid_day_repeat() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -r3 -o1", "20240427\n20240428\n20240429\n")
}

#[test]
fn test_date_valid_day_offset() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -o1", "20240428")
}

#[test]
fn test_date_valid_day_offset_negative() -> Result<(), Box<dyn std::error::Error>> {
    test_on_stdout("20240427 -td -o -1", "20240426")
}
