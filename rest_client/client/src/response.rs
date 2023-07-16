use crate::errors::*;
use error_chain::mock::ResultExt;
use reqwest::header::HeaderMap;
use reqwest::Response as ReqResponse;
use reqwest::{self, StatusCode};
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Resp {
    pub headers: HeaderMap,
    pub body: Vec<u8>,
    pub status: StatusCode,
}

impl Resp {
    pub(crate) fn from_reqwest(original: ReqResponse) -> Result<Self> {
        let mut original = original.error_for_status()?;
        let headers = original.headers().clone();
        let status = original.status();

        let mut body = Vec::new();
        original
            .read_to_end(&mut body)
            .chain_err(|| "Unable to read the response body")?;
        //TODO: For some strange reason, ? operator is not working with error_chain.
        // .expect("Error reading the response");

        Ok(Resp {
            status,
            body,
            headers,
        })
    }
}
