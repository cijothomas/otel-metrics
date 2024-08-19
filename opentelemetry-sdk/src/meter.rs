use opentelemetry::meter::Meter;

use crate::instruments::{
    F64CounterBuilderImpl, F64HistogramBuilderImpl, U64CounterBuilderImpl, U64HistogramBuilderImpl,
};

pub struct MeterImpl;

impl Meter for MeterImpl {
    type U64CounterBuilder = U64CounterBuilderImpl;
    type F64CounterBuilder = F64CounterBuilderImpl;
    type U64HistogramBuilder = U64HistogramBuilderImpl;
    type F64HistogramBuilder = F64HistogramBuilderImpl;

    fn u64_counter_builder(&self, name: &str) -> Self::U64CounterBuilder {
        U64CounterBuilderImpl::new(name)
    }

    fn f64_counter_builder(&self, name: &str) -> Self::F64CounterBuilder {
        F64CounterBuilderImpl::new(name)
    }

    fn u64_histogram_builder(&self, name: &str) -> Self::U64HistogramBuilder {
        U64HistogramBuilderImpl::new(name)
    }

    fn f64_histogram_builder(&self, name: &str) -> Self::F64HistogramBuilder {
        F64HistogramBuilderImpl::new(name)
    }
}
