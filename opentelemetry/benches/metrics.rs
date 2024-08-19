/*
    Benchmark Results:
    criterion = "0.5.1"
    OS: Ubuntu 22.04.4 LTS (5.15.153.1-microsoft-standard-WSL2)
    Hardware: CPU

	11th Gen Intel(R) Core(TM) i7-1185G7 @ 3.00GHz
    8 Cores 
    RAM: 48.0 GB

    | Test                                                | Average time| 
    |-----------------------------------------------------|-------------|
    | NoAttributes                                        | 0.0000 ps   | 1.18
    | AddWithInlineStaticAttributes                       | 08.480 ns   | 11.00
    | AddWithStaticArray                                  | 1.1612 ns   | 1.37
    | AddWithDynamicAttributes                            | 93.40 ns    | 94
    | AddWithDynamicAttributes_WithStringAllocation       | 62.338 ns   | 70
*/

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use opentelemetry::instrument_builder::U64CounterBuilder;
use opentelemetry::instruments::NoopU64Counter;
use opentelemetry::instruments::U64Counter;
use opentelemetry::keyvalue::KeyValue;
use opentelemetry::meter::Meter;
use opentelemetry::meterprovider::{MeterProvider, NoopMeterProvider};

// Run this benchmark with:
// cargo bench --bench metrics

fn create_counter() -> NoopU64Counter {
    let provider = NoopMeterProvider;
    let meter = provider.get_meter("example");
    let counter = meter.u64_counter_builder("example_counter").build();
    counter
}

fn criterion_benchmark(c: &mut Criterion) {
    counter_add(c);
}

fn counter_add(c: &mut Criterion) {
    let counter = create_counter();

    c.bench_function("NoAttributes", |b| {
        b.iter(|| {
            counter.add(1, &[]);
        });
    });

    c.bench_function("AddWithInlineStaticAttributes", |b| {
        b.iter(|| {
            counter.add(
                1,
                &[
                    KeyValue::new("attribute1", "value1"),
                    KeyValue::new("attribute2", "value2"),
                    KeyValue::new("attribute3", "value3"),
                    KeyValue::new("attribute4", "value4"),
                ],
            );
        });
    });

    let kv = [
        KeyValue::new("attribute1", "value1"),
        KeyValue::new("attribute2", "value2"),
        KeyValue::new("attribute3", "value3"),
        KeyValue::new("attribute4", "value4"),
    ];

    c.bench_function("AddWithStaticArray", |b| {
        b.iter(|| {
            counter.add(1, &kv);
        });
    });

    c.bench_function("AddWithDynamicAttributes", |b| {
        b.iter_batched(
            || {
                let value1 = "value1".to_string();
                let value2 = "value2".to_string();
                let value3 = "value3".to_string();
                let value4 = "value4".to_string();

                (value1, value2, value3, value4)
            },
            |values| {
                let kv = &[
                    KeyValue::new("attribute1", values.0),
                    KeyValue::new("attribute2", values.1),
                    KeyValue::new("attribute3", values.2),
                    KeyValue::new("attribute4", values.3),
                ];

                counter.add(1, kv);
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("AddWithDynamicAttributes_WithStringAllocation", |b| {
        b.iter(|| {
            let kv = &[
                KeyValue::new("attribute1", black_box("value1".to_string())),
                KeyValue::new("attribute2", black_box("value2".to_string())),
                KeyValue::new("attribute3", black_box("value3".to_string())),
                KeyValue::new("attribute4", black_box("value4".to_string())),
            ];

            counter.add(1, kv);
        });
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
