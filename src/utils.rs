//! Utility functions for taxi data processing and validation.

use crate::error::ProcessingError;
use crate::models::TaxiTrip;
use chrono::{Datelike, NaiveDateTime, Timelike};

/// Parses a datetime string in NYC TLC format.
///
/// This function parses datetime strings from NYC taxi data, which follow the format
/// "YYYY-MM-DD HH:MM:SS". It's used for both pickup and dropoff datetime fields.
pub fn parse_datetime(datetime_str: &str) -> Result<NaiveDateTime, ProcessingError> {
    NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").map_err(|_| {
        ProcessingError::Validation {
            message: format!("Invalid datetime format: {}", datetime_str),
        }
    })
}

/// Extracts the hour of day from a taxi trip's pickup time.
pub fn get_hour_of_day(trip: &TaxiTrip) -> Result<u8, ProcessingError> {
    Ok(trip.pickup_datetime.hour() as u8)
}

/// Extracts the day of week from a taxi trip's pickup time.
pub fn get_day_of_week(trip: &TaxiTrip) -> Result<u8, ProcessingError> {
    Ok(trip.pickup_datetime.weekday().num_days_from_monday() as u8)
}

/// Calculates the duration of a taxi trip in minutes.
pub fn calculate_trip_duration(trip: &TaxiTrip) -> Result<f64, ProcessingError> {
    let duration = trip.dropoff_datetime - trip.pickup_datetime;
    Ok(duration.num_minutes() as f64)
}

/// Determines the NYC location zone from latitude and longitude coordinates.
pub fn get_zone_id(lat: f64, lng: f64) -> String {
    // Handle invalid coordinates
    if lat == 0.0 || lng == 0.0 || !(40.0..=41.5).contains(&lat) || !(-75.0..=-73.0).contains(&lng)
    {
        return "Unknown".to_string();
    }

    // JFK Airport
    if (40.635..=40.655).contains(&lat) && (-73.795..=-73.755).contains(&lng) {
        return "JFK_Airport".to_string();
    }

    // LaGuardia Airport
    if (40.755..=40.785).contains(&lat) && (-73.895..=-73.855).contains(&lng) {
        return "LaGuardia_Airport".to_string();
    }

    // Manhattan
    if (40.695..=40.805).contains(&lat) && (-74.025..=-73.895).contains(&lng) {
        return "Manhattan".to_string();
    }

    // Bronx
    if (40.785..=40.925).contains(&lat) && (-73.935..=-73.755).contains(&lng) {
        return "Bronx".to_string();
    }

    // Brooklyn
    if (40.565..=40.745).contains(&lat) && (-74.045..=-73.825).contains(&lng) {
        return "Brooklyn".to_string();
    }

    // Queens
    if (40.535..=40.805).contains(&lat) && (-73.850..=-73.695).contains(&lng) {
        return "Queens".to_string();
    }

    // Staten Island
    if (40.475..=40.655).contains(&lat) && (-74.265..=-74.045).contains(&lng) {
        return "Staten_Island".to_string();
    }

    // NYC Other
    if (40.4..=41.0).contains(&lat) && (-74.3..=-73.7).contains(&lng) {
        return "NYC_Other".to_string();
    }

    // Outside NYC, Unknown
    "Unknown".to_string()
}

/// Determines if a given hour is considered a peak traffic hour.
pub fn is_peak_hour(hour: u8) -> bool {
    matches!(hour, 7..=9 | 17..=19)
}

/// Rounds a float to exactly two decimal places.
pub fn round_to_2_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

/// Validates a taxi trip record for data integrity.
pub fn validate_trip(trip: &TaxiTrip) -> Result<(), ProcessingError> {
    if trip.trip_distance < 0.0 {
        return Err(ProcessingError::Validation {
            message: "Trip distance cannot be negative".to_string(),
        });
    }

    if !matches!(trip.vendor_id, 1 | 2) {
        return Err(ProcessingError::Validation {
            message: format!("Invalid vendor ID: {}", trip.vendor_id),
        });
    }

    if trip.total_amount < 0.0 {
        return Err(ProcessingError::Validation {
            message: "Total amount cannot be negative".to_string(),
        });
    }

    Ok(())
}
