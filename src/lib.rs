//! # NYC Taxi Data Processor
pub mod error;
pub mod models;
pub mod processors;
pub mod transformations;
pub mod utils;

pub use models::TaxiTrip;
pub use processors::TaxiProcessor;
pub use transformations::{
    BatchAggregator, HourlyPattern, HourlyPatternAnalyzer, PaymentAnalyzer, PaymentStats, PeakZone,
    PeakZoneAnalyzer,
};
