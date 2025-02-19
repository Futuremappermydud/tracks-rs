use std::cell::RefCell;

use glam::EulerRot;
use minifb::Window;
use plotters::{
    backend::BGRXPixel,
    chart::{ChartBuilder, ChartState},
    coord::{Shift, ranged3d::Cartesian3d, types::RangedCoordf64},
    prelude::{
        BitMapBackend, DiscreteRanged, DrawingArea, IntoLinspace,
    },
    series::LineSeries,
    style::{BLACK, Color, RED, RGBAColor, WHITE},
};
use serde_json::json;

use crate::{
    point_definition::{PointDefinition, quaternion_point_definition::QuaternionPointDefinition},
    values::base_provider_context::BaseProviderContext,
};

pub struct QuatContext {
    pub definition: QuaternionPointDefinition,
    pub context: RefCell<BaseProviderContext>,
}

impl QuatContext {
    pub fn new() -> Self {
        let context = BaseProviderContext::new();
        let definition = QuaternionPointDefinition::new(
            &json!([[0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 1.0]]),
            &context,
        );
        Self {
            definition,
            context: RefCell::new(context),
        }
    }
}

pub fn graph_quat(
    root: DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
) -> (
    ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>,
    QuatContext,
) {
    let mut chart = ChartBuilder::on(&root)
        .caption("3D Plot Test", ("sans", 20))
        .build_cartesian_3d(0.0..3.0, 0.0..3.0, 0.0..3.0)
        .unwrap();

    chart.with_projection(|mut pb| {
        pb.yaw = 0.5;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(1)
        .draw()
        .unwrap();

    (chart.into_chart_state(), QuatContext::new())
}

pub fn draw_quat(
    root: &DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
    chart: &ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>,
    context: &QuatContext,
    epoch: f64,
    _window: &Window,
) {
    {
        context
            .context
            .borrow_mut()
            .set_values("baseCombo", vec![(epoch.sin() as f32 + 1.0) * 0.5]);
        let mut chart = chart.clone().restore(&root);
        chart.plotting_area().fill(&WHITE).unwrap();

        chart.with_projection(|mut pb| {
            pb.yaw = epoch / 10.0;
            pb.pitch = 0.5;
            pb.scale = 0.9;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(1)
            .draw()
            .unwrap();

        let _ = (0.0..1.0)
            .step(0.01)
            .values()
            .map(|x| {
                let point = context
                    .definition
                    .interpolate(x as f32, &context.context.borrow())
                    .0;
                let to: [f32; 3] = point.to_euler(EulerRot::ZXY).into();

                chart
                    .draw_series(LineSeries::new(
                        [
                            (x, 0.0, 0.0),
                            (x + to[0] as f64, to[1] as f64, to[2] as f64),
                        ],
                        RGBAColor {
                            0: (255.0 * x) as u8,
                            1: (255.0 * (1.0 - x)) as u8,
                            2: 0,
                            3: 1.0,
                        },
                    ))
                    .unwrap();
            })
            .collect::<Vec<_>>();

        chart
            .configure_series_labels()
            .border_style(RED)
            .draw()
            .unwrap();
    }
}
