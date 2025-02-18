use plotters::{
    backend::BGRXPixel,
    chart::{ChartBuilder, ChartState},
    coord::{types::RangedCoordf64, Shift},
    prelude::{BitMapBackend, Cartesian2d, DiscreteRanged, DrawingArea, IntoLinspace},
    series::LineSeries,
    style::{Color, IntoFont, BLACK, BLUE, GREEN, TRANSPARENT},
};

pub fn graph_2d(
    root: DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
) -> ChartState<Cartesian2d<RangedCoordf64, RangedCoordf64>> {
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .set_all_label_area_size(30)
        .build_cartesian_2d(-1.2..1.2, -1.2..12.0)
        .unwrap();

    chart
        .configure_mesh()
        .label_style(("sans-serif", 15).into_font().color(&GREEN))
        .axis_style(GREEN)
        .draw()
        .unwrap();

    chart.into_chart_state()
}

pub fn draw_2d(
    root: &DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
    chart: &ChartState<Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) {
    {
        let mut chart = chart.clone().restore(&root);
        chart.plotting_area().fill(&BLACK).unwrap();

        chart
            .configure_mesh()
            .bold_line_style(GREEN.mix(0.2))
            .light_line_style(TRANSPARENT)
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0f64..1f64).step(0.0001).values().map(|x| (x, x.sin())),
                &BLUE,
            ))
            .unwrap();
    }
}
