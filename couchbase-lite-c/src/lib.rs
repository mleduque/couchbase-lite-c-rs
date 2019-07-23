extern crate couchbase_lite_c_sys as ffi;
extern crate time;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use core::ptr;
use std::mem;
use std::os::raw::c_void;
use std::str;

pub use database::*;
pub use document::*;
pub use errors::*;
pub use query::*;
pub use replicator::*;
pub use resultset::*;
use std::mem::MaybeUninit;

mod database;
mod document;
mod errors;
mod query;
mod replicator;
mod resultset;

fn init_error() -> ffi::CBLError{
    unsafe { MaybeUninit::<ffi::CBLError>::uninit().assume_init() }
}

/// Convert a native string to a Rust string
fn to_string(pointer: *const c_char) -> String {
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    str::from_utf8(slice).unwrap().to_string()
}

/// Convert a Rust string to a native string
fn to_ptr(string: String) -> *const c_char {
    let cs = CString::new(string.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    // Tell Rust not to clean up the string while we still have a pointer to it.
    // Otherwise, we'll get a segfault.
    mem::forget(cs);
    ptr
}
