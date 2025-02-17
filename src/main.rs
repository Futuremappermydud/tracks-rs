#![feature(let_chains)]

use std::{
    borrow::{Borrow, BorrowMut},
    error::Error,
    time::SystemTime,
};

use minifb::{Key, Window, WindowOptions};
use plotters::{
    backend::BGRXPixel,
    chart::ChartBuilder,
    prelude::*,
    series::LineSeries,
    style::{BLACK, Color, GREEN, IntoFont, TRANSPARENT},
};
use point_definition::{
    float_point_definition::FloatPointDefinition, PointDefinition,
};
use serde_json::json;
mod easings;
mod modifiers;
mod point_data;
mod point_definition;
mod values;

const W: usize = 800;
const H: usize = 600;

const FRAME_RATE: f64 = 120.0;

struct BufferWrapper(Vec<u32>);
impl Borrow<[u8]> for BufferWrapper {
    fn borrow(&self) -> &[u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.0.len() * 4) }
    }
}
impl BorrowMut<[u8]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe { std::slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut u8, self.0.len() * 4) }
    }
}
impl Borrow<[u32]> for BufferWrapper {
    fn borrow(&self) -> &[u32] {
        self.0.as_slice()
    }
}
impl BorrowMut<[u32]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u32] {
        self.0.as_mut_slice()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut window = Window::new("Tracks", W, H, WindowOptions::default())?;

    let mut buf = BufferWrapper(vec![0u32; W * H]);

    let cs = {
        let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
            buf.borrow_mut(),
            (W as u32, H as u32),
        )?
        .into_drawing_area();
        root.fill(&BLACK)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .set_all_label_area_size(30)
            .build_cartesian_2d(-1.2..1.2, -1.2..12.0)?;

        chart
            .configure_mesh()
            .label_style(("sans-serif", 15).into_font().color(&GREEN))
            .axis_style(GREEN)
            .draw()?;

        let cs = chart.into_chart_state();
        root.present()?;
        cs
    };

    let start_ts = SystemTime::now();
    let mut last_flushed = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let epoch = SystemTime::now()
            .duration_since(start_ts)
            .unwrap()
            .as_secs_f64();

        if epoch - last_flushed > 1.0 / FRAME_RATE {
            {
                let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
                    buf.borrow_mut(),
                    (W as u32, H as u32),
                )?
                .into_drawing_area();
                {
                    let mut chart = cs.clone().restore(&root);
                    chart.plotting_area().fill(&BLACK)?;

                    chart
                        .configure_mesh()
                        .bold_line_style(GREEN.mix(0.2))
                        .light_line_style(TRANSPARENT)
                        .draw()?;

                    let testing = json!([[0.0, 0.0], [5.0, 1.0, "easeInOutSine"],]);

                    let float_point_definition = FloatPointDefinition::new(&testing);

                    chart.draw_series(LineSeries::new(
                        (0f64..1f64)
                            .step(0.0001)
                            .values()
                            .map(|x| (x, float_point_definition.interpolate(x as f32).0 as f64)),
                        &BLUE,
                    ))?;
                }
                root.present()?;
            }

            window.update_with_buffer(buf.borrow(), W, H)?;
            last_flushed = epoch;
        }
    }
    Ok(())
}
