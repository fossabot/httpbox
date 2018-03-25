use clap::ErrorKind;

use ::cli;
use ::core;

#[test]
fn should_fail_on_empty_args() {
  let result = cli::build_arg_matches_from(vec!["exec"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn should_handle_mandatory_url_arg() {
  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(cli::ARG_URL).unwrap(), "http://example.com");
}

#[test]
fn should_handle_help_arg() {
  let result = cli::build_arg_matches_from(vec!["exec", "--help"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::HelpDisplayed);

  let result = cli::build_arg_matches_from(vec!["exec", "-h"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn should_handle_version_arg() {
  let result = cli::build_arg_matches_from(vec!["exec", "--version"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::VersionDisplayed);

  let result = cli::build_arg_matches_from(vec!["exec", "-V"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::VersionDisplayed);
}

#[test]
fn should_fail_for_unexpected_arg() {
  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "unexpected_2nd_arg"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn should_allow_multiple_verbose_arg() {
  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "-vvvvvv"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().occurrences_of(cli::ARG_VERBOSE), 6 as u64);
}

#[test]
fn should_handle_quiet_arg() {
  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "--quiet"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().occurrences_of(cli::ARG_QUIET), 1 as u64);

  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "-q"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().occurrences_of(cli::ARG_QUIET), 1 as u64);
}

#[test]
fn should_handle_method_arg() {
  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "-m"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(cli::ARG_METHOD).unwrap(), "GET");

  core::SUPPORTED_METHODS.iter().for_each(|method| {
    let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "-m", method.as_ref()]);
    assert!(!result.is_err());
    assert_eq!(result.unwrap().value_of(cli::ARG_METHOD).unwrap(), method.as_ref());

    let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "--method", method.as_ref()]);
    assert!(!result.is_err());
    assert_eq!(result.unwrap().value_of(cli::ARG_METHOD).unwrap(), method.as_ref());
  });
}

#[test]
fn should_failt_on_invalid_method_arg() {
  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "-m", "PEST"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidValue);

  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "--method", "PAST"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidValue);
}

#[test]
fn should_handle_output_arg() {
  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(cli::ARG_OUTPUT).unwrap(), cli::ARG_OUTPUT_DEFAULT);

  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "-o"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(cli::ARG_OUTPUT).unwrap(), cli::ARG_OUTPUT_DEFAULT);

  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com", "--output", "relative/path/to/file"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(cli::ARG_OUTPUT).unwrap(), "relative/path/to/file");
}
