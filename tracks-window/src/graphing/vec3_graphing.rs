use std::cell::RefCell;

use glam::Vec3;
use minifb::Window;
use plotters::{
    backend::BGRXPixel,
    chart::{ChartBuilder, ChartState},
    coord::{Shift, ranged3d::Cartesian3d, types::RangedCoordf64},
    prelude::{
        BitMapBackend, Circle, DiscreteRanged, DrawingArea, EmptyElement, IntoLinspace, Text,
    },
    series::LineSeries,
    style::{BLACK, BLUE, Color, IntoFont, RED, RGBColor, ShapeStyle, WHITE},
};
use serde_json::json;

use tracks_rs::{
    point_definition::{PointDefinition, vector3_point_definition::Vector3PointDefinition},
    values::base_provider_context::{BaseProviderContext, UpdatableProviderContext},
};

pub struct Vec3Context {
    pub definition: Vector3PointDefinition,
    pub definition2: Vector3PointDefinition,
    pub context: RefCell<BaseProviderContext>,
    pub updatable_provider: RefCell<UpdatableProviderContext>,
    pub last_epoch: RefCell<f64>,
}

impl Vec3Context {
    pub fn new() -> Self {
        let mut context = BaseProviderContext::new();
        let mut updatable_provider = UpdatableProviderContext::new();
        let definition = Vector3PointDefinition::new(
            json!(["baseLeftHandPosition"]),
            &mut context,
            &mut updatable_provider,
        );
        let definition2 = Vector3PointDefinition::new(
            json!(["baseLeftHandPosition.s10", [0, 0.2, 0, "opAdd"]]),
            &mut context,
            &mut updatable_provider,
        );
        Self {
            definition,
            definition2,
            context: RefCell::new(context),
            updatable_provider: RefCell::new(updatable_provider),
            last_epoch: RefCell::new(0.0),
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
    _window: &Window,
) {
    {
        context.context.borrow_mut().set_values(
            "baseLeftHandPosition",
            Vec3::new(epoch.sin() as f32 + 1.0, 2.0, 3.0).into(),
        );
        let mut chart: plotters::prelude::ChartContext<
            '_,
            BitMapBackend<'_, BGRXPixel>,
            Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>,
        > = chart.clone().restore(&root);
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

        let delta = epoch - context.last_epoch.borrow().clone();
        context.last_epoch.replace(epoch);

        context.updatable_provider.borrow_mut().update(delta as f32, &mut context.context.borrow_mut());

        let dot_and_label = |x: f64, y: f64, z: f64, color: RGBColor| {
            return EmptyElement::<(f64, f64, f64), BitMapBackend<BGRXPixel>>::at((x, y, z))
                + Circle::new((0, 0), 3, ShapeStyle::from(&color).filled())
                + Text::new(
                    format!("({:.2},{:.2},{:.2})", x, y, z),
                    (10, 0),
                    ("sans-serif", 15.0).into_font(),
                );
        };

        let mut draw_t = |x: f32| {
            let point = context
                .definition
                .interpolate(x as f32, &context.context.borrow())
                .0;
            let point2 = context
                .definition2
                .interpolate(x as f32, &context.context.borrow())
                .0;
            chart
                .draw_series(std::iter::once(dot_and_label(
                    point.x as f64,
                    point.y as f64,
                    point.z as f64,
                    RED,
                )))
                .unwrap();
            chart
                .draw_series(std::iter::once(dot_and_label(
                    point2.x as f64,
                    point2.y as f64,
                    point2.z as f64,
                    BLUE,
                )))
                .unwrap();
        };

        draw_t(0.0);
        draw_t(((epoch.sin() + 1.0) * 0.5) as f32);
        draw_t(1.0);

        chart
            .configure_series_labels()
            .border_style(RED)
            .draw()
            .unwrap();
    }
}
