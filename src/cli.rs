use clap::{App, Arg, ArgMatches};
use log::LevelFilter;
use reqwest::{Method, Url};
use std::process;
use std::str::FromStr;
use std::io::{self, Write, Result};
use std::path::Path;
use std::fs::OpenOptions;

pub const ARG_URL: &'static str = "URL";
pub const ARG_METHOD: &'static str = "method";
pub const ARG_OUTPUT: &'static str = "output";
pub const ARG_OUTPUT_DEFAULT: &'static str = "STDOUT";
pub const ARG_VERBOSE: &'static str = "verbose";
pub const ARG_QUIET: &'static str = "quiet";

pub fn build_arg_matches<'a>() -> ArgMatches<'a> {
  let supported_methods = [
    Method::Get.as_ref(),
    Method::Post.as_ref(),
    Method::Put.as_ref(),
    Method::Delete.as_ref(),
    Method::Head.as_ref(),
    Method::Options.as_ref(),
    Method::Connect.as_ref(),
    Method::Trace.as_ref(),
    Method::Patch.as_ref()
  ];

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
      .possible_values(&supported_methods)
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
    .get_matches()
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
