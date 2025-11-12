use std::panic::{AssertUnwindSafe, catch_unwind};
use std::{ffi::CString, mem::transmute, ptr::null_mut};

use copt_sys::COPT_Interrupt;

use crate::{
    env::{Env, EnvAPI},
    util,
};

#[derive(Debug, Clone, Copy)]
pub enum VarType {
    Binary,
    Continuous,
    Integer,
}

impl Into<std::ffi::c_char> for VarType {
    fn into(self) -> std::ffi::c_char {
        match self {
            VarType::Binary => 'B' as std::ffi::c_char,
            VarType::Continuous => 'C' as std::ffi::c_char,
            VarType::Integer => 'I' as std::ffi::c_char,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ConstrSense {
    Equal,
    Greater,
    Less,
}

impl Into<std::ffi::c_char> for ConstrSense {
    fn into(self) -> std::ffi::c_char {
        match self {
            ConstrSense::Equal => 'E' as std::ffi::c_char,
            ConstrSense::Less => 'L' as std::ffi::c_char,
            ConstrSense::Greater => 'G' as std::ffi::c_char,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ModelSense {
    Minimize = 1,
    Maximize = -1,
}

impl Into<i32> for ModelSense {
    fn into(self) -> i32 {
        (unsafe { std::mem::transmute::<_, i8>(self) }) as i32
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Unstarted = 0,
    Optimal,
    Infeasible,
    Unbounded,
    Numerical,
    InfOrUnb,
    NodeLimit,
    Imprecise,
    Timeout,
    Unfinished,
    Interrupted,
}

impl From<i32> for Status {
    fn from(val: i32) -> Status {
        match val {
            0..=10 => unsafe { std::mem::transmute(val as i8) },
            _ => panic!("cannot convert to Status: {}", val),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Var(pub i32);

impl Into<i32> for Var {
    fn into(self) -> i32 {
        return self.0;
    }
}

//pub type Var = i32;
pub type Constr = i32;

struct LogCallbackData<'a> {
    callback: &'a mut dyn FnMut(crate::callback::LogCallbackParams),
}

#[allow(unused_variables)]
extern "C" fn callback_wrapper(msg: copt_sys::c_str, usrdata: *mut std::ffi::c_void) {
    let usrdata = unsafe { transmute::<_, &mut LogCallbackData>(usrdata) };
    //let (callback, model) = (&mut usrdata.callback, &usrdata.model);
    let callback = &mut usrdata.callback;
    match unsafe { std::ffi::CStr::from_ptr(msg) }.to_str() {
        Ok(msg_str) => {
            // let params=crate::callback::LogCallbackParams { msg: msg_str };
            // callback(params);
            let _ = catch_unwind(AssertUnwindSafe(move|| {
                callback(crate::callback::LogCallbackParams { msg: msg_str })
            }));
            //println!("{}",params.msg)
        }
        Err(_) => {}
    }
}

struct TerminateCallbackData<'a> {
    callback: &'a mut dyn FnMut()->bool,
}

#[allow(unused_variables)]
extern "C" fn terminate_callback_wrapper(prob: *mut copt_sys::copt_prob, cbdata: *mut std::ffi::c_void, cbctx: std::ffi::c_int,usrdata: *mut std::ffi::c_void)->std::ffi::c_int {
    let usrdata = unsafe { transmute::<_, &mut TerminateCallbackData>(usrdata) };
    //let (callback, model) = (&mut usrdata.callback, &usrdata.model);
    let callback = &mut usrdata.callback;
    if callback(){
        unsafe {COPT_Interrupt(prob)};
        return 10 as std::ffi::c_int;
    }else{
        return 0 as std::ffi::c_int;
    }
}

// #[allow(unused_variables)]
// extern "C" fn null_log_callback_wrapper(msg: copt_sys::c_str, usrdata: *mut std::ffi::c_void) {}

pub struct Model {
    model: *mut copt_sys::copt_prob,
    env: Env,
    var_count: i32,
    constr_count: i32,
}

impl Model {
    pub fn new(env: &Env) -> crate::error::Result<Model> {
        let mut model = null_mut();
        env.check_apicall(unsafe { copt_sys::COPT_CreateProb(env.get_ptr(), &mut model) })?;
        let env_copy = Env::shallow_copy(env);
        let model = Model { model, env: env_copy, var_count: 0, constr_count: 0 };
        Ok(model)
    }

    pub fn add_var(
        &mut self,
        name: &str,
        vtype: VarType,
        obj: f64,
        lb: f64,
        ub: f64,
        constrs: &[Constr],
        coeffs: &[f64],
    ) -> crate::error::Result<Var> {
        if constrs.len() != coeffs.len() {
            return Err(crate::error::Error::InconsitentDims);
        }
        let name_c = CString::new(name)?;
        self.check_apicall(unsafe {
            copt_sys::COPT_AddCol(
                self.model,
                obj,
                constrs.len() as std::ffi::c_int,
                constrs.as_ptr(),
                coeffs.as_ptr(),
                vtype.into(),
                lb,
                ub,
                name_c.as_ptr(),
            )
        })?;
        self.var_count += 1;
        Ok(Var(self.var_count - 1))
    }

    pub fn add_constr_low_level(
        &mut self,
        name: &str,
        vars: &[i32],
        coeffs: &[f64],
        sense: ConstrSense,
        rhs: f64,
    ) -> crate::error::Result<Constr> {
        if vars.len() != coeffs.len() {
            return Err(crate::error::Error::InconsitentDims);
        }
        let constrname = CString::new(name)?;
        self.check_apicall(unsafe {
            copt_sys::COPT_AddRow(
                self.model,
                vars.len() as std::ffi::c_int,
                vars.as_ptr(),
                coeffs.as_ptr(),
                sense.into(),
                rhs,
                0.0,
                constrname.as_ptr(),
            )
        })?;
        self.constr_count += 1;
        Ok(self.constr_count - 1)
    }

    pub fn add_constr(
        &mut self,
        name: &str,
        expr: crate::expr::LinExpr,
        sense: ConstrSense,
        rhs: f64,
    ) -> crate::error::Result<Constr> {
        let (vars, coeffs, offset) = expr.into();
        let constrname = CString::new(name)?;
        self.check_apicall(unsafe {
            copt_sys::COPT_AddRow(
                self.model,
                vars.len() as std::ffi::c_int,
                vars.as_ptr(),
                coeffs.as_ptr(),
                sense.into(),
                rhs - offset,
                0.0,
                constrname.as_ptr(),
            )
        })?;
        self.constr_count += 1;
        Ok(self.constr_count - 1)
    }

    pub fn set_objective_low_level(
        &mut self,
        vars: &[i32],
        coeffs: &[f64],
        sense: ModelSense,
    ) -> crate::error::Result<()> {
        if vars.len() != coeffs.len() {
            return Err(crate::error::Error::InconsitentDims);
        }
        self.check_apicall(unsafe {
            copt_sys::COPT_SetColObj(self.model, vars.len() as i32, vars.as_ptr(), coeffs.as_ptr())
        })?;
        self.check_apicall(unsafe { copt_sys::COPT_SetObjSense(self.model, sense.into()) })?;
        Ok(())
    }

    pub fn set_objective(
        &mut self,
        expr: crate::expr::LinExpr,
        sense: ModelSense,
    ) -> crate::error::Result<()> {
        let (vars, coeffs, offset) = expr.into();
        self.check_apicall(unsafe {
            copt_sys::COPT_SetColObj(self.model, vars.len() as i32, vars.as_ptr(), coeffs.as_ptr())
        })?;
        if offset != 0.0 {
            self.check_apicall(unsafe { copt_sys::COPT_SetObjConst(self.model, offset) })?;
        }
        self.check_apicall(unsafe { copt_sys::COPT_SetObjSense(self.model, sense.into()) })?;
        Ok(())
    }

    pub fn get_attribute<A: crate::attribute::Attr>(
        &self,
        attr: A,
    ) -> crate::error::Result<A::Out> {
        let mut value: A::Buf = util::Init::init();
        self.check_apicall(unsafe {
            use util::AsRawPtr;
            A::get_attr(self.model, attr.into().as_ptr(), value.as_rawptr())
        })?;
        Ok(util::Into::into(value))
    }

    pub fn get_param<A: crate::param::Param>(&self, param: A) -> crate::error::Result<A::Out> {
        let mut value: A::Buf = util::Init::init();
        self.check_apicall(unsafe {
            use util::AsRawPtr;
            A::get_param(self.model, param.into().as_ptr(), value.as_rawptr())
        })?;
        Ok(util::Into::into(value))
    }

    pub fn set_param<A: crate::param::Param>(
        &self,
        param: A,
        value: A::Out,
    ) -> crate::error::Result<()> {
        self.check_apicall(unsafe {
            A::set_param(self.model, param.into().as_ptr(), util::FromRaw::from(value))
        })?;
        Ok(())
    }

    pub fn add_mip_start(&mut self,start:&Vec<(i32, f64)>)-> crate::error::Result<()> {
        if start.is_empty() {
            return Ok(());
        }
        let mut vars=Vec::with_capacity(start.len());
        let mut values = Vec::with_capacity(start.len());
        for (var,value) in start{
            vars.push(*var);
            values.push(*value);
        }
        self.check_apicall(unsafe { copt_sys::COPT_AddMipStart(self.model,start.len() as i32,vars.as_ptr(),values.as_ptr()) })
    }

    pub fn optimize(&mut self) -> crate::error::Result<()> {
        self.check_apicall(unsafe { copt_sys::COPT_Solve(self.model) })
    }

    pub fn optimize_with_log_callback<F>(&mut self,mut callback: F) -> crate::error::Result<()>
    where F: FnMut(crate::callback::LogCallbackParams) + 'static {
        let usrdata = LogCallbackData { callback: &mut callback };
        self.check_apicall(unsafe {
            copt_sys::COPT_SetLogCallback(self.model, Some(callback_wrapper), transmute(&usrdata))
        })?;
        self.check_apicall(unsafe { copt_sys::COPT_Solve(self.model) })?;
        self.check_apicall(unsafe {
            copt_sys::COPT_SetLogCallback(self.model, None, null_mut())
        })
    }

    pub fn optimize_with_terminate_callback<G>(&mut self,mut terminate_callback: G) -> crate::error::Result<()>
    where G: FnMut() -> bool + 'static {
        let terminate_usrdata = TerminateCallbackData { callback: &mut terminate_callback };
        self.check_apicall(unsafe {
            copt_sys::COPT_SetCallback(self.model, Some(terminate_callback_wrapper), 4i32/*COPT_CBCONTEXT_MIPNODE */,transmute(&terminate_usrdata))
        })?;
        self.check_apicall(unsafe { copt_sys::COPT_Solve(self.model) })?;
        self.check_apicall(unsafe {
            copt_sys::COPT_SetCallback(self.model, None, 4i32/*COPT_CBCONTEXT_MIPNODE */,null_mut())
        })
    }

    pub fn optimize_with_log_callback_and_terminate_callback<F,G>(&mut self,mut callback: F,mut terminate_callback: G) -> crate::error::Result<()>
    where F: FnMut(crate::callback::LogCallbackParams) + 'static,
    G: FnMut() -> bool + 'static {
        let usrdata = LogCallbackData { callback: &mut callback };
        let terminate_usrdata = TerminateCallbackData { callback: &mut terminate_callback };
        self.check_apicall(unsafe {
            copt_sys::COPT_SetLogCallback(self.model, Some(callback_wrapper), transmute(&usrdata))
        })?;
        self.check_apicall(unsafe {
            copt_sys::COPT_SetCallback(self.model, Some(terminate_callback_wrapper), 4i32/*COPT_CBCONTEXT_MIPNODE */,transmute(&terminate_usrdata))
        })?;
        self.check_apicall(unsafe { copt_sys::COPT_Solve(self.model) })?;
        self.check_apicall(unsafe {
            copt_sys::COPT_SetLogCallback(self.model, None, null_mut())
        })?;
        self.check_apicall(unsafe {
            copt_sys::COPT_SetCallback(self.model, None, 4i32/*COPT_CBCONTEXT_MIPNODE */,null_mut())
        })
    }

    pub fn get_results(&self) -> crate::error::Result<Vec<f64>> {
        let mut res = vec![0.0; self.var_count as usize];
        self.check_apicall(unsafe { copt_sys::COPT_GetSolution(self.model, res.as_mut_ptr()) })?;
        Ok(res)
    }

    pub fn get_lp_results(&self) -> crate::error::Result<Vec<f64>> {
        let mut res = vec![0.0; self.var_count as usize];
        self.check_apicall(unsafe { copt_sys::COPT_GetLpSolution(self.model, res.as_mut_ptr(), null_mut(),null_mut(),null_mut()) })?;
        Ok(res)
    }

    pub fn terminate(&self) {
        unsafe { copt_sys::COPT_Interrupt(self.model) };
    }

    pub fn read(&mut self, filename: &str) -> crate::error::Result<()> {
        let low_case = filename.to_ascii_lowercase();
        let filename_c = CString::new(filename)?;
        if low_case.ends_with(".mps") {
            self.check_apicall(unsafe { copt_sys::COPT_ReadMps(self.model, filename_c.as_ptr()) })
        } else if low_case.ends_with(".lp") {
            self.check_apicall(unsafe { copt_sys::COPT_ReadLp(self.model, filename_c.as_ptr()) })
        } else {
            use crate::env::ErrorFromAPI;
            Err(self.env.error_from_api(2))
        }
    }

    pub fn write(&self, filename: &str) -> crate::error::Result<()> {
        let low_case = filename.to_ascii_lowercase();
        let filename_c = CString::new(filename)?;
        if low_case.ends_with(".mps") {
            self.check_apicall(unsafe { copt_sys::COPT_WriteMps(self.model, filename_c.as_ptr()) })
        } else if low_case.ends_with(".lp") {
            self.check_apicall(unsafe { copt_sys::COPT_WriteLp(self.model, filename_c.as_ptr()) })
        } else {
            use crate::env::ErrorFromAPI;
            Err(self.env.error_from_api(2))
        }
    }

    pub fn set_log_file(&mut self, filename: &str) -> crate::error::Result<()> {
        let filename_c = CString::new(filename)?;
        self.check_apicall(unsafe { copt_sys::COPT_SetLogFile(self.model, filename_c.as_ptr()) })
    }

    // pub fn unset_log_callback(&mut self) -> crate::error::Result<()> {
    //     self.check_apicall(unsafe {
    //         copt_sys::COPT_SetLogCallback(self.model, null_log_callback_wrapper, null_mut())
    //     })
    // }

    // pub fn set_log_callback<F>(&mut self, callback: &'static mut F) -> crate::error::Result<()>
    // where
    //     F: FnMut(crate::callback::LogCallbackParams) + 'static,
    // {
    //     //callback(crate::callback::LogCallbackParams{msg:"set_log_callback"});
    //     let usrdata = LogCallbackData { callback: callback };
    //     self.check_apicall(unsafe {
    //         copt_sys::COPT_SetLogCallback(self.model, callback_wrapper, transmute(&usrdata))
    //     })
    // }

    fn check_apicall(&self, error: std::ffi::c_int) -> crate::error::Result<()> {
        if error != 0 {
            use crate::env::ErrorFromAPI;
            return Err(self.env.error_from_api(error));
        }
        Ok(())
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe { copt_sys::COPT_DeleteProb(&mut self.model) };
        self.model = null_mut();
    }
}
