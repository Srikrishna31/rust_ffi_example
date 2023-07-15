//! The foreign function interface which exposes this library to non-Rust languages.
use crate::errors::*;
use crate::request::Request;
use crate::response::Resp;
use crate::send_request;
use libc::{c_char, c_int, size_t};
use reqwest::{Method, Url};
use std::cell::RefCell;
use std::error::Error as StdError;
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
        let err = Error::from("No URL provided");
        update_last_error(err);
        return ptr::null_mut();
    }

    let raw = CStr::from_ptr(url);

    let url_as_str = match raw.to_str() {
        Ok(s) => s,
        Err(e) => {
            let err = Error::with_chain(e, "Unable to convert URL to a UTF-8 string");
            update_last_error(err);
            return ptr::null_mut();
        }
    };

    let parsed_url = match Url::parse(url_as_str) {
        Ok(u) => u,
        Err(e) => {
            let err = Error::with_chain(e, "Unable to parse the URL");
            update_last_error(err);
            return ptr::null_mut();
        }
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
        let err = Error::from("Null request object provided");
        update_last_error(err);
        return ptr::null_mut();
    }

    let response = match send_request(&*req) {
        Ok(r) => r,
        Err(e) => {
            let err = Error::with_chain(e, "Error sending the request");
            update_last_error(err);
            return ptr::null_mut();
        }
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
        let err = Error::from("Null response object provided");
        update_last_error(err);
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
        let err = Error::from("Error: received either a null response object or a null buffer");
        update_last_error(err);
        return -1;
    }

    let res = &*res;
    let buffer: &mut [u8] = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if buffer.len() < res.body.len() {
        let err = Error::from("Couldn't copy the expected length of bytes");
        update_last_error(err);
        return -1;
    }

    ptr::copy_nonoverlapping(res.body.as_ptr(), buffer.as_mut_ptr(), res.body.len());

    res.body.len() as c_int
}

thread_local! {
    static LAST_ERROR: RefCell<Option<Box<dyn StdError>>> = RefCell::new(None);
}

/// Update the most recent error, clearing whatever may have been there before.
pub fn update_last_error<E: StdError + 'static>(err: E) {
    error!("Setting LAST_ERROR: {err}");

    {
        // Print a pseudo-backtrace for this error, following back each error's cause until we
        // reach the root error.
        let mut cause = err.cause();
        while let Some(parent_err) = cause {
            warn!("Caused by: {parent_err}");
            cause = parent_err.cause();
        }
    }

    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

/// Retrieve the most recent error, clearing it in the process.
pub fn take_last_error() -> Option<Box<dyn StdError>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

/// Calculate the number of bytes in the last error's error message **not** including any trailing
/// `null` characters
#[no_mangle]
pub extern "C" fn last_error_length() -> c_int {
    LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len() as c_int + 1,
        None => 0,
    })
}

/// Write the most recent error message into a caller-provided buffer as a UTF-8 string, returning
/// the number of bytes written.
///
/// # Note
/// This writes a **UTF-8** string into the buffer. Windows users may need to convert it to a UTF-16
/// "unicode" afterwards.
///
/// If there are no recent errors then this returns `0` (because we wrote 0 bytes). `-1` is returned
/// if there are any errors, for example when passed a null pointer or a buffer of insufficient size.
///
/// # Side Note
///
/// Writing into borrowed buffers instead of returning an owned object is a common pattern when doing
/// FFI. It helps simplify things and avoid errors due to object lifetimes(in general, not just in the
/// Rust sense of the word) and forgetting to call destructors.
#[no_mangle]
pub unsafe extern "C" fn last_error_message(buffer: *mut c_char, length: c_int) -> c_int {
    if buffer.is_null() {
        warn!("Null pointer passed into last_error_message() as the buffer");
        return -1;
    }

    let last_error = match take_last_error() {
        Some(err) => err,
        None => return 0,
    };

    let error_message = last_error.to_string();

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if error_message.len() >= buffer.len() {
        warn!("Buffer provided for writing the last error message is too small.");
        warn!(
            "Expected at least {} bytes but got {}",
            error_message.len() + 1,
            buffer.len()
        );
        return -1;
    }

    ptr::copy_nonoverlapping(
        error_message.as_ptr(),
        buffer.as_mut_ptr(),
        error_message.len(),
    );

    // Add a trailing null so people using the string as a `char *` don't accidentally read into garbage.
    buffer[error_message.len()] = 0;

    error_message.len() as c_int
}
