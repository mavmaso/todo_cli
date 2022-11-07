use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn add_actions_works() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("todo_cli")?;

    cmd.arg("add").arg("algo");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("item saved"));

    Ok(())
}

#[test]
fn add_actions_failed() {
    let mut cmd = Command::cargo_bin("todo_cli").unwrap();

    cmd.arg("add");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Plase type an item"));
}
