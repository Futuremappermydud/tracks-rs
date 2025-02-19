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

#[cfg(target_os = "windows")]
fn main() {
    //graphing::graph("2d");
    //graphing::graph("3d");
    graphing::graph("color");
    //graphing::graph("quat");
}

#[cfg(target_os = "android")]
#[ctor]
fn main() {
    setup("tracks-rs");

    info!("setup HI");
}

#[allow(clippy::needless_pass_by_value)]
pub fn setup(tag: impl ToString) {
    paranoid_android::init(tag);
    std::panic::set_hook(panic_hook(true, true));
}

/// Returns a panic handler, optionally with backtrace and spantrace capture.
pub fn panic_hook(
    backtrace: bool,
    spantrace: bool,
) -> Box<dyn Fn(&PanicHookInfo) + Send + Sync + 'static> {
    // Mostly taken from https://doc.rust-lang.org/src/std/panicking.rs.html
    Box::new(move |info| {
        let location = info.location().unwrap();
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<dyn Any>",
            },
        };

        error!(target: "panic", "panicked at '{}', {}", msg, location);
        if backtrace {
            error!(target: "panic", "{:?}", Backtrace::force_capture());
        }
        if spantrace {
            error!(target: "panic", "{:?}", SpanTrace::capture());
        }
    })
}
