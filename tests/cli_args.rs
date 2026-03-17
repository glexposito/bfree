use bfree::cli::Args;
use clap::{Parser, error::ErrorKind};
use rstest::rstest;

#[test]
fn parses_with_no_flags() {
    let args = Args::try_parse_from(["bfree"]).expect("valid args");
    assert!(!args.extended);
    assert!(!args.visual);
    assert!(!args.json);
    assert!(!args.yaml);
}

#[test]
fn parses_extended_flag() {
    let args = Args::try_parse_from(["bfree", "--extended"]).expect("valid args");
    assert!(args.extended);
    assert!(!args.visual);
    assert!(!args.json);
    assert!(!args.yaml);
}

#[test]
fn parses_short_extended_flag() {
    let args = Args::try_parse_from(["bfree", "-e"]).expect("valid args");
    assert!(args.extended);
    assert!(!args.visual);
    assert!(!args.json);
    assert!(!args.yaml);
}

#[rstest]
#[case("--json")]
#[case("--yaml")]
fn rejects_visual_with_structured_formats(#[case] format_flag: &str) {
    let err = Args::try_parse_from(["bfree", "--visual", format_flag]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn rejects_visual_with_extended() {
    let err = Args::try_parse_from(["bfree", "--visual", "--extended"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn rejects_json_with_yaml() {
    let err = Args::try_parse_from(["bfree", "--json", "--yaml"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}
