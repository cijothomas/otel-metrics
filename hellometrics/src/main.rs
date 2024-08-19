use opentelemetry::instrument_builder::U64CounterBuilder;
use opentelemetry::instruments::U64Counter;
use opentelemetry::meter::Meter;
use opentelemetry::meterprovider::MeterProvider;
use opentelemetry_sdk::meterprovider::MeterProviderImpl;

fn main() {
    println!("Hello, world! from OpenTelemetry Metrics Example");

    // Replace with MeterProvider builder pattern to build it out.
    let provider = MeterProviderImpl;
    let meter = provider.get_meter("example");
    let counter = meter.u64_counter_builder("example_counter").build();
    counter.add(1, &[]);
}
