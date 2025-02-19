#![feature(let_chains)]
#![feature(impl_trait_in_assoc_type)]

mod easings;
mod ffi;
#[cfg(not(target_os = "android"))]
mod graphing;
mod modifiers;
mod point_data;
mod point_definition;
mod values;

use cfg_if::cfg_if;
use ctor::ctor;
use std::{backtrace::Backtrace, panic::PanicHookInfo};
use tracing::{error, info};
use tracing_error::SpanTrace;


fn main() {
    //graphing::graph("2d");
    //graphing::graph("3d");
    graphing::graph("color");
    //graphing::graph("quat");
}
