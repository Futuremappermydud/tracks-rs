use easings::functions::Functions;
use serde_json::json;

mod easings;

fn main() {
  let testing = json!([
    [
      0,
      0
    ],
    [
      15,
      1,
      "easeInExpo"
    ]
    ]);

  let value = easings::interpolate(0.5, Functions::EaseStep);
  println!("Hello, world! {} {}", testing, value);
}
