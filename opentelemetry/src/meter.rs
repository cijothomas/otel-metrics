use crate::instrument_builder::{
    F64CounterBuilder, F64HistogramBuilder, NoopF64CounterBuilder, NoopF64HistogramBuilder,
    NoopU64CounterBuilder, NoopU64HistogramBuilder, U64CounterBuilder, U64HistogramBuilder,
};

pub trait Meter {
    type U64CounterBuilder: U64CounterBuilder;
    type F64CounterBuilder: F64CounterBuilder;
    type U64HistogramBuilder: U64HistogramBuilder;
    type F64HistogramBuilder: F64HistogramBuilder;

    fn u64_counter_builder(&self, name: &str) -> Self::U64CounterBuilder;
    fn f64_counter_builder(&self, name: &str) -> Self::F64CounterBuilder;
    fn u64_histogram_builder(&self, name: &str) -> Self::U64HistogramBuilder;
    fn f64_histogram_builder(&self, name: &str) -> Self::F64HistogramBuilder;
}

pub struct NoopMeter;

impl Meter for NoopMeter {
    type U64CounterBuilder = NoopU64CounterBuilder;
    type F64CounterBuilder = NoopF64CounterBuilder;
    type U64HistogramBuilder = NoopU64HistogramBuilder;
    type F64HistogramBuilder = NoopF64HistogramBuilder;

    fn u64_counter_builder(&self, _name: &str) -> Self::U64CounterBuilder {
        NoopU64CounterBuilder
    }

    fn f64_counter_builder(&self, _name: &str) -> Self::F64CounterBuilder {
        NoopF64CounterBuilder
    }

    fn u64_histogram_builder(&self, _name: &str) -> Self::U64HistogramBuilder {
        NoopU64HistogramBuilder
    }

    fn f64_histogram_builder(&self, _name: &str) -> Self::F64HistogramBuilder {
        NoopF64HistogramBuilder
    }
}
