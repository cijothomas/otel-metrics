use opentelemetry::meterprovider::MeterProvider;

use crate::meter::MeterImpl;

pub struct MeterProviderImpl;

impl MeterProvider for MeterProviderImpl {
    type Meter = MeterImpl;

    fn get_meter(&self, _name: &str) -> Self::Meter {
        MeterImpl { /* Initialization */ }
    }
}
