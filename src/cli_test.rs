use clap::ErrorKind;

use ::cli;

#[test]
fn should_fail_on_empty_args() {
  let result = cli::build_arg_matches_from(vec!["exec"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn should_handle_single_arg() {
  let result = cli::build_arg_matches_from(vec!["exec", "http://example.com"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(cli::ARG_URL).unwrap(), "http://example.com");
}

#[test]
fn should_handle_help() {
  let result = cli::build_arg_matches_from(vec!["exec", "--help"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::HelpDisplayed);

  let result = cli::build_arg_matches_from(vec!["exec", "-h"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn should_handle_version() {
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
