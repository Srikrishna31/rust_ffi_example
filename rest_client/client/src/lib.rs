pub mod errors;
mod request;
mod response;
pub mod utils;

extern crate chrono;
extern crate cookie;
#[macro_use]
extern crate error_chain;
extern crate fern;
extern crate libc;
#[macro_use]
extern crate log;
extern crate reqwest;

use crate::errors::*;
use request::Request;
use reqwest::Client;
use response::Resp;
use std::io::Read;

#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello World");
}

/// Send a `Request`
pub fn send_request(req: &Request) -> Result<Resp> {
    info!("Sending a GET request to {}", req.destination);
    if log_enabled!(::log::Level::Debug) {
        debug!("Sending {} Headers", req.headers.len());
        for (name, value) in req.headers.iter() {
            debug!("\t{} = {}", name, value.to_str().unwrap());
        }
        for cookie in req.cookies.iter() {
            debug!("\t{} = {}", cookie.name(), cookie.value());
        }
        trace!("{:#?}", req);
    }

    let client = Client::builder()
        .build()
        .chain_err(|| "The native TLS backend couldn't be initialized")?;

    client
        .execute(req.to_reqwest())
        .chain_err(|| "The request failed")
        .and_then(|r| Resp::from_reqwest(r))
}
