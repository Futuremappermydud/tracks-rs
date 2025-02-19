#![feature(let_chains)]

mod easings;
mod ffi;
#[cfg(not(target_os = "android"))]
mod graphing;
mod modifiers;
mod point_data;
mod point_definition;
mod values;

use tracing::error;
use tracing_error::SpanTrace;
use std::{backtrace::Backtrace, panic::PanicHookInfo};
use cfg_if::cfg_if;


#[cfg(target_os = "windows")]
fn main() {
    //graphing::graph("2d");
    //graphing::graph("3d");
    graphing::graph("color");
    //graphing::graph("quat");
}

#[cfg(target_os = "android")]
fn main() {
  setup("tracks-rs");
}

#[allow(clippy::needless_pass_by_value)]
pub fn setup(tag: impl ToString) {
    cfg_if! {
        if #[cfg(target_os = "android")] {
            paranoid_android::init(tag);
        } else {
            let env = format!("LOG_{}", tag.to_string().to_ascii_uppercase());
            let filter = tracing_subscriber::filter::EnvFilter::from_env(env);
            tracing_subscriber::fmt().with_env_filter(filter).init();
        }
    }
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