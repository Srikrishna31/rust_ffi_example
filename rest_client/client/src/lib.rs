pub mod errors;
pub mod ffi;
mod plugin_manager;
mod request;
mod request_response_plugin;
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

pub use request_response_plugin::RequestResponsePlugin;
// Re-export this definition so that plugin implementations can use this.
pub use plugin_framework::declare_plugin;

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
