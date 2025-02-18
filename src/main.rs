#![feature(let_chains)]

mod easings;
mod ffi;
mod graphing;
mod modifiers;
mod point_data;
mod point_definition;
mod values;

#[cfg(target_os = "windows")]
fn main() {
    graphing::graph("3d");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("Plot feature is not enabled");
}
