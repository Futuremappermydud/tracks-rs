#![feature(let_chains)]

#[cfg(target_os = "windows")]
use minifb::{Key, Window, WindowOptions};
#[cfg(target_os = "windows")]
use plotters::{
    backend::BGRXPixel,
    chart::ChartBuilder,
    prelude::*,
    series::LineSeries,
    style::{BLACK, Color, GREEN, IntoFont, TRANSPARENT},
};
use point_definition::{PointDefinition, float_point_definition::FloatPointDefinition};
use serde_json::json;
use std::{
    borrow::{Borrow, BorrowMut},
    error::Error,
    time::SystemTime,
};
use values::base_provider_context::BaseProviderContext;
mod easings;
mod ffi;
mod graphing;
mod modifiers;
mod point_data;
mod point_definition;
mod values;

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn Error>> {
    let testing = json!([["baseCombo", 0.0], [5.0, 1.0, "easeInOutSine"],]);

    let context = BaseProviderContext::new();

    let float_point_definition = FloatPointDefinition::new(&testing, &context);

    graphing::graph("2d");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("Plot feature is not enabled");
}
