use std::ffi::CString;
use copt_sys::{DoubleAttr, IntAttr};

use crate::util;

pub trait Attr: Into<CString> {
    type Out;
    type Buf: util::Init + util::Into<Self::Out> + util::AsRawPtr<Self::RawGet>;
    type RawGet;
    type RawSet: util::From<Self::Out>;
  
    unsafe fn get_attr(model: *mut copt_sys::copt_prob, attrname: copt_sys::c_str, value: Self::RawGet) -> std::ffi::c_int;
  
    //unsafe fn set_attr(model: *mut copt_sys::copt_prob, attrname: copt_sys::c_str, value: Self::RawSet) -> std::ffi::c_int;
  }

  impl Attr for IntAttr {
    type Out = i32;
    type Buf = i32;
    type RawGet = *mut std::ffi::c_int;
    type RawSet = std::ffi::c_int;
  
    unsafe fn get_attr(model: *mut copt_sys::copt_prob, attrname: copt_sys::c_str, value: *mut std::ffi::c_int) -> std::ffi::c_int {
        copt_sys::COPT_GetIntAttr(model, attrname, value)
    }
  }
  
  impl Attr for DoubleAttr {
    type Out = f64;
    type Buf = f64;
    type RawGet = *mut std::ffi::c_double;
    type RawSet = std::ffi::c_double;
  
    unsafe fn get_attr(model: *mut copt_sys::copt_prob, attrname: copt_sys::c_str, value: *mut std::ffi::c_double) -> std::ffi::c_int {
        copt_sys::COPT_GetDblAttr(model, attrname, value)
    }
  }