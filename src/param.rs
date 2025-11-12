use crate::util;
use copt_sys::{DoubleParam, IntParam};
use std::ffi::CString;

pub trait Param: Sized + Into<CString> {
    type Out;
    type Buf: util::Init + util::Into<Self::Out> + util::AsRawPtr<Self::RawFrom>;
    type RawFrom;
    type RawTo: util::FromRaw<Self::Out>;

    unsafe fn get_param(
        model: *mut copt_sys::copt_prob,
        paramname: copt_sys::c_str,
        value: Self::RawFrom,
    ) -> std::ffi::c_int;

    unsafe fn set_param(
        model: *mut copt_sys::copt_prob,
        paramname: copt_sys::c_str,
        value: Self::RawTo,
    ) -> std::ffi::c_int;
}

impl Param for IntParam {
    type Out = i32;
    type Buf = std::ffi::c_int;
    type RawFrom = *mut std::ffi::c_int;
    type RawTo = std::ffi::c_int;

    unsafe fn get_param(
        model: *mut copt_sys::copt_prob,
        paramname: copt_sys::c_str,
        value: *mut std::ffi::c_int,
    ) -> std::ffi::c_int {
        copt_sys::COPT_GetIntParam(model, paramname, value)
    }

    unsafe fn set_param(
        model: *mut copt_sys::copt_prob,
        paramname: copt_sys::c_str,
        value: std::ffi::c_int,
    ) -> std::ffi::c_int {
        copt_sys::COPT_SetIntParam(model, paramname, value)
    }
}

impl Param for DoubleParam {
    type Out = f64;
    type Buf = std::ffi::c_double;
    type RawFrom = *mut std::ffi::c_double;
    type RawTo = std::ffi::c_double;

    unsafe fn get_param(
        model: *mut copt_sys::copt_prob,
        paramname: copt_sys::c_str,
        value: *mut std::ffi::c_double,
    ) -> std::ffi::c_int {
        copt_sys::COPT_GetDblParam(model, paramname, value)
    }

    unsafe fn set_param(
        model: *mut copt_sys::copt_prob,
        paramname: copt_sys::c_str,
        value: std::ffi::c_double,
    ) -> std::ffi::c_int {
        copt_sys::COPT_SetDblParam(model, paramname, value)
    }
}
