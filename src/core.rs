use reqwest::{Method, Url};
use std::io::Write;
use std::borrow::BorrowMut;

pub struct RequestParameters {
  pub method: Method,
  pub url: Url,
  pub output_writer: Box<Write>
}

impl RequestParameters {
  pub fn borrow_mut_output_writer(&mut self) -> &mut Box<Write> {
    self.output_writer.borrow_mut()
  }
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
