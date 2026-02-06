//! Defines the PeakZone struct used in peak zone analysis.
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PeakZone {
    /// Name of the zone
    pub zone_name: String,

    /// Hour of the day (0-23)
    pub hour: u32,

    /// Number of trips in this zone during the specified hour
    pub trip_count: usize,

    /// Total revenue generated
    pub total_revenue: f64,

    /// Average fare amount
    pub avg_fare: f64,

    /// Center latitude of the zone based on trip coordinates
    pub center_lat: f64,

    /// Center longitude of the zone based on trip coordinates
    pub center_lng: f64,
}
