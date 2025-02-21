use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::json;
use std::hint::black_box;
use tracks_rs::{
    point_definition::{
        PointDefinition, float_point_definition::FloatPointDefinition,
        vector4_point_definition::Vector4PointDefinition,
    },
    values::base_provider_context::BaseProviderContext,
};

fn point_step(n: u64) {
    let context = BaseProviderContext::new();
    let definition = Vector4PointDefinition::new(
        json!([
            [0.0, 1.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 1.0, 1.0, 1.0, "easeInOutSine"]
        ]),
        &context,
    );

    // let step = 1.0 / n as f32;

    let values: Vec<f64> = (0..=(n as usize)).map(|i| i as f64 / n as f64).collect();

    values.into_iter().for_each(|x| {
        black_box(definition.interpolate(x as f32, &context));
    });
}
fn point_step_slow(n: u64) {
    let context = track_rs_old::values::base_provider_context::BaseProviderContext::new();
    let definition =
        track_rs_old::point_definition::vector4_point_definition::Vector4PointDefinition::new(
        &json!([
            [0.0, 1.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 1.0, 1.0, 1.0, "easeInOutSine"]
        ]),
        &context,
    );

    // let step = 1.0 / n as f32;

    let values: Vec<f64> = (0..=(n as usize)).map(|i| i as f64 / n as f64).collect();

    values.into_iter().for_each(|x| {
        black_box(
            track_rs_old::point_definition::PointDefinition::interpolate(
                &definition,
                x as f32,
                &context,
            ),
        );
    });
}

fn benchmark_both(n: u64, c: &mut Criterion) {
    let mut group = c.benchmark_group("vec4");

    group.bench_function(format!("vec4_{n}"), |b| b.iter(|| point_step(n)));
    group.bench_function(format!("vec4_slow_{n}"), |b| b.iter(|| point_step_slow(n)));
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_both(1000, c);
    benchmark_both(10000, c);
    benchmark_both(100000, c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
