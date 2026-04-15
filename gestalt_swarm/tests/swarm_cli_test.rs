use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_swarm_help() {
    let mut cmd = Command::cargo_bin("swarm").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Gestalt Swarm CLI"));
}

#[test]
fn test_swarm_status() {
    let mut cmd = Command::cargo_bin("swarm").unwrap();
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Gestalt Swarm is active and ready."));
}

#[test]
fn test_swarm_run() {
    let mut cmd = Command::cargo_bin("swarm").unwrap();
    cmd.arg("run").arg("--goal").arg("test_goal");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Executing goal: test_goal"));
}

#[test]
fn test_swarm_verbose() {
    let mut cmd = Command::cargo_bin("swarm").unwrap();
    cmd.arg("--verbose").arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Gestalt Swarm is active and ready."));
}
