//! Data model for a taxi trip record

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::datetime_format;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaxiTrip {
    /// Indicates Taxicab Passenger Enhancement Program (TPEP) that provided the
    /// register, either Creative Mobile Technologies (1) or VeriFone Inc. (2)
    #[serde(rename = "VendorID")]
    pub vendor_id: i32,

    /// Date and time when the meter was engaged
    #[serde(rename = "tpep_pickup_datetime", with = "datetime_format")]
    pub pickup_datetime: DateTime<Utc>,

    /// Date and time when the meter was disengaged
    #[serde(rename = "tpep_dropoff_datetime", with = "datetime_format")]
    pub dropoff_datetime: DateTime<Utc>,

    /// Number of passengers in the taxi
    #[serde(rename = "passenger_count")]
    pub passenger_count: Option<i32>,

    /// The elapsed trip distance in miles reported by the taximeter
    #[serde(rename = "trip_distance")]
    pub trip_distance: f64,

    /// Longitude where the meter was engaged
    #[serde(rename = "pickup_longitude")]
    pub pickup_longitude: f64,

    /// Latitude where the meter was engaged
    #[serde(rename = "pickup_latitude")]
    pub pickup_latitude: f64,

    /// The final rate code in effect at the end of the trip
    /// (more info in readme)
    #[serde(alias = "RateCodeID", alias = "RatecodeID")]
    pub rate_code_id: i32,

    /// This flag indicates whether the trip record was held in vehicle memory
    /// before sending to the vendor
    #[serde(rename = "store_and_fwd_flag")]
    pub store_and_fwd_flag: Option<String>,

    /// Longitude where the meter was disengaged
    #[serde(rename = "dropoff_longitude")]
    pub dropoff_longitude: f64,

    /// Latitude where the meter was disengaged
    #[serde(rename = "dropoff_latitude")]
    pub dropoff_latitude: f64,

    /// Indicates the payment method
    /// 1=Credit card, 2=Cash, 3=No charge,
    /// 4=Dispute, 5=Unknown, 6=Voided trip
    #[serde(rename = "payment_type")]
    pub payment_type: i32,

    /// The time-and-distance fare calculated by the meter
    #[serde(rename = "fare_amount")]
    pub fare_amount: f64,

    /// Additional charges incurred during the trip.
    /// Only includes the $0.50 and $1 rush hour and overnight charges
    #[serde(rename = "extra")]
    pub extra: f64,

    /// 0.5 MTA (Metropolitan Transportation Authority) tax that is automatically
    /// triggered based on the metered rate in use
    #[serde(rename = "mta_tax")]
    pub mta_tax: f64,

    /// This field is automatically populated for credit card trips.
    /// Cash tips are not included
    #[serde(rename = "tip_amount")]
    pub tip_amount: f64,

    /// Total amount of all tolls paid in trip
    #[serde(rename = "tolls_amount")]
    pub tolls_amount: f64,

    /// 0.30 improvement surcharge assessed trips at the flag drop.
    #[serde(rename = "improvement_surcharge")]
    pub improvement_surcharge: Option<f64>,

    /// The total amount charged to passengers. Does not include cash tips.
    #[serde(rename = "total_amount")]
    pub total_amount: f64,
}
