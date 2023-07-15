//! The foreign function interface which exposes this library to non-Rust languages.
use crate::request::Request;
use crate::response::Resp;
use crate::send_request;
use libc::{c_char, c_int, size_t};
use reqwest::{Method, Url};
use std::ffi::CStr;
use std::ptr;
use std::slice;

/// Construct a new `Request` which will target the provided URL and fill out all other fields
/// with their defaults.
///
///
/// # Note
///
/// If the string passed in isn't a valid URL this will return a null pointer.
///
/// # Safety
///
/// Make sure you destroy the request with [`request_destroy()`] once you are done with it.
///
/// [`request_destroy()`]: fn.request_destroy.html
#[no_mangle]
pub unsafe extern "C" fn request_create(url: *const c_char) -> *mut Request {
    if url.is_null() {
        return ptr::null_mut();
    }

    let raw = CStr::from_ptr(url);

    let url_as_str = match raw.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let parsed_url = match Url::parse(url_as_str) {
        Ok(u) => u,
        Err(_) => return ptr::null_mut(),
    };

    let req = Request::new(parsed_url, Method::GET);

    println!("Request created in Rust: {}", url_as_str);
    Box::into_raw(Box::new(req))
}

/// Destroy a `Request` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn request_destroy(req: *mut Request) {
    destroy(req);
}

/// Take a reference to a `Request` and execute it, getting back the server's response.
///
/// If something goes wrong, this will return a null pointer. Don't forget to destroy the
/// `Response` once you are done with it!
#[no_mangle]
pub unsafe extern "C" fn request_send(req: *const Request) -> *mut Resp {
    if req.is_null() {
        return ptr::null_mut();
    }

    let response = match send_request(&*req) {
        Ok(r) => r,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(response))
}

/// Destroy a `Response` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn response_destroy(res: *mut Resp) {
    destroy(res);
}

unsafe fn destroy<T>(obj: *mut T) {
    if !obj.is_null() {
        drop(Box::from_raw(obj));
    }
}

/// Get the length of a `Response`'s body.
#[no_mangle]
pub unsafe extern "C" fn response_body_length(res: *const Resp) -> size_t {
    if res.is_null() {
        return 0;
    }

    (&*res).body.len() as size_t
}

/// Copy the response body into a user-provided buffer, returning the number of bytes copied.
///
/// If an error is encountered, this returns `-1`
///
/// To copy the response body to some buffer supplied by C++ we'll want to first turn it from a
/// pointer and a length into a more Rust-ic `&mut [u8]`. Luckily the `slice::from_raw_parts_mut()`
/// exists for just this purpose. We can then do the usual length checks before using
/// `ptr::copy_nonoverlapping()` to copy the buffer contents across.
///
///
/// In general, whenever you are wanting to pass data in the form of arrays from one language to
/// another, it's easiest to ask the caller to provide some buffer the data can be written into.
/// If you were to instead return a Vec<u8> or similar dynamically allocated type native to a
/// particular language, that means the caller must return that object to the language so it can be
/// free'd appropriately. This can get pretty error-prone and annoying after a while.
#[no_mangle]
pub unsafe extern "C" fn response_body(
    res: *const Resp,
    buffer: *mut c_char,
    length: size_t,
) -> c_int {
    if res.is_null() || buffer.is_null() {
        return -1;
    }

    let res = &*res;
    let buffer: &mut [u8] = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if buffer.len() < res.body.len() {
        return -1;
    }

    ptr::copy_nonoverlapping(res.body.as_ptr(), buffer.as_mut_ptr(), res.body.len());

    res.body.len() as c_int
}
