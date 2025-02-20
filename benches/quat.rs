use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::json;
use std::hint::black_box;
use tracks_rs::{
    point_definition::{
        PointDefinition, float_point_definition::FloatPointDefinition,
        quaternion_point_definition::QuaternionPointDefinition,
    },
    values::base_provider_context::BaseProviderContext,
};

fn point_step(n: u64) {
    let context = BaseProviderContext::new();
    let definition = QuaternionPointDefinition::new(
        json!([[0.0, 0.0, 0.0, 0.0], [1.0, 1.0, 1.0, 1.0, "easeInOutSine"]]),
        &context,
    );

    // let step = 1.0 / n as f32;

    let values: Vec<f64> = (0..=(n as usize)).map(|i| i as f64 / n as f64).collect();

    values.into_iter().for_each(|x| {
        black_box(definition.interpolate(x as f32, &context));
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("quat4", |b| b.iter(|| point_step(1000)));
    c.bench_function("quat5", |b| b.iter(|| point_step(10000)));
    c.bench_function("quat6", |b| b.iter(|| point_step(100000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
