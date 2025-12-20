//! Integration tests for Gestalt Timeline CLI
//!
//! These tests verify CLI command parsing without requiring SurrealDB.

use std::process::Command;

fn gestalt_help(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_gestalt"))
        .args(args)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_main_help() {
    let output = gestalt_help(&["--help"]);
    assert!(output.contains("Meta-Agent") || output.contains("gestalt"), "Should show app name");
    assert!(output.contains("add-project"), "Should list add-project command");
    assert!(output.contains("add-task"), "Should list add-task command");
    assert!(output.contains("timeline"), "Should list timeline command");
    assert!(output.contains("watch"), "Should list watch command");
    assert!(output.contains("broadcast"), "Should list broadcast command");
    assert!(output.contains("list-agents"), "Should list list-agents command");
}

#[test]
fn test_add_project_help() {
    let output = gestalt_help(&["add-project", "--help"]);
    assert!(output.contains("Register a new project"), "Should show command description");
}

#[test]
fn test_add_task_help() {
    let output = gestalt_help(&["add-task", "--help"]);
    assert!(output.contains("Add a task"), "Should show command description");
}

#[test]
fn test_watch_help() {
    let output = gestalt_help(&["watch", "--help"]);
    assert!(output.contains("Watch"), "Should show command description");
    assert!(output.contains("--project"), "Should show project option");
    assert!(output.contains("--events"), "Should show events option");
}

#[test]
fn test_broadcast_help() {
    let output = gestalt_help(&["broadcast", "--help"]);
    assert!(output.contains("Broadcast"), "Should show command description");
}

#[test]
fn test_timeline_help() {
    let output = gestalt_help(&["timeline", "--help"]);
    assert!(output.contains("timeline"), "Should show command name");
    assert!(output.contains("--since"), "Should show since option");
}

#[test]
fn test_agent_connect_help() {
    let output = gestalt_help(&["agent-connect", "--help"]);
    assert!(output.contains("agent"), "Should show command description");
    assert!(output.contains("--name"), "Should show name option");
}

#[test]
fn test_list_agents_help() {
    let output = gestalt_help(&["list-agents", "--help"]);
    assert!(output.contains("agents"), "Should show command description");
    assert!(output.contains("--online"), "Should show online option");
}

#[test]
fn test_subscribe_help() {
    let output = gestalt_help(&["subscribe", "--help"]);
    assert!(output.contains("Subscribe"), "Should show command description");
}

#[test]
fn test_json_flag_recognized() {
    // Test that --json is a valid global flag
    let output = Command::new(env!("CARGO_BIN_EXE_gestalt"))
        .args(["--help"])
        .output()
        .expect("Failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--json"), "Should recognize --json flag");
}
