use crate::meter::{Meter, NoopMeter};

pub trait MeterProvider {
    type Meter: Meter;

    fn get_meter(&self, name: &str) -> Self::Meter;
}

pub struct NoopMeterProvider;

impl MeterProvider for NoopMeterProvider {
    type Meter = NoopMeter;

    fn get_meter(&self, _name: &str) -> Self::Meter {
        NoopMeter
    }
}
