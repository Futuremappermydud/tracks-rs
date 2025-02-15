use super::implementations::*;

#[allow(dead_code)]
pub enum Functions {
  EaseLinear,
  EaseStep,
  EaseInQuad,
  EaseOutQuad,
  EaseInOutQuad,
  EaseInCubic,
  EaseOutCubic,
  EaseInOutCubic,
  EaseInQuart,
  EaseOutQuart,
  EaseInOutQuart,
  EaseInQuint,
  EaseOutQuint,
  EaseInOutQuint,
  EaseInSine,
  EaseOutSine,
  EaseInOutSine,
  EaseInCirc,
  EaseOutCirc,
  EaseInOutCirc,
  EaseInExpo,
  EaseOutExpo,
  EaseInOutExpo,
  EaseInElastic,
  EaseOutElastic,
  EaseInOutElastic,
  EaseInBack,
  EaseOutBack,
  EaseInOutBack,
  EaseInBounce,
  EaseOutBounce,
  EaseInOutBounce,
}

impl Functions {

  pub fn interpolate(&self, t: f32) -> f32 {
    match self {
      Functions::EaseLinear => {
        ease_linear(t)
      },
      Functions::EaseStep => {
        ease_step(t)
      },
      Functions::EaseInQuad => {
        ease_in_quad(t)
      },
      Functions::EaseOutQuad => {
        ease_out_quad(t)
      },
      Functions::EaseInOutQuad => {
        ease_in_out_quad(t)
      },
      Functions::EaseInCubic => {
        ease_in_cubic(t)
      },
      Functions::EaseOutCubic => {
        ease_out_cubic(t)
      },
      Functions::EaseInOutCubic => {
        ease_in_out_cubic(t)
      },
      Functions::EaseInQuart => {
        ease_in_quart(t)
      },
      Functions::EaseOutQuart => {
        ease_out_quart(t)
      },
      Functions::EaseInOutQuart => {
        ease_in_out_quart(t)
      },
      Functions::EaseInQuint => {
        ease_in_quint(t)
      },
      Functions::EaseOutQuint => {
        ease_out_quint(t)
      },
      Functions::EaseInOutQuint => {
        ease_in_out_quint(t)
      },
      Functions::EaseInSine => {
        println!("Using EaseInSine");
        ease_in_sine(t)
      },
      Functions::EaseOutSine => {
        ease_out_sine(t)
      },
      Functions::EaseInOutSine => {
        ease_in_out_sine(t)
      },
      Functions::EaseInCirc => {
        ease_in_circ(t)
      },
      Functions::EaseOutCirc => {
        ease_out_circ(t)
      },
      Functions::EaseInOutCirc => {
        ease_in_out_circ(t)
      },
      Functions::EaseInExpo => {
        ease_in_expo(t)
      },
      Functions::EaseOutExpo => {
        ease_out_expo(t)
      },
      Functions::EaseInOutExpo => {
        ease_in_out_expo(t)
      },
      Functions::EaseInElastic => {
        ease_in_elastic(t)
      },
      Functions::EaseOutElastic => {
        ease_out_elastic(t)
      },
      Functions::EaseInOutElastic => {
        ease_in_out_elastic(t)
      },
      Functions::EaseInBack => {
        ease_in_back(t)
      },
      Functions::EaseOutBack => {
        ease_out_back(t)
      },
      Functions::EaseInOutBack => {
        ease_in_out_back(t)
      },
      Functions::EaseInBounce => {
        ease_in_bounce(t)
      },
      Functions::EaseOutBounce => {
        ease_out_bounce(t)
      },
      Functions::EaseInOutBounce => {
        ease_in_out_bounce(t)
      },
    }
  }
}