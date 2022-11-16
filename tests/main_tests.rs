use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn add_action_works() {
    let mut cmd = Command::cargo_bin("todo_cli").unwrap();

    cmd.arg("add").arg("algo");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("item saved"));
}

#[test]
fn add_action_failed() {
    let mut cmd = Command::cargo_bin("todo_cli").unwrap();

    cmd.arg("add");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Plase type an item"));
}

#[test]
fn complete_action_works() {
    let mut cmd = Command::cargo_bin("todo_cli").unwrap();

    cmd.arg("complete").arg("algo");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("item completed"));
}

#[test]
fn complete_action_failed() {
    let mut cmd = Command::cargo_bin("todo_cli").unwrap();

    cmd.arg("complete").arg("diferente");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(" not found"));
}
