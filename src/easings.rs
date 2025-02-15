use std::default;

use functions::Functions;
use implementations::*;

pub mod functions;
mod implementations;

pub fn interpolate(t: f32, easing: Functions) -> f32 {
  println!("Interpolating {}", t);
  match easing {
    Functions::EaseLinear => {
      println!("Using EaseLinear");
      ease_linear(t)
    },
    Functions::EaseStep => {
      println!("Using EaseStep");
      ease_step(t)
    },
    Functions::EaseInQuad => {
      println!("Using EaseInQuad");
      ease_in_quad(t)
    },
    Functions::EaseOutQuad => {
      println!("Using EaseOutQuad");
      ease_out_quad(t)
    },
    Functions::EaseInOutQuad => {
      println!("Using EaseInOutQuad");
      ease_in_out_quad(t)
    },
    Functions::EaseInCubic => {
      println!("Using EaseInCubic");
      ease_in_cubic(t)
    },
    Functions::EaseOutCubic => {
      println!("Using EaseOutCubic");
      ease_out_cubic(t)
    },
    Functions::EaseInOutCubic => {
      println!("Using EaseInOutCubic");
      ease_in_out_cubic(t)
    },
    Functions::EaseInQuart => {
      println!("Using EaseInQuart");
      ease_in_quart(t)
    },
    Functions::EaseOutQuart => {
      println!("Using EaseOutQuart");
      ease_out_quart(t)
    },
    Functions::EaseInOutQuart => {
      println!("Using EaseInOutQuart");
      ease_in_out_quart(t)
    },
    Functions::EaseInQuint => {
      println!("Using EaseInQuint");
      ease_in_quint(t)
    },
    Functions::EaseOutQuint => {
      println!("Using EaseOutQuint");
      ease_out_quint(t)
    },
    Functions::EaseInOutQuint => {
      println!("Using EaseInOutQuint");
      ease_in_out_quint(t)
    },
    Functions::EaseInSine => {
      println!("Using EaseInSine");
      ease_in_sine(t)
    },
    Functions::EaseOutSine => {
      println!("Using EaseOutSine");
      ease_out_sine(t)
    },
    Functions::EaseInOutSine => {
      println!("Using EaseInOutSine");
      ease_in_out_sine(t)
    },
    Functions::EaseInCirc => {
      println!("Using EaseInCirc");
      ease_in_circ(t)
    },
    Functions::EaseOutCirc => {
      println!("Using EaseOutCirc");
      ease_out_circ(t)
    },
    Functions::EaseInOutCirc => {
      println!("Using EaseInOutCirc");
      ease_in_out_circ(t)
    },
    Functions::EaseInExpo => {
      println!("Using EaseInExpo");
      ease_in_expo(t)
    },
    Functions::EaseOutExpo => {
      println!("Using EaseOutExpo");
      ease_out_expo(t)
    },
    Functions::EaseInOutExpo => {
      println!("Using EaseInOutExpo");
      ease_in_out_expo(t)
    },
    Functions::EaseInElastic => {
      println!("Using EaseInElastic");
      ease_in_elastic(t)
    },
    Functions::EaseOutElastic => {
      println!("Using EaseOutElastic");
      ease_out_elastic(t)
    },
    Functions::EaseInOutElastic => {
      println!("Using EaseInOutElastic");
      ease_in_out_elastic(t)
    },
    Functions::EaseInBack => {
      println!("Using EaseInBack");
      ease_in_back(t)
    },
    Functions::EaseOutBack => {
      println!("Using EaseOutBack");
      ease_out_back(t)
    },
    Functions::EaseInOutBack => {
      println!("Using EaseInOutBack");
      ease_in_out_back(t)
    },
    Functions::EaseInBounce => {
      println!("Using EaseInBounce");
      ease_in_bounce(t)
    },
    Functions::EaseOutBounce => {
      println!("Using EaseOutBounce");
      ease_out_bounce(t)
    },
    Functions::EaseInOutBounce => {
      println!("Using EaseInOutBounce");
      ease_in_out_bounce(t)
    },
  }
}