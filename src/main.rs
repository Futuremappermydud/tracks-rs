#![feature(let_chains)]

mod easings;
mod ffi;
#[cfg(not(target_os = "android"))]
mod graphing;
mod modifiers;
mod point_data;
mod point_definition;
mod values;

#[cfg(target_os = "windows")]
fn main() {
    //graphing::graph("2d");
    //graphing::graph("3d");
    //graphing::graph("color");
    graphing::graph("quat");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("Plot feature is not enabled");
}
