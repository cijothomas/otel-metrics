use opentelemetry::instrument_builder::U64CounterBuilder;
use opentelemetry::instruments::U64Counter;
use opentelemetry::keyvalue::KeyValue;
use opentelemetry::meter::Meter;
use opentelemetry::meterprovider::MeterProvider;
use opentelemetry_sdk::meterprovider::MeterProviderImpl;

fn main() {
    println!("Hello, world! from OpenTelemetry Metrics Example");

    // Replace with MeterProvider builder pattern to build it out.
    let provider = MeterProviderImpl;
    let meter = provider.get_meter("example");
    let counter = meter.u64_counter_builder("example_counter").build();
    counter.add(
        1,
        &[
            KeyValue::new("attribute1", "value1"),
            KeyValue::new("attribute2", "value2"),
            KeyValue::new("attribute3", "value3"),
            KeyValue::new("attribute4", "value4"),
        ],
    );
}
