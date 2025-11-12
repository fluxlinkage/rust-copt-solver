#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

mod util;
mod error;
mod param;
mod attribute;
mod expr;
mod env;
mod model;
mod callback;

pub use copt_sys::{IntAttr,DoubleAttr,IntParam,DoubleParam};
pub use error::{Error, Result};
pub use env::Env;
pub use expr::LinExpr;
pub use model::Model;
pub use model::Var;
pub use model::Status::*;
pub use model::VarType::*;
pub use model::ConstrSense::*;
pub use model::ModelSense::*;
pub use callback::LogCallbackParams;

pub const INFINITY: f64 = 1e30;