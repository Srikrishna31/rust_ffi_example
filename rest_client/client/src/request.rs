use reqwest::{self, header::HeaderMap, Method, Url};
//use reqwest::cookies::;
use cookie::CookieJar;
use reqwest::cookie::Cookie;
/// A HTTP Request.
#[derive(Debug, Clone)]
pub struct Request {
    pub destination: Url,
    pub method: Method,
    pub headers: HeaderMap,
    pub cookies: CookieJar,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(destination: Url, method: Method) -> Self {
        Self {
            destination,
            method,
            headers: HeaderMap::default(),
            cookies: CookieJar::default(),
            body: None,
        }
    }

    pub(crate) fn to_reqwest(&self) -> reqwest::Request {
        let mut r = reqwest::Request::new(self.method.clone(), self.destination.clone());

        for (name, value) in &self.headers {
            // assumption is that each header has a name associated with it
            r.headers_mut().append(name, value.clone());
        }

        // TODO: Figure out copying of cookies
        // let mut cookie_header = Cookie::new("", "");
        // r.headers_mut().append(
        //     "Cookie: ",
        //     self.cookies
        //         .iter()
        //         .fold("".to_string(), |acc, val| {
        //             acc + &*format!(" {}={};", val.name(), val.value())
        //         })
        //         .parse()
        //         .unwrap(),
        // );

        // for cookie in self.cookies.iter() {
        //     cookie_header.set(cookie.name().to_owned(), cookie.value().to_owned());
        // }
        // r.headers_mut().set(cookie_header);

        r
    }
}
