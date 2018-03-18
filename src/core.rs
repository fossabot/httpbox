use reqwest::{Method, Url};
use std::io::Write;

pub struct RequestParameters {
  method: Method,
  url: Url,
  output_writer: Box<Write>
}

pub const SUPPORTED_METHODS: [Method; 9] = [
  Method::Get,
  Method::Post,
  Method::Put,
  Method::Delete,
  Method::Head,
  Method::Options,
  Method::Connect,
  Method::Trace,
  Method::Patch
];

pub fn supported_methods() -> Vec<&'static str> {
  SUPPORTED_METHODS.iter()
    .map(|method| method.as_ref())
    .collect()
}
