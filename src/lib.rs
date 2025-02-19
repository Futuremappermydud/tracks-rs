#![feature(let_chains)]

pub mod easings;
pub mod ffi;
#[cfg(not(target_os = "android"))]
pub mod graphing;
pub mod modifiers;
pub mod point_data;
pub mod point_definition;
pub mod values;
