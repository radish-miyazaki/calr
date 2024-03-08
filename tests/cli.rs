use assert_cmd::Command;
use predicates::prelude::*;
use std::{error::Error, fs};

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "calr";

#[test]
fn dies_year_0() -> TestResult {
    let expected = "year \"0\" not in the range 1 through 9999";
    Command::cargo_bin(PRG)?
        .arg("0")
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn dies_year_13() -> TestResult {
    let expected = "year \"10000\" not in the range 1 through 9999";
    Command::cargo_bin(PRG)?
        .arg("10000")
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn dies_invalid_year() -> TestResult {
    let expected = "invalid value 'foo' for '[YEAR]'";
    Command::cargo_bin(PRG)?
        .arg("foo")
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn dies_month_0() -> TestResult {
    let expected = "month \"0\" not in the range 1 through 12";
    Command::cargo_bin(PRG)?
        .args(["-m", "0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn dies_month_13() -> TestResult {
    let expected = "month \"13\" not in the range 1 through 12";
    Command::cargo_bin(PRG)?
        .args(["-m", "13"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn dies_invalid_month() -> TestResult {
    let expected = "invalid month \"foo\"";
    Command::cargo_bin(PRG)?
        .args(["-m", "foo"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn dies_y_and_month() -> TestResult {
    let expected = "the argument '--month <MONTH>' cannot be used with '--year'";
    Command::cargo_bin(PRG)?
        .args(["-m", "1", "-y"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn dies_y_and_year() -> TestResult {
    let expected = "the argument '--year' cannot be used with '[YEAR]'";
    Command::cargo_bin(PRG)?
        .args(["-y", "2000"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn month_num() -> TestResult {
    let expected = &[
        ("1", "1月"),
        ("2", "2月"),
        ("3", "3月"),
        ("4", "4月"),
        ("5", "5月"),
        ("6", "6月"),
        ("7", "7月"),
        ("8", "8月"),
        ("9", "9月"),
        ("10", "10月"),
        ("11", "11月"),
        ("12", "12月"),
    ];

    for (num, month) in expected {
        Command::cargo_bin(PRG)?
            .args(["-m", num])
            .assert()
            .success()
            .stdout(predicates::str::contains(month.to_string()));
    }
    Ok(())
}

#[test]
fn partial_month() -> TestResult {
    let expected = &[
        ("ja", "1月"),
        ("f", "2月"),
        ("mar", "3月"),
        ("ap", "4月"),
        ("may", "5月"),
        ("jun", "6月"),
        ("jul", "7月"),
        ("au", "8月"),
        ("s", "9月"),
        ("o", "10月"),
        ("n", "11月"),
        ("d", "12月"),
    ];

    for (arg, month) in expected {
        Command::cargo_bin(PRG)?
            .args(["-m", arg])
            .assert()
            .success()
            .stdout(predicates::str::contains(month.to_string()));
    }
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn default_one_month() -> TestResult {
    let cmd = Command::cargo_bin(PRG)?.assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let lines: Vec<_> = stdout.split('\n').collect();
    assert_eq!(lines.len(), 9);
    assert_eq!(lines[0].chars().count(), 22);
    Ok(())
}

#[test]
fn test_2_2020_leap_year() -> TestResult {
    run(&["-m", "2", "2020"], "tests/expected/2-2020.txt")
}

#[test]
fn test_4_2023() -> TestResult {
    run(&["-m", "4", "2023"], "tests/expected/4-2023.txt")
}

#[test]
fn test_april_2023() -> TestResult {
    run(&["2023", "-m", "april"], "tests/expected/4-2023.txt")
}

#[test]
fn test_2023() -> TestResult {
    run(&["2023"], "tests/expected/2023.txt")
}

#[test]
fn year() -> TestResult {
    let cmd = Command::cargo_bin(PRG)?.arg("-y").assert().success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let lines: Vec<&str> = stdout.split('\n').collect();
    assert_eq!(lines.len(), 37);
    Ok(())
}
