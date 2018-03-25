// Self
use ::core;
// Third-party
use clap::{App, Arg, ArgMatches};
use clap::Result as ClapResult;
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
pub const ARG_OUTPUT_DEFAULT: &'static str = "STDOUT";
pub const ARG_VERBOSE: &'static str = "verbose";
pub const ARG_QUIET: &'static str = "quiet";

pub fn build_arg_matches<'a>() -> ClapResult<ArgMatches<'a>> {
  build_arg_matches_from(&mut env::args_os())
}

pub fn build_arg_matches_from<'a, I, T>(iter: I) -> ClapResult<ArgMatches<'a>>
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
      .help("Write output to FILE"))
    .arg(Arg::with_name(ARG_VERBOSE)
      .short(first_char(ARG_VERBOSE))
      .long(ARG_VERBOSE)
      .multiple(true)
      .required(false)
      .help("Verbosity level (can specify multiple times)"))
    .arg(Arg::with_name(ARG_QUIET)
      .short(first_char(ARG_QUIET))
      .long(ARG_QUIET)
      .required(false)
      .help("Don't write anything to standard output (i.e. 'quiet mode')"))
    .get_matches_from_safe(iter)
}

pub fn get_method(arg_matches: &ArgMatches) -> Method {
  let arg_method = arg_matches.value_of(ARG_METHOD).unwrap();

  Method::from_str(arg_method).unwrap()
}

pub fn get_url(arg_matches: &ArgMatches) -> Url {
  let url_str = arg_matches.value_of(ARG_URL).unwrap();

  match Url::parse(url_str) {
    Ok(url) => url,
    Err(err) => {
      error!("Malformed URL '{}': {}", url_str, err);
      process::exit(1) //< TODO Define a range of error codes to pick from
    }
  }
}

pub fn get_output_writer(arg_matches: &ArgMatches, resume: bool) -> Result<Box<Write>> {
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

pub fn get_log_level_filter(arg_matches: &ArgMatches) -> LevelFilter {
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
