use core::fmt;

use opentelemetry::{
    instrument_builder::{
        F64CounterBuilder, F64HistogramBuilder, U64CounterBuilder, U64HistogramBuilder,
    },
    instruments::{F64Counter, F64Histogram, U64Counter, U64Histogram},
};

pub struct U64CounterImpl {
    name: String,
    unit: Option<String>,
    description: Option<String>,
}

impl U64Counter for U64CounterImpl {
    fn add(&self, value: u64, labels: &[(&'static str, &'static str)]) {
        println!("Measurement recorded with value: {} {:?}", value, labels);
    }
}

impl fmt::Debug for U64CounterImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "U64CounterImpl {{ name: {}, unit: {:?}, description: {:?} }}",
            self.name, self.unit, self.description
        )
    }
}

pub struct F64CounterImpl {
    name: String,
    unit: Option<String>,
    description: Option<String>,
}

impl F64Counter for F64CounterImpl {
    fn add(&self, value: f64, labels: &[(&'static str, &'static str)]) {
        println!("Measurement recorded with value: {} {:?}", value, labels);
    }
}

impl fmt::Debug for F64CounterImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "F64CounterImpl {{ name: {}, unit: {:?}, description: {:?} }}",
            self.name, self.unit, self.description
        )
    }
}

pub struct U64HistogramImpl {
    name: String,
    unit: Option<String>,
    description: Option<String>,
}

impl U64Histogram for U64HistogramImpl {
    fn record(&self, value: u64, labels: &[(&'static str, &'static str)]) {
        println!("Measurement recorded with value: {} {:?}", value, labels);
    }
}

impl fmt::Debug for U64HistogramImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "U64HistogramImpl {{ name: {}, unit: {:?}, description: {:?} }}",
            self.name, self.unit, self.description
        )
    }
}

pub struct F64HistogramImpl {
    name: String,
    unit: Option<String>,
    description: Option<String>,
}

impl F64Histogram for F64HistogramImpl {
    fn record(&self, value: f64, labels: &[(&'static str, &'static str)]) {
        println!("Measurement recorded with value: {} {:?}", value, labels);
    }
}

impl fmt::Debug for F64HistogramImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "F64HistogramImpl {{ name: {}, unit: {:?}, description: {:?} }}",
            self.name, self.unit, self.description
        )
    }
}

// builders
pub struct U64CounterBuilderImpl {
    name: String,
    unit: Option<String>,
    description: Option<String>,
}

impl U64CounterBuilderImpl {
    pub fn new(name: &str) -> Self {
        U64CounterBuilderImpl {
            name: name.to_string(),
            unit: None,
            description: None,
        }
    }
}

impl U64CounterBuilder for U64CounterBuilderImpl {
    type U64Counter = U64CounterImpl;

    fn with_unit(mut self, unit: &str) -> Self {
        self.unit = Some(unit.to_string());
        self
    }

    fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    fn build(self) -> Self::U64Counter {
        U64CounterImpl {
            name: self.name,
            unit: self.unit,
            description: self.description,
        }
    }
}

pub struct F64CounterBuilderImpl {
    name: String,
    unit: Option<String>,
    description: Option<String>,
}

impl F64CounterBuilderImpl {
    pub fn new(name: &str) -> Self {
        F64CounterBuilderImpl {
            name: name.to_string(),
            unit: None,
            description: None,
        }
    }
}

impl F64CounterBuilder for F64CounterBuilderImpl {
    type F64Counter = F64CounterImpl;

    fn with_unit(mut self, unit: &str) -> Self {
        self.unit = Some(unit.to_string());
        self
    }

    fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    fn build(self) -> Self::F64Counter {
        F64CounterImpl {
            name: self.name,
            unit: self.unit,
            description: self.description,
        }
    }
}

pub struct U64HistogramBuilderImpl {
    name: String,
    unit: Option<String>,
    description: Option<String>,
}

impl U64HistogramBuilderImpl {
    pub fn new(name: &str) -> Self {
        U64HistogramBuilderImpl {
            name: name.to_string(),
            unit: None,
            description: None,
        }
    }
}

impl U64HistogramBuilder for U64HistogramBuilderImpl {
    type U64Histogram = U64HistogramImpl;

    fn with_unit(mut self, unit: &str) -> Self {
        self.unit = Some(unit.to_string());
        self
    }

    fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    fn build(self) -> Self::U64Histogram {
        U64HistogramImpl {
            name: self.name,
            unit: self.unit,
            description: self.description,
        }
    }
}

pub struct F64HistogramBuilderImpl {
    name: String,
    unit: Option<String>,
    description: Option<String>,
}

impl F64HistogramBuilderImpl {
    pub fn new(name: &str) -> Self {
        F64HistogramBuilderImpl {
            name: name.to_string(),
            unit: None,
            description: None,
        }
    }
}

impl F64HistogramBuilder for F64HistogramBuilderImpl {
    type F64Histogram = F64HistogramImpl;

    fn with_unit(mut self, unit: &str) -> Self {
        self.unit = Some(unit.to_string());
        self
    }

    fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    fn build(self) -> Self::F64Histogram {
        F64HistogramImpl {
            name: self.name,
            unit: self.unit,
            description: self.description,
        }
    }
}
