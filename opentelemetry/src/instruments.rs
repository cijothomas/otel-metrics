use crate::keyvalue::KeyValue;

pub trait U64Counter {
    fn add(&self, _value: u64, _labels: &[KeyValue]) {}
}

pub trait F64Counter {
    fn add(&self, _value: f64, _labels: &[KeyValue]) {}
}

pub trait U64Histogram {
    fn record(&self, _value: u64, _labels: &[KeyValue]) {}
}

pub trait F64Histogram {
    fn record(&self, _value: f64, _labels: &[KeyValue]) {}
}

pub struct NoopU64Counter;

impl U64Counter for NoopU64Counter {}

pub struct NoopF64Counter;

impl F64Counter for NoopF64Counter {}

pub struct NoopU64Histogram;

impl U64Histogram for NoopU64Histogram {}

pub struct NoopF64Histogram;

impl F64Histogram for NoopF64Histogram {}
