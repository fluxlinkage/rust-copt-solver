use std::ptr::null_mut;

use crate::error::{Error, Result};

pub struct Env {
    env: *mut copt_sys::copt_env,
    require_drop: bool,
}

impl Env {
    pub fn get_banner() -> Result<String> {
        let mut buff= [0i8; 1024];
        let error = unsafe { copt_sys::COPT_GetBanner(&mut buff[0], buff.len() as i32) };
        if error != 0 {
            return Err(Error::FromAPI(error));
        }
        let raw:Vec<u8>=buff.iter().map(|&x|x as u8).collect();
        Ok(String::from_utf8_lossy(&raw).to_string())
    }

    pub fn new() -> Result<Env> {
        let mut env = null_mut();
        let error = unsafe { copt_sys::COPT_CreateEnv(&mut env) };
        if error != 0 {
            return Err(Error::FromAPI(error));
        }
        Ok(Env { env, require_drop: true })
    }

    pub fn shallow_copy(other: &Env) -> Env {
        Env { env: other.env, require_drop: false }
    }
}

pub trait EnvAPI {
    fn get_ptr(&self) -> *mut copt_sys::copt_env;
    fn check_apicall(&self, error: std::ffi::c_int) -> Result<()>;
}

impl EnvAPI for Env {
    fn get_ptr(&self) -> *mut copt_sys::copt_env {
        self.env
    }

    fn check_apicall(&self, error: std::ffi::c_int) -> Result<()> {
        if error != 0 {
            return Err(self.error_from_api(error));
        }
        Ok(())
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        if self.require_drop {
            unsafe { copt_sys::COPT_DeleteEnv(&mut self.env) };
            self.env = null_mut();
        }
    }
}

pub trait ErrorFromAPI {
    fn error_from_api(&self, error: std::ffi::c_int) -> Error;
}

impl ErrorFromAPI for Env {
    fn error_from_api(&self, error: std::ffi::c_int) -> Error {
        Error::FromAPI(error)
    }
}

pub trait FromRaw {
    fn from_raw(env: *mut copt_sys::copt_env) -> Self;
}

impl FromRaw for Env {
    fn from_raw(env: *mut copt_sys::copt_env) -> Env {
        Env { env, require_drop: false }
    }
}
