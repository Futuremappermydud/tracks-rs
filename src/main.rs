#![feature(let_chains)]

mod easings;
mod ffi;
#[cfg(not(target_os = "android"))]
mod graphing;
mod modifiers;
mod point_data;
mod point_definition;
mod values;

#[cfg(target_os = "android")]
#[macro_use] extern crate log;
#[cfg(target_os = "android")]
extern crate android_logger;
#[cfg(target_os = "android")]
use log::LevelFilter;
#[cfg(target_os = "android")]
use android_logger::{Config,FilterBuilder};

#[cfg(target_os = "windows")]
fn main() {
    //graphing::graph("2d");
    //graphing::graph("3d");
    //graphing::graph("color");
    graphing::graph("quat");
}

#[cfg(target_os = "android")]
fn main() {
  android_logger::init_once(
    Config::default()
        .with_max_level(LevelFilter::Trace)
);
}
