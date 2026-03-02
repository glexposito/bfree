use bfree::cli::Args;
use clap::{Parser, error::ErrorKind};

#[test]
fn parses_extended_flag() {
    let args = Args::try_parse_from(["bfree", "--extended"]).expect("valid args");
    assert!(args.extended);
    assert!(!args.pretty);
    assert!(!args.json);
    assert!(!args.yaml);
}

#[test]
fn rejects_pretty_with_json() {
    let err = Args::try_parse_from(["bfree", "--pretty", "--json"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn rejects_pretty_with_extended() {
    let err = Args::try_parse_from(["bfree", "--pretty", "--extended"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn rejects_json_with_yaml() {
    let err = Args::try_parse_from(["bfree", "--json", "--yaml"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}
