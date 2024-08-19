use crate::instruments::{
    F64Counter, F64Histogram, NoopF64Counter, NoopF64Histogram, NoopU64Counter, NoopU64Histogram,
    U64Counter, U64Histogram,
};

pub trait U64CounterBuilder {
    type U64Counter: U64Counter;

    fn with_unit(self, unit: &str) -> Self;
    fn with_description(self, description: &str) -> Self;
    fn build(self) -> Self::U64Counter;
}

pub trait F64CounterBuilder {
    type F64Counter: F64Counter;

    fn with_unit(self, unit: &str) -> Self;
    fn with_description(self, description: &str) -> Self;
    fn build(self) -> Self::F64Counter;
}

pub trait U64HistogramBuilder {
    type U64Histogram: U64Histogram;

    fn with_unit(self, unit: &str) -> Self;
    fn with_description(self, description: &str) -> Self;
    fn build(self) -> Self::U64Histogram;
}

pub trait F64HistogramBuilder {
    type F64Histogram: F64Histogram;

    fn with_unit(self, unit: &str) -> Self;
    fn with_description(self, description: &str) -> Self;
    fn build(self) -> Self::F64Histogram;
}

pub struct NoopU64CounterBuilder;

impl U64CounterBuilder for NoopU64CounterBuilder {
    type U64Counter = NoopU64Counter;

    fn with_unit(self, _unit: &str) -> Self {
        self
    }

    fn with_description(self, _description: &str) -> Self {
        self
    }

    fn build(self) -> Self::U64Counter {
        NoopU64Counter
    }
}

pub struct NoopF64CounterBuilder;

impl F64CounterBuilder for NoopF64CounterBuilder {
    type F64Counter = NoopF64Counter;

    fn with_unit(self, _unit: &str) -> Self {
        self
    }

    fn with_description(self, _description: &str) -> Self {
        self
    }

    fn build(self) -> Self::F64Counter {
        NoopF64Counter
    }
}

pub struct NoopU64HistogramBuilder;

impl U64HistogramBuilder for NoopU64HistogramBuilder {
    type U64Histogram = NoopU64Histogram;

    fn with_unit(self, _unit: &str) -> Self {
        self
    }

    fn with_description(self, _description: &str) -> Self {
        self
    }

    fn build(self) -> Self::U64Histogram {
        NoopU64Histogram
    }
}

pub struct NoopF64HistogramBuilder;

impl F64HistogramBuilder for NoopF64HistogramBuilder {
    type F64Histogram = NoopF64Histogram;

    fn with_unit(self, _unit: &str) -> Self {
        self
    }

    fn with_description(self, _description: &str) -> Self {
        self
    }

    fn build(self) -> Self::F64Histogram {
        NoopF64Histogram
    }
}
