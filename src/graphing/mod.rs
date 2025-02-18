use std::{
    borrow::{Borrow, BorrowMut},
    time::SystemTime,
};

use float_graphing::FloatContext;
use minifb::{Key, Window, WindowOptions};
use plotters::{
    backend::BGRXPixel,
    chart::ChartState,
    coord::{ranged3d::Cartesian3d, types::RangedCoordf64},
    prelude::{BitMapBackend, Cartesian2d, IntoDrawingArea},
    style::BLACK,
};
use vec3_graphing::Vec3Context;

pub mod float_graphing;
pub mod vec3_graphing;

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

enum GraphContext<'a> {
    Float2D(
        &'a ChartState<Cartesian2d<RangedCoordf64, RangedCoordf64>>,
        &'a FloatContext,
    ),
    Vector3D(
        &'a ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>,
        &'a Vec3Context,
    ),
}

pub fn graph(context: &str) {
    let mut window = Window::new("Tracks", W, H, WindowOptions::default()).unwrap();
    let mut buf = BufferWrapper(vec![0u32; W * H]);

    let mut binding_2d: Option<ChartState<Cartesian2d<RangedCoordf64, RangedCoordf64>>> = None;
    let mut binding_3d: Option<
        ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>,
    > = None;
    let mut float_context: Option<FloatContext> = None;
    let mut vec3_context: Option<Vec3Context> = None;

    let cs: GraphContext = {
        let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
            buf.borrow_mut(),
            (W as u32, H as u32),
        )
        .unwrap()
        .into_drawing_area();

        root.fill(&BLACK).unwrap();

        match context {
            "2d" => {
                let (new_binding, new_context) = float_graphing::graph_2d(root);
                binding_2d = Some(new_binding);
                float_context = Some(new_context);
                GraphContext::Float2D(
                    binding_2d.as_ref().unwrap(),
                    float_context.as_ref().unwrap(),
                )
            }
            "3d" => {
                let (new_binding, new_context) = vec3_graphing::graph_vec3(root);
                binding_3d = Some(new_binding);
                vec3_context = Some(new_context);
                GraphContext::Vector3D(binding_3d.as_ref().unwrap(), vec3_context.as_ref().unwrap())
            }
            _ => panic!("Invalid context"),
        }
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
                )
                .unwrap()
                .into_drawing_area();
                match cs {
                    GraphContext::Float2D(state, context) => {
                        float_graphing::draw_2d(&root, state, context, epoch)
                    }
                    GraphContext::Vector3D(state, context) => {
                        vec3_graphing::draw_vec3(&root, state, context, epoch, &window)
                    }
                }
                root.present().unwrap();
            }

            window.update_with_buffer(buf.borrow(), W, H).unwrap();
            last_flushed = epoch;
        }
    }
}
