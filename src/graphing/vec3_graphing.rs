use std::cell::RefCell;

use glam::Vec3Swizzles;
use minifb::{MouseButton, MouseMode, Window};
use plotters::{
    backend::BGRXPixel,
    chart::{ChartBuilder, ChartState},
    coord::{Shift, ranged3d::Cartesian3d, types::RangedCoordf64},
    prelude::{
        BitMapBackend, Circle, DiscreteRanged, DrawingArea, EmptyElement, IntoLinspace,
        PathElement, Text,
    },
    series::LineSeries,
    style::{BLACK, BLUE, Color, GREEN, IntoFont, RED, ShapeStyle, TRANSPARENT, WHITE},
};
use serde_json::json;

use crate::{
    point_definition::{PointDefinition, vector3_point_definition::Vector3PointDefinition},
    values::base_provider_context::BaseProviderContext,
};

pub struct Vec3Context {
    pub definition: Vector3PointDefinition,
    pub context: RefCell<BaseProviderContext>,
}

impl Vec3Context {
    pub fn new() -> Self {
        let context = BaseProviderContext::new();
        let definition = Vector3PointDefinition::new(
            &json!([[0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 3.0, 1.0]]),
            &context,
        );
        Self {
            definition,
            context: RefCell::new(context),
        }
    }
}

pub fn graph_vec3(
    root: DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
) -> (
    ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>,
    Vec3Context,
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

    (chart.into_chart_state(), Vec3Context::new())
}

pub fn draw_vec3(
    root: &DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
    chart: &ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>,
    context: &Vec3Context,
    epoch: f64,
    window: &Window,
) {
    {
        let mut chart = chart.clone().restore(&root);
        chart.plotting_area().fill(&WHITE).unwrap();

        chart.with_projection(|mut pb| {
            pb.yaw = epoch / 10.0;
            pb.scale = 0.9;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(1)
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0.0..1.0).step(0.0001).values().map(|x| {
                    let point = context
                        .definition
                        .interpolate(x as f32, &context.context.borrow())
                        .0;
                    (point.x as f64, point.y as f64, point.z as f64)
                }),
                &RED,
            ))
            .unwrap();

        let dot_and_label = |x: f64, y: f64, z: f64| {
            return EmptyElement::<(f64, f64, f64), BitMapBackend<BGRXPixel>>::at((x, y, z))
                + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
                + Text::new(
                    format!("({:.2},{:.2},{:.2})", x, y, z),
                    (10, 0),
                    ("sans-serif", 15.0).into_font(),
                );
        };

        let mut draw_T = |x: f32| {
          let point = context
            .definition
            .interpolate(x as f32, &context.context.borrow())
            .0;
          chart.draw_series(std::iter::once(dot_and_label(point.x as f64, point.y as f64, point.z as f64))).unwrap();
        };

        draw_T(0.0);
        draw_T(0.5);
        draw_T(1.0);

        chart
            .configure_series_labels()
            .border_style(RED)
            .draw()
            .unwrap();
    }
}
