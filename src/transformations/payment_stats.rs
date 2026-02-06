//! A struct to hold payment statistics
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentStats {
    /// Payment type identifier
    /// 1=Credit card, 2=Cash, 3=No charge,
    /// 4=Dispute, 5=Unknown, 6=Voided trip
    pub payment_type: i32,

    /// Number of trips for this payment type
    pub trip_count: usize,

    /// Total amount charged for this payment type
    pub total_amount: f64,

    /// Average amount charged per trip
    pub avg_amount: f64,

    /// Percentage of trips for this payment type relative to all valid trips
    pub percentage: f64,
}
