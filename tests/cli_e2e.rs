use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

#[test]
fn default_output_contains_mem_and_swap_sections() {
    let mut cmd = cargo_bin_cmd!("bfree");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("mem"))
        .stdout(predicate::str::contains("swap"));
}

#[test]
fn extended_flag_outputs_multiline_sections() {
    let mut cmd = cargo_bin_cmd!("bfree");
    cmd.arg("--extended")
        .assert()
        .success()
        .stdout(predicate::str::contains("memory"))
        .stdout(predicate::str::contains("swap"))
        .stdout(predicate::str::contains("cache breakdown"))
        .stdout(predicate::str::contains("█").not());
}

#[test]
fn pretty_flag_outputs_pretty_sections() {
    let mut cmd = cargo_bin_cmd!("bfree");
    cmd.arg("--pretty")
        .assert()
        .success()
        .stdout(predicate::str::contains("memory"))
        .stdout(predicate::str::contains("swap"))
        .stdout(predicate::str::contains("█"));
}

#[test]
fn json_flag_outputs_valid_json_document() {
    let mut cmd = cargo_bin_cmd!("bfree");
    let output = cmd
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let parsed: JsonValue = serde_json::from_slice(&output).expect("stdout should be valid json");

    assert!(parsed.get("memory").is_some());
    assert!(parsed.get("swap").is_some());
}

#[test]
fn yaml_flag_outputs_valid_yaml_document() {
    let mut cmd = cargo_bin_cmd!("bfree");
    let output = cmd
        .arg("--yaml")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let parsed: YamlValue = serde_yaml::from_slice(&output).expect("stdout should be valid yaml");

    assert!(parsed.get("memory").is_some());
    assert!(parsed.get("swap").is_some());
}
