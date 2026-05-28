//! End-to-end footer test: real fixture → real tokens → real footer.

use headroom_xray::footer;
use headroom_xray::tokenize::count_by_tool;
use headroom_xray::transcripts::claude_code;
use std::path::PathBuf;

#[test]
fn footer_renders_top3_with_hints() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("claude_code_minimal.jsonl");
    let t = claude_code::parse(&fixture).expect("parse");
    let counts = count_by_tool(&t).expect("count");
    let rendered = footer::render(&counts);

    assert!(rendered.contains("Headroom: top compression opportunities"));
    assert!(
        rendered.contains("Bash") || rendered.contains("Read"),
        "footer missing tool rows:\n{rendered}"
    );
    assert!(rendered.contains("coming soon"));
    assert!(rendered.contains("────"));
}

use assert_cmd::Command;
use predicates::str::contains;
use predicates::prelude::PredicateBooleanExt;

fn has_node() -> bool {
    std::process::Command::new("node")
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

#[test]
fn no_footer_flag_suppresses() {
    if !has_node() {
        eprintln!("[skip] node not on PATH");
        return;
    }
    Command::cargo_bin("headroom-xray")
        .unwrap()
        .args(["--no-footer", "today", "--format", "json"])
        .assert()
        .stdout(contains("Headroom: top compression").not());
}

#[test]
fn env_var_suppresses() {
    if !has_node() {
        eprintln!("[skip] node not on PATH");
        return;
    }
    Command::cargo_bin("headroom-xray")
        .unwrap()
        .env("HEADROOM_XRAY_NO_FOOTER", "1")
        .args(["today", "--format", "json"])
        .assert()
        .stdout(contains("Headroom: top compression").not());
}
