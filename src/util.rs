use std::ffi::{CStr, CString};
use std::ptr::null;

#[allow(dangling_pointers_from_temporaries)]
pub unsafe fn from_c_str(s: *const std::ffi::c_char) -> String { CStr::from_ptr(s).to_string_lossy().into_owned() }


///
pub trait From<T> {
  fn from(val: T) -> Self;
}

impl From<i32> for std::ffi::c_int {
  fn from(val: i32) -> std::ffi::c_int { val }
}

impl From<i8> for std::ffi::c_char {
  fn from(val: i8) -> std::ffi::c_char { val }
}

impl From<f64> for std::ffi::c_double {
  fn from(val: f64) -> std::ffi::c_double { val }
}

impl From<String> for copt_sys::c_str {
  #[allow(dangling_pointers_from_temporaries)]
  fn from(val: String) -> copt_sys::c_str { CString::new(val.as_str()).unwrap().as_ptr() }
}


/// make an empty instance.
pub trait Init {
  fn init() -> Self;
}

impl Init for std::ffi::c_char {
  fn init() -> std::ffi::c_char { 0 }
}

impl Init for std::ffi::c_int {
  fn init() -> i32 { 0 }
}

impl Init for std::ffi::c_double {
  fn init() -> std::ffi::c_double { 0.0 }
}

impl Init for Vec<std::ffi::c_char> {
  fn init() -> Vec<std::ffi::c_char> { Vec::with_capacity(4096) }
}

impl Init for copt_sys::c_str {
  fn init() -> copt_sys::c_str { null() }
}


/// convert into different type.
pub trait Into<T> {
  fn into(self) -> T;
}

impl Into<i32> for std::ffi::c_int {
  fn into(self) -> i32 { self }
}

impl Into<f64> for std::ffi::c_double {
  fn into(self) -> f64 { self }
}

impl Into<String> for Vec<std::ffi::c_char> {
  fn into(self) -> String { unsafe { from_c_str(self.as_ptr()) } }
}

impl Into<i8> for std::ffi::c_char {
  fn into(self) -> i8 { self }
}

impl Into<String> for copt_sys::c_str {
  fn into(self) -> String { unsafe { from_c_str(self).to_owned() } }
}


/// convert to Raw C Pointer.
pub trait AsRawPtr<T> {
  fn as_rawptr(&mut self) -> T;
}

impl AsRawPtr<*mut std::ffi::c_int> for i32 {
  fn as_rawptr(&mut self) -> *mut std::ffi::c_int { self }
}

impl AsRawPtr<*mut std::ffi::c_char> for i8 {
  fn as_rawptr(&mut self) -> *mut std::ffi::c_char { self }
}

impl AsRawPtr<*mut std::ffi::c_double> for f64 {
  fn as_rawptr(&mut self) -> *mut std::ffi::c_double { self }
}

impl AsRawPtr<*mut copt_sys::c_str> for copt_sys::c_str {
  fn as_rawptr(&mut self) -> *mut copt_sys::c_str { self }
}

impl AsRawPtr<*mut std::ffi::c_char> for Vec<std::ffi::c_char> {
  fn as_rawptr(&mut self) -> *mut std::ffi::c_char { self.as_mut_ptr() }
}


///
pub trait FromRaw<T> {
  fn from(val:T) -> Self;
}

impl FromRaw<i32> for std::ffi::c_int {
  fn from(val: i32) -> std::ffi::c_int { val }
}

impl FromRaw<f64> for std::ffi::c_double {
  fn from(val: f64) -> std::ffi::c_double { val }
}

impl FromRaw<String> for copt_sys::c_str {
  #[allow(dangling_pointers_from_temporaries)]
  fn from(val: String) -> *const std::ffi::c_char { CString::new(val.as_str()).unwrap().as_ptr() }
}