use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyPattern {
    pub hour: u32,
    pub trip_count: usize,
    pub avg_distance: f64,
    pub avg_fare: f64,
    pub avg_duration: f64,
}
