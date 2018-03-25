// Self
use ::core;
// Third-party
use clap::{App, Arg, ArgMatches, Result as ClapResult};
#[allow(unused_imports)] use clap::ErrorKind;
use log::LevelFilter;
use reqwest::{Method, Url};
// Std
use std::env;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::{self, Result, Write};
use std::path::Path;
use std::process;
use std::str::FromStr;

pub const ARG_URL: &'static str = "URL";
pub const ARG_METHOD: &'static str = "method";
pub const ARG_OUTPUT: &'static str = "output";
pub const ARG_OUTPUT_DEFAULT: &'static str = "-";
pub const ARG_VERBOSE: &'static str = "verbose";
pub const ARG_QUIET: &'static str = "quiet";

pub struct CLI<'a> {
  arg_matches: ArgMatches<'a>
}

impl<'a> CLI<'a> {
  pub fn new() -> CLI<'a> {
    let instance = CLI {
      arg_matches: build_arg_matches_from(&mut env::args_os()).unwrap_or_else(|clap_err| {
        clap_err.exit()
      })
    };

    trace!("Parsed input arguments: {:#?}", instance.arg_matches);
    return instance;
  }

  pub fn get_method(&self) -> Method {
    let method = get_method(&self.arg_matches);
    trace!("{}: {}", ARG_METHOD, method);
    return method;
  }

  pub fn get_url(&self) -> Url {
    let url = get_url(&self.arg_matches);
    trace!("{}: {}", ARG_URL, url);
    return url;
  }

  pub fn get_output_writer(&self) -> Result<Box<Write>> {
    get_output_writer(&self.arg_matches, false)
  }

  pub fn get_log_level_filter(&self) -> LevelFilter {
    let log_level_filter = get_log_level_filter(&self.arg_matches);
    trace!("log level: {}", log_level_filter);
    return log_level_filter;
  }

  pub fn get_request_parameters(&self) -> core::RequestParameters {
    core::RequestParameters {
      url: self.get_url(),
      method: self.get_method(),
      output_writer: match self.get_output_writer() {
        Ok(boxed_writer) => boxed_writer,
        Err(err) => {
          error!("Unable to open output: {}", err);
          process::exit(1) //< TODO Define a range of error codes to pick from
        }
      },
    }
  }
}

// ------------------------------------------------------------------------------- Private Functions

fn build_arg_matches_from<'a, I, T>(iter: I) -> ClapResult<ArgMatches<'a>>
  where
    I: IntoIterator<Item=T>,
    T: Into<OsString> + Clone {
  App::new(crate_name!())
    .version(crate_version!())
    .about(crate_description!())
    .author(crate_authors!("\n"))
    .arg(Arg::with_name(ARG_URL)
      .index(1)
      .required(true)
      .help("URL to send the HTTP(S) request to"))
    .arg(Arg::with_name(ARG_METHOD)
      .short(first_char(ARG_METHOD))
      .long(ARG_METHOD)
      .value_name("METHOD")
      .default_value(Method::Get.as_ref())
      .possible_values(core::supported_methods().as_ref())
      .help("HTTP Method"))
    .arg(Arg::with_name(ARG_OUTPUT)
      .short(first_char(ARG_OUTPUT))
      .long(ARG_OUTPUT)
      .value_name("FILE")
      .default_value(ARG_OUTPUT_DEFAULT)
      .help("Write response body to file (omit or '-' for std.output)"))
    .arg(Arg::with_name(ARG_VERBOSE)
      .short(first_char(ARG_VERBOSE))
      .long(ARG_VERBOSE)
      .multiple(true)
      .required(false)
      .help("Verbosity level (can use multiple times)"))
    .arg(Arg::with_name(ARG_QUIET)
      .short(first_char(ARG_QUIET))
      .long(ARG_QUIET)
      .required(false)
      .help("Don't write anything to std.output (i.e. 'quiet mode')"))
    .get_matches_from_safe(iter)
}

fn get_method(arg_matches: &ArgMatches) -> Method {
  let arg_method = arg_matches.value_of(ARG_METHOD).unwrap();

  Method::from_str(arg_method).unwrap()
}

fn get_url(arg_matches: &ArgMatches) -> Url {
  let url_str = arg_matches.value_of(ARG_URL).unwrap();

  match Url::parse(url_str) {
    Ok(url) => url,
    Err(err) => {
      error!("Malformed URL '{}': {}", url_str, err);
      process::exit(1) //< TODO Define a range of error codes to pick from
    }
  }
}

fn get_output_writer(arg_matches: &ArgMatches, resume: bool) -> Result<Box<Write>> {
  let output_filename = arg_matches.value_of(ARG_OUTPUT).unwrap();

  if ARG_OUTPUT_DEFAULT == output_filename {
    trace!("Output will be written to Standard Output");

    Ok(Box::new(io::stdout()))
  } else if resume && Path::new(output_filename).exists() {
    trace!("Output will be appended to '{}' file (resume)", output_filename);

    match OpenOptions::new().append(true).open(output_filename) {
      Ok(file) => Ok(Box::new(file)),
      Err(error) => Err(error),
    }
  } else {
    trace!("Output will be written to '{}' file", output_filename);

    match OpenOptions::new().create(true).write(true).open(output_filename) {
      Ok(file) => Ok(Box::new(file)),
      Err(error) => Err(error),
    }
  }
}

fn get_log_level_filter(arg_matches: &ArgMatches) -> LevelFilter {
  if arg_matches.occurrences_of(ARG_QUIET) == 1 {
    LevelFilter::Warn
  } else {
    match arg_matches.occurrences_of(ARG_VERBOSE) {
      0 => LevelFilter::Info,
      1 => LevelFilter::Debug,
      2 | _ => LevelFilter::Trace
    }
  }
}

fn first_char(input: &str) -> String {
  input.chars().next().unwrap().to_string()
}

// ------------------------------------------------------------------------------ Private Unit Tests

#[test]
fn should_fail_on_empty_args() {
  let result = build_arg_matches_from(vec!["exec"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn should_handle_mandatory_url_arg() {
  let result = build_arg_matches_from(vec!["exec", "http://example.com"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(ARG_URL).unwrap(), "http://example.com");
}

#[test]
fn should_handle_help_arg() {
  let result = build_arg_matches_from(vec!["exec", "--help"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::HelpDisplayed);

  let result = build_arg_matches_from(vec!["exec", "-h"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn should_handle_version_arg() {
  let result = build_arg_matches_from(vec!["exec", "--version"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::VersionDisplayed);

  let result = build_arg_matches_from(vec!["exec", "-V"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::VersionDisplayed);
}

#[test]
fn should_fail_for_unexpected_arg() {
  let result = build_arg_matches_from(vec!["exec", "http://example.com", "unexpected_2nd_arg"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn should_allow_multiple_verbose_arg() {
  let result = build_arg_matches_from(vec!["exec", "http://example.com", "-vvvvvv"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().occurrences_of(ARG_VERBOSE), 6 as u64);
}

#[test]
fn should_handle_quiet_arg() {
  let result = build_arg_matches_from(vec!["exec", "http://example.com", "--quiet"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().occurrences_of(ARG_QUIET), 1 as u64);

  let result = build_arg_matches_from(vec!["exec", "http://example.com", "-q"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().occurrences_of(ARG_QUIET), 1 as u64);
}

#[test]
fn should_handle_method_arg() {
  let result = build_arg_matches_from(vec!["exec", "http://example.com", "-m"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(ARG_METHOD).unwrap(), "GET");

  core::SUPPORTED_METHODS.iter().for_each(|method| {
    let result = build_arg_matches_from(vec!["exec", "http://example.com", "-m", method.as_ref()]);
    assert!(!result.is_err());
    assert_eq!(result.unwrap().value_of(ARG_METHOD).unwrap(), method.as_ref());

    let result = build_arg_matches_from(vec!["exec", "http://example.com", "--method", method.as_ref()]);
    assert!(!result.is_err());
    assert_eq!(result.unwrap().value_of(ARG_METHOD).unwrap(), method.as_ref());
  });
}

#[test]
fn should_failt_on_invalid_method_arg() {
  let result = build_arg_matches_from(vec!["exec", "http://example.com", "-m", "PEST"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidValue);

  let result = build_arg_matches_from(vec!["exec", "http://example.com", "--method", "PAST"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidValue);
}

#[test]
fn should_handle_output_arg() {
  let result = build_arg_matches_from(vec!["exec", "http://example.com"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(ARG_OUTPUT).unwrap(), ARG_OUTPUT_DEFAULT);

  let result = build_arg_matches_from(vec!["exec", "http://example.com", "-o"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(ARG_OUTPUT).unwrap(), ARG_OUTPUT_DEFAULT);

  let result = build_arg_matches_from(vec!["exec", "http://example.com", "--output", "relative/path/to/file"]);
  assert!(!result.is_err());
  assert_eq!(result.unwrap().value_of(ARG_OUTPUT).unwrap(), "relative/path/to/file");
}
