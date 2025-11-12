#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]

pub use std::os::raw::{c_int, c_double, c_char, c_void,c_longlong};
pub type c_str = *const c_char;

use std::ffi::CString;
use std::convert::From;

#[repr(C)]
pub struct copt_env_config;

#[repr(C)]
pub struct copt_env;

#[repr(C)]
pub struct copt_prob;


#[derive(Debug,Copy,Clone)]
pub enum IntParam{
  Logging,
  LogToConsole,
  Presolve,
  Scaling,
  Dualize,
  LpMethod,
  GPUMode,
  GPUDevice,
  ReqFarkasRay,
  DualPrice,
  DualPerturb,
  CutLevel,
  RootCutLevel,
  NodeCutRounds,
  HeurLevel,
  RoundingHeurLevel,
  DivingHeurLevel,
  FAPHeurLevel,
  SubMipHeurLevel,
  StrongBranching,
  ConflictAnalysis,
  NodeLimit,
  MipTasks,
  BarHomogeneous,
  BarOrder,
  BarStart,
  BarIterLimit,
  Threads,
  BarThreads,
  SimplexThreads,
  CrossoverThreads,
  Crossover,
  SDPMethod,
  IISMethod,
  FeasRelaxMode,
  MipStartMode,
  MipStartNodeLimit,
  TuneMethod,
  TuneMode,
  TuneMeasure,
  TunePermutes,
  TuneOutputLevel,
  LazyConstraints
}

#[derive(Debug,Copy,Clone)]
pub enum DoubleParam {
  TimeLimit,
  SolTimeLimit,
  MatrixTol,
  FeasTol,
  DualTol,
  IntTol,
  PDLPTol,
  RelGap,
  AbsGap,
  TuneTimeLimit,
  TuneTargetTime,
  TuneTargetRelGap
}

#[derive(Debug,Copy,Clone)]
pub enum IntAttr {
  Cols,
  PSDCols,
  Rows,
  Elems,
  QElems,
  PSDElems,
  SymMats,
  Bins,
  Ints,
  Soss,
  Cones,
  ExpCones,
  QConstrs,
  PSDConstrs,
  LMIConstrs,
  Indicators,
  IISCols,
  IISRows,
  IISSOSs,
  IISIndicators,
  ObjSense,
  LpStatus,
  MipStatus,
  SimplexIter,
  BarrierIter,
  NodeCnt,
  PoolSols,
  TuneResults,
  HasLpSol,
  HasDualFarkas,
  HasPrimalRay,
  HasBasis,
  HasMipSol,
  HasQObj,
  HasPSDObj,
  HasIIS,
  HasFeasRelaxSol,
  IsMIP,
  IsMinIIS
}

#[derive(Debug,Copy,Clone)]
pub enum DoubleAttr {
  SolvingTime,
  ObjConst,
  LpObjval,
  BestObj,
  BestBnd,
  BestGap,
  FeasRelaxObj
}

macro_rules! impl_from {
  ($($t:ty)*) => ($(
    impl From<$t> for CString {
      fn from(attr: $t) -> CString {
        CString::new(format!("{:?}", attr).as_str()).unwrap()
      }
    }
  )*)
}

impl_from! { IntParam DoubleParam }
impl_from! { IntAttr DoubleAttr }

extern "C" {

  pub fn COPT_GetBanner(buff: *mut c_char, buffSize: c_int) -> c_int;
  pub fn COPT_GetRetcodeMsg(code: c_int, buff: *mut c_char, buffSize: c_int) -> c_int;

  pub fn COPT_CreateEnvConfig(p_config: *mut *mut copt_env_config) -> c_int;
  pub fn COPT_DeleteEnvConfig(config: *mut *mut copt_env_config) -> c_int;
  pub fn COPT_SetEnvConfig(config: copt_env_config, name: *const c_char, value: *const c_char) -> c_int;

  pub fn COPT_CreateEnv(p_env: *mut *mut copt_env) -> c_int;
  pub fn COPT_CreateEnvWithPath(licDir: *const c_char, p_env: *mut *mut copt_env) -> c_int;
  pub fn COPT_SetEnvConfigPtr(env: copt_env, config: copt_env_config) -> c_int;
  pub fn COPT_CloseEnv(envP: *mut *mut copt_env) -> c_int;
  pub fn COPT_DeleteEnv(envP: *mut *mut copt_env) -> c_int;
  pub fn COPT_GetLicenseMsg(env: *mut copt_env, buff: *mut c_char, buffSize: c_int) -> c_int;

  pub fn COPT_CreateProb(env: *mut copt_env, p_prob: *mut *mut copt_prob) -> c_int;
  pub fn COPT_CreateCopy(src_prob: *mut copt_prob, p_dst_prob: *mut *mut copt_prob) -> c_int;
  pub fn COPT_ClearProb(prob: *mut copt_prob) -> c_int;
  pub fn COPT_DeleteProb(probP: *mut *mut copt_prob) -> c_int;

  pub fn COPT_LoadProb(prob: copt_prob, nCol: c_int, nRow: c_int, iObjSense: c_int, dObjConst: c_double, colObj: *const c_double, colMatBeg: *const c_int, colMatCnt: *const c_int, colMatIdx: *const c_int, colMatElem: *const c_double, colType: *const c_char, colLower: *const c_double, colUpper: *const c_double, rowSense: *const c_char, rowBound: *const c_double, rowUpper: *const c_double, colNames: *const *const c_char, rowNames: *const *const c_char) -> c_int;
  pub fn COPT_AddCol(prob: *mut copt_prob, dColObj:c_double,nColMatCnt: c_int, colMatIdx: *const c_int, colMatElem: *const c_double, cColType: c_char, dColLower: c_double, dColUpper: c_double, colName: *const c_char) -> c_int;
  pub fn COPT_AddPSDCol(prob: *mut copt_prob, colDim: c_int, name: *const c_char) -> c_int;
  pub fn COPT_AddRow(prob: *mut copt_prob, nRowMatCnt: c_int, rowMatIdx: *const c_int, rowMatElem: *const c_double, cRowSense: c_char, dRowBound: c_double, dRowUpper: c_double, rowName : *const c_char) -> c_int;
  pub fn COPT_AddCols(prob: *mut copt_prob, nAddCol: c_int, colObj: *const c_double, colMatBeg: *const c_int, colMatCnt: *const c_int, colMatIdx: *const c_int, colMatElem: *const c_double, colType: *const c_char, colLower: *const c_double, colUpper: *const c_double, rowName : *const *const c_char) -> c_int;
  pub fn COPT_AddPSDCols(prob: *mut copt_prob, nAddCol: c_int, colDims: *const c_int, names : *const *const c_char) -> c_int;
  pub fn COPT_AddRows(prob: *mut copt_prob, nAddRow: c_int, rowMatBeg: *const c_int, rowMatCnt: *const c_int, rowMatIdx: *const c_int, rowMatElem: *const c_double, rowSense : c_char, rowBound: c_double, rowUpper: c_double, rowNames : *const *const c_char) -> c_int;
  // ...
  pub fn COPT_GetCols(prob: *mut copt_prob, nCol: c_int, list: *const c_int, colMatBeg: *mut c_int, colMatCnt: *mut c_int, colMatIdx: *mut c_int, colMatElem: *mut c_double, nElemSize: c_int, pReqSize: *mut c_int) -> c_int;
  pub fn COPT_GetPSDCols(prob: *mut copt_prob, nCol: c_int, list: *mut c_int, colDims: *mut c_int, colLens: *mut c_int) -> c_int;
  pub fn COPT_GetRows(prob: *mut copt_prob, nRow: c_int, list: *const c_int, rowMatBeg: *mut c_int, rowMatCnt: *mut c_int, rowMatIdx: *mut c_int, rowMatElem: *mut c_double, nElemSize: c_int, pReqSize: *mut c_int) -> c_int;
  // ...
  pub fn COPT_GetElem(prob: *mut copt_prob, iCol: c_int, iRow: c_int, p_elem: *mut c_double) -> c_int;
  pub fn COPT_SetElem(prob: *mut copt_prob, iCol: c_int, iRow: c_int, newElem: c_double) -> c_int;

  pub fn COPT_GetPSDElem(prob: *mut copt_prob, iCol: c_int, iRow: c_int, p_idx: *mut c_int) -> c_int;
  pub fn COPT_SetPSDElem(prob: *mut copt_prob, iCol: c_int, iRow: c_int, newIdx: c_int) -> c_int;

  pub fn COPT_GetLMIElem(prob: *mut copt_prob, iCol: c_int, iRow: c_int, p_idx: *mut c_int) -> c_int;
  pub fn COPT_SetLMIElem(prob: *mut copt_prob, iCol: c_int, iRow: c_int, newIdx: c_int) -> c_int;
  
  pub fn COPT_DelCols(prob: *mut copt_prob, num: c_int, list: *const c_int) -> c_int;
  pub fn COPT_DelPSDCols(prob: *mut copt_prob, num: c_int, list: *const c_int) -> c_int;
  pub fn COPT_DelRows(prob: *mut copt_prob, num: c_int, list: *const c_int) -> c_int;
  pub fn COPT_DelSOSs(prob: *mut copt_prob, num: c_int, list: *const c_int) -> c_int;
  pub fn COPT_DelCones(prob: *mut copt_prob, num: c_int, list: *const c_int) -> c_int;
  pub fn COPT_DelExpCones(prob: *mut copt_prob, num: c_int, list: *const c_int) -> c_int;
  pub fn COPT_DelQConstrs(prob: *mut copt_prob, num: c_int, list: *const c_int) -> c_int;
  pub fn COPT_DelPSDConstrs(prob: *mut copt_prob, num: c_int, list: *const c_int) -> c_int;
  pub fn COPT_DelLMIConstrs(prob: *mut copt_prob, num: c_int, list: *const c_int) -> c_int;
  pub fn COPT_DelIndicators(prob: *mut copt_prob, num: c_int, list: *const c_int) -> c_int;
  //...
  pub fn COPT_SetObjSense(prob: *mut copt_prob, iObjSense: c_int) -> c_int;
  pub fn COPT_SetObjConst(prob: *mut copt_prob, dObjConst: c_double) -> c_int;

  pub fn COPT_SetColObj(prob: *mut copt_prob, num: c_int, list: *const c_int, obj: *const c_double) -> c_int;
  pub fn COPT_SetColType(prob: *mut copt_prob, num: c_int, list: *const c_int, types: *const c_char) -> c_int;
  pub fn COPT_SetColLower(prob: *mut copt_prob, num: c_int, list: *const c_int, lower: *const c_double) -> c_int;
  pub fn COPT_SetColUpper(prob: *mut copt_prob, num: c_int, list: *const c_int, upper: *const c_double) -> c_int;
  pub fn COPT_SetColNames(prob: *mut copt_prob, num: c_int, list: *const c_int, names: *const *const c_char) -> c_int;

  pub fn COPT_SetPSDColNames(prob: *mut copt_prob, num: c_int, list: *const c_int, names: *const *const c_char) -> c_int;

  pub fn COPT_SetRowLower(prob: *mut copt_prob, num: c_int, list: *const c_int, lower: *const c_double) -> c_int;
  pub fn COPT_SetRowUpper(prob: *mut copt_prob, num: c_int, list: *const c_int, upper: *const c_double) -> c_int;
  pub fn COPT_SetRowNames(prob: *mut copt_prob, num: c_int, list: *const c_int, names: *const *const c_char) -> c_int;
  //...
  pub fn COPT_ReadMps(prob: *mut copt_prob, mpsfilename: *const c_char) -> c_int;
  pub fn COPT_ReadLp(prob: *mut copt_prob, lpfilename: *const c_char) -> c_int;
  pub fn COPT_ReadSDPA(prob: *mut copt_prob, sdpafilename: *const c_char) -> c_int;
  pub fn COPT_ReadCbf(prob: *mut copt_prob, cbffilename: *const c_char) -> c_int;
  pub fn COPT_ReadBin(prob: *mut copt_prob, binfilename: *const c_char) -> c_int;
  pub fn COPT_ReadSol(prob: *mut copt_prob, solfilename: *const c_char) -> c_int;
  pub fn COPT_ReadBasis(prob: *mut copt_prob, basfilename: *const c_char) -> c_int;
  pub fn COPT_ReadMst(prob: *mut copt_prob, mstfilename: *const c_char) -> c_int;
  pub fn COPT_ReadParam(prob: *mut copt_prob, parfilename: *const c_char) -> c_int;
  pub fn COPT_ReadParamStr(prob: *mut copt_prob, strparam: *const c_char) -> c_int;
  pub fn COPT_ReadTune(prob: *mut copt_prob, tunefilename: *const c_char) -> c_int;
  pub fn COPT_ReadBlob(prob: *mut copt_prob, blob: *mut c_void, len: c_longlong) -> c_int;

  pub fn COPT_WriteMps(prob: *mut copt_prob, mpsfilename: *const c_char) -> c_int;
  pub fn COPT_WriteLp(prob: *mut copt_prob, lpfilename: *const c_char) -> c_int;
  pub fn COPT_WriteCbf(prob: *mut copt_prob, cbffilename: *const c_char) -> c_int;
  pub fn COPT_WriteBin(prob: *mut copt_prob, binfilename: *const c_char) -> c_int;
  pub fn COPT_WriteIIS(prob: *mut copt_prob, iisfilename: *const c_char) -> c_int;
  pub fn COPT_WriteRelax(prob: *mut copt_prob, relaxfilename: *const c_char) -> c_int;
  pub fn COPT_WriteSol(prob: *mut copt_prob, solfilename: *const c_char) -> c_int;
  pub fn COPT_WritePoolSol(prob: *mut copt_prob, solfilename: *const c_char) -> c_int;
  pub fn COPT_WriteBasis(prob: *mut copt_prob, basfilename: *const c_char) -> c_int;
  pub fn COPT_WriteMst(prob: *mut copt_prob, mstfilename: *const c_char) -> c_int;
  pub fn COPT_WriteParam(prob: *mut copt_prob, parfilename: *const c_char) -> c_int;
  pub fn COPT_WriteTuneParam(prob: *mut copt_prob, parfilename: *const c_char) -> c_int;
  pub fn COPT_WriteMpsStr(prob: *mut copt_prob, str: *const c_char, nStrSize: c_int, pReqSize: *mut c_int) -> c_int;
  pub fn COPT_WriteParamStr(prob: *mut copt_prob, str: *const c_char, nStrSize: c_int, pReqSize: *mut c_int) -> c_int;
  pub fn COPT_WriteBlob(prob: *mut copt_prob, tryCompress: c_int, p_blob: *mut *mut c_void, pLen: *mut c_longlong) -> c_int;

  pub fn COPT_AddMipStart(prob: *mut copt_prob, num: c_int, list: *const c_int, colVal: *const c_double) -> c_int;

  pub fn COPT_SolveLp(prob: *mut copt_prob) -> c_int;
  pub fn COPT_Solve(prob: *mut copt_prob) -> c_int;
  //...
  pub fn COPT_GetSolution(prob: *mut copt_prob, colVal: *mut c_double) -> c_int;
  pub fn COPT_GetLpSolution(prob: *mut copt_prob, value: *mut c_double, slack: *mut c_double, rowDual: *mut c_double, redCost: *mut c_double) -> c_int;
  pub fn COPT_SetLpSolution(prob: *mut copt_prob, value: *const c_double, slack: *const c_double, rowDual: *const c_double, redCost: *const c_double) -> c_int;
  pub fn COPT_GetBasis(prob: *mut copt_prob, colBasis: *mut c_int, rowBasis: *mut c_int) -> c_int;
  pub fn COPT_SetBasis(prob: *mut copt_prob, colBasis: *const c_int, rowBasis: *const c_int) -> c_int;
  pub fn COPT_SetSlackBasis(prob: *mut copt_prob) -> c_int;

  pub fn COPT_GetPoolObjVal(prob: *mut copt_prob, iSol: c_int, p_objVal: *mut c_double) -> c_int;
  pub fn COPT_GetPoolSolution(prob: *mut copt_prob, iSol: c_int, num: c_int, list: *const c_int, colVal: *mut c_double) -> c_int;

  pub fn COPT_SetIntParam(prob: *mut copt_prob, paramName: *const c_char, intParam: c_int) -> c_int;
  pub fn COPT_GetIntParam(prob: *mut copt_prob, paramName: *const c_char, p_intParam: *mut c_int) -> c_int;
  pub fn COPT_GetIntParamDef(prob: *mut copt_prob, paramName: *const c_char, p_intParam: *mut c_int) -> c_int;
  pub fn COPT_GetIntParamMin(prob: *mut copt_prob, paramName: *const c_char, p_intParam: *mut c_int) -> c_int;
  pub fn COPT_GetIntParamMax(prob: *mut copt_prob, paramName: *const c_char, p_intParam: *mut c_int) -> c_int;

  pub fn COPT_SetDblParam(prob: *mut copt_prob, paramName: *const c_char, dblParam: f64) -> c_int;
  pub fn COPT_GetDblParam(prob: *mut copt_prob, paramName: *const c_char, p_dblParam: *mut f64) -> c_int;
  pub fn COPT_GetDblParamDef(prob: *mut copt_prob, paramName: *const c_char, p_dblParam: *mut f64) -> c_int;
  pub fn COPT_GetDblParamMin(prob: *mut copt_prob, paramName: *const c_char, p_dblParam: *mut f64) -> c_int;
  pub fn COPT_GetDblParamMax(prob: *mut copt_prob, paramName: *const c_char, p_dblParam: *mut f64) -> c_int;

  pub fn COPT_ResetParam(prob: *mut copt_prob) -> c_int;
  pub fn COPT_Reset(prob: *mut copt_prob, iClearAll: c_int) -> c_int;
  
  pub fn COPT_GetIntAttr(prob: *mut copt_prob, attrName: *const c_char, p_intAttr: *mut c_int) -> c_int;
  pub fn COPT_GetDblAttr(prob: *mut copt_prob, attrName: *const c_char, p_dblAttr: *mut f64) -> c_int;
  //...
  pub fn COPT_SetLogFile(prob: *mut copt_prob, logfilename: *const c_char) -> c_int;
  pub fn COPT_SetLogCallback(prob: *mut copt_prob,logcb: Option<extern "C" fn(*const c_char, *mut c_void)>, userdata: *mut c_void) -> c_int;
  //...
  pub fn COPT_SetCallback(prob: *mut copt_prob,cb: Option<extern "C" fn(*mut copt_prob, *mut c_void, c_int, *mut c_void)->c_int>,cbctx: c_int, userdata: *mut c_void) -> c_int;
  //...
  pub fn COPT_Interrupt(prob: *mut copt_prob) -> c_int;
}
