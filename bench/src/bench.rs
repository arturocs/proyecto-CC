use criterion::{black_box, criterion_group, criterion_main, Criterion};
use device::{rdevice::RDevice, rwdevice::RWDevice, Readable, Writable};
use serde_json::json;

pub fn criterion_benchmark1(c: &mut Criterion) {
    c.bench_function("get_status", |b| {
        let mockup_device = RDevice::new(
            "mockup_device",
            "fake_plugin",
            "../target/release/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();
        b.iter(|| {
            let status = black_box(mockup_device.get_status().unwrap());
            black_box(status.get("data").unwrap());
        })
    });
}

pub fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("create_device", |b| {
        b.iter(|| {
            black_box(
                RDevice::new(
                    "mockup_device",
                    "fake_plugin",
                    "../target/release/libfake_plugin.so",
                    "127.0.0.1",
                )
                .unwrap(),
            );
        })
    });
}

pub fn criterion_benchmark3(c: &mut Criterion) {
    c.bench_function("set_status", |b| {
        let mockup_device = RWDevice::new(
            "mockup_device",
            "fake_plugin",
            "../target/release/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();
        b.iter(|| {
            let status = black_box(mockup_device.set_status(&json!({"on":true})).unwrap());
            black_box(status.get("data").unwrap());
        })
    });
}

criterion_group!(
    benches,
    criterion_benchmark1,
    criterion_benchmark2,
    criterion_benchmark3,
);
criterion_main!(benches);