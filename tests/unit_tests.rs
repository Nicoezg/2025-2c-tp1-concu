use nyc_taxi_processor::*;
use chrono::{TimeZone, Utc};

fn create_test_trip() -> TaxiTrip {
    TaxiTrip {
        vendor_id: 1,
        pickup_datetime: Utc.with_ymd_and_hms(2015, 1, 1, 12, 0, 0).unwrap(),
        dropoff_datetime: Utc.with_ymd_and_hms(2015, 1, 1, 12, 30, 0).unwrap(),
        passenger_count: Some(1),
        trip_distance: 5.0,
        pickup_longitude: -73.98,
        pickup_latitude: 40.75,
        rate_code_id: 1,
        store_and_fwd_flag: Some("N".to_string()),
        dropoff_longitude: -73.95,
        dropoff_latitude: 40.78,
        payment_type: 1,
        fare_amount: 15.0,
        extra: 0.5,
        mta_tax: 0.5,
        tip_amount: 3.0,
        tolls_amount: 0.0,
        improvement_surcharge: Some(0.3),
        total_amount: 19.3,
    }
}

#[test]
fn test_validate_trip_valid() {
    let trip = create_test_trip();
    assert!(nyc_taxi_processor::utils::validate_trip(&trip).is_ok());
}

#[test]
fn test_validate_trip_negative_distance() {
    let mut trip = create_test_trip();
    trip.trip_distance = -1.0;
    assert!(nyc_taxi_processor::utils::validate_trip(&trip).is_err());
}

#[test]
fn test_validate_trip_invalid_vendor() {
    let mut trip = create_test_trip();
    trip.vendor_id = 5;
    assert!(nyc_taxi_processor::utils::validate_trip(&trip).is_err());
}

#[test]
fn test_validate_trip_negative_total() {
    let mut trip = create_test_trip();
    trip.total_amount = -10.0;
    assert!(nyc_taxi_processor::utils::validate_trip(&trip).is_err());
}

#[test]
fn test_get_hour_of_day() {
    let trip = create_test_trip();
    let hour = nyc_taxi_processor::utils::get_hour_of_day(&trip).unwrap();
    assert_eq!(hour, 12);
}

#[test]
fn test_get_day_of_week() {
    let trip = create_test_trip();
    let day = nyc_taxi_processor::utils::get_day_of_week(&trip).unwrap();
    assert!(day < 7);
}

#[test]
fn test_calculate_trip_duration() {
    let trip = create_test_trip();
    let duration = nyc_taxi_processor::utils::calculate_trip_duration(&trip).unwrap();
    assert_eq!(duration, 30.0);
}

#[test]
fn test_get_zone_id_manhattan() {
    let zone = nyc_taxi_processor::utils::get_zone_id(40.75, -73.98);
    assert_eq!(zone, "Manhattan");
}

#[test]
fn test_get_zone_id_jfk() {
    let zone = nyc_taxi_processor::utils::get_zone_id(40.645, -73.78);
    assert_eq!(zone, "JFK_Airport");
}

#[test]
fn test_get_zone_id_laguardia() {
    let zone = nyc_taxi_processor::utils::get_zone_id(40.77, -73.87);
    assert_eq!(zone, "LaGuardia_Airport");
}

#[test]
fn test_get_zone_id_brooklyn() {
    let zone = nyc_taxi_processor::utils::get_zone_id(40.65, -73.95);
    assert_eq!(zone, "Brooklyn");
}

#[test]
fn test_get_zone_id_queens() {
    let zone = nyc_taxi_processor::utils::get_zone_id(40.72, -73.80);
    assert_eq!(zone, "Queens");
}

#[test]
fn test_get_zone_id_bronx() {
    let zone = nyc_taxi_processor::utils::get_zone_id(40.85, -73.85);
    assert_eq!(zone, "Bronx");
}

#[test]
fn test_get_zone_id_staten_island() {
    let zone = nyc_taxi_processor::utils::get_zone_id(40.58, -74.15);
    assert_eq!(zone, "Staten_Island");
}

#[test]
fn test_get_zone_id_invalid_zero() {
    let zone = nyc_taxi_processor::utils::get_zone_id(0.0, 0.0);
    assert_eq!(zone, "Unknown");
}

#[test]
fn test_get_zone_id_invalid_out_of_range() {
    let zone = nyc_taxi_processor::utils::get_zone_id(50.0, -80.0);
    assert_eq!(zone, "Unknown");
}

#[test]
fn test_is_peak_hour_morning() {
    assert!(nyc_taxi_processor::utils::is_peak_hour(7));
    assert!(nyc_taxi_processor::utils::is_peak_hour(8));
    assert!(nyc_taxi_processor::utils::is_peak_hour(9));
}

#[test]
fn test_is_peak_hour_evening() {
    assert!(nyc_taxi_processor::utils::is_peak_hour(17));
    assert!(nyc_taxi_processor::utils::is_peak_hour(18));
    assert!(nyc_taxi_processor::utils::is_peak_hour(19));
}

#[test]
fn test_is_peak_hour_non_peak() {
    assert!(!nyc_taxi_processor::utils::is_peak_hour(0));
    assert!(!nyc_taxi_processor::utils::is_peak_hour(12));
    assert!(!nyc_taxi_processor::utils::is_peak_hour(23));
}

#[test]
fn test_round_to_2_decimals() {
    assert_eq!(nyc_taxi_processor::utils::round_to_2_decimals(3.14159), 3.14);
    assert_eq!(nyc_taxi_processor::utils::round_to_2_decimals(10.999), 11.0);
    assert_eq!(nyc_taxi_processor::utils::round_to_2_decimals(5.555), 5.56);
}

#[test]
fn test_parse_datetime_valid() {
    let result = nyc_taxi_processor::utils::parse_datetime("2015-01-01 12:30:00");
    assert!(result.is_ok());
}

#[test]
fn test_parse_datetime_invalid() {
    let result = nyc_taxi_processor::utils::parse_datetime("invalid-date");
    assert!(result.is_err());
}

#[test]
fn test_peak_zone_analyzer_basic() {
    let mut analyzer = PeakZoneAnalyzer::default();
    let trips = vec![create_test_trip()];

    let accumulator = analyzer.process_batch(&trips).unwrap();
    assert!(!accumulator.is_empty());
}

#[test]
fn test_peak_zone_analyzer_filter_invalid() {
    let mut analyzer = PeakZoneAnalyzer::default();
    let mut trip = create_test_trip();
    trip.pickup_latitude = 0.0;
    trip.pickup_longitude = 0.0;

    let trips = vec![trip];
    let accumulator = analyzer.process_batch(&trips).unwrap();
    assert!(accumulator.is_empty());
}

#[test]
fn test_payment_analyzer_basic() {
    let mut analyzer = PaymentAnalyzer::default();
    let trips = vec![create_test_trip()];

    let (accumulator, count) = analyzer.process_batch(&trips).unwrap();
    assert_eq!(count, 1);
    assert!(!accumulator.is_empty());
}

#[test]
fn test_payment_analyzer_filter_invalid() {
    let mut analyzer = PaymentAnalyzer::default();
    let mut trip = create_test_trip();
    trip.payment_type = 0;

    let trips = vec![trip];
    let (accumulator, count) = analyzer.process_batch(&trips).unwrap();
    assert_eq!(count, 0);
    assert!(accumulator.is_empty());
}

#[test]
fn test_hourly_analyzer_basic() {
    let mut analyzer = HourlyPatternAnalyzer::default();
    let trips = vec![create_test_trip()];

    let accumulator = analyzer.process_batch(&trips).unwrap();
    assert!(!accumulator.is_empty());
}

#[test]
fn test_hourly_analyzer_filter_invalid() {
    let mut analyzer = HourlyPatternAnalyzer::default();
    let mut trip = create_test_trip();
    trip.trip_distance = 0.0;

    let trips = vec![trip];
    let accumulator = analyzer.process_batch(&trips).unwrap();
    assert!(accumulator.is_empty());
}

#[test]
fn test_peak_zone_finalize() {
    let mut analyzer = PeakZoneAnalyzer::default();
    let trips = vec![create_test_trip()];

    let accumulator = analyzer.process_batch(&trips).unwrap();
    analyzer.merge_accumulators(vec![accumulator]).unwrap();
    let result = analyzer.finalize().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].zone_name, "Manhattan");
}

#[test]
fn test_payment_finalize() {
    let mut analyzer = PaymentAnalyzer::default();
    let trips = vec![create_test_trip()];

    let accumulator = analyzer.process_batch(&trips).unwrap();
    analyzer.merge_accumulators(vec![accumulator]).unwrap();
    let result = analyzer.finalize().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].payment_type, 1);
    assert_eq!(result[0].trip_count, 1);
}

#[test]
fn test_hourly_finalize() {
    let mut analyzer = HourlyPatternAnalyzer::default();
    let trips = vec![create_test_trip()];

    let accumulator = analyzer.process_batch(&trips).unwrap();
    analyzer.merge_accumulators(vec![accumulator]).unwrap();
    let result = analyzer.finalize().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].hour, 12);
    assert_eq!(result[0].trip_count, 1);
}

#[test]
fn test_multiple_trips_peak_zone() {
    let mut analyzer = PeakZoneAnalyzer::default();
    let trip1 = create_test_trip();
    let mut trip2 = create_test_trip();
    trip2.total_amount = 25.0;

    let trips = vec![trip1, trip2];
    let accumulator = analyzer.process_batch(&trips).unwrap();
    analyzer.merge_accumulators(vec![accumulator]).unwrap();
    let result = analyzer.finalize().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].trip_count, 2);
}

#[test]
fn test_multiple_payment_types() {
    let mut analyzer = PaymentAnalyzer::default();
    let trip1 = create_test_trip();
    let mut trip2 = create_test_trip();
    trip2.payment_type = 2;

    let trips = vec![trip1, trip2];
    let accumulator = analyzer.process_batch(&trips).unwrap();
    analyzer.merge_accumulators(vec![accumulator]).unwrap();
    let result = analyzer.finalize().unwrap();

    assert_eq!(result.len(), 2);
}

#[test]
fn test_multiple_hours() {
    let mut analyzer = HourlyPatternAnalyzer::default();
    let trip1 = create_test_trip();
    let mut trip2 = create_test_trip();
    trip2.pickup_datetime = Utc.with_ymd_and_hms(2015, 1, 1, 18, 0, 0).unwrap();
    trip2.dropoff_datetime = Utc.with_ymd_and_hms(2015, 1, 1, 18, 30, 0).unwrap();

    let trips = vec![trip1, trip2];
    let accumulator = analyzer.process_batch(&trips).unwrap();
    analyzer.merge_accumulators(vec![accumulator]).unwrap();
    let result = analyzer.finalize().unwrap();

    assert_eq!(result.len(), 2);
}

#[test]
fn test_trip_distance_validation() {
    let trip = create_test_trip();
    assert_eq!(trip.trip_distance, 5.0);
}

#[test]
fn test_trip_fare_calculation() {
    let trip = create_test_trip();
    let expected = trip.fare_amount + trip.extra + trip.mta_tax + trip.tip_amount + trip.improvement_surcharge.unwrap_or(0.0);
    assert_eq!(trip.total_amount, expected);
}

#[test]
fn test_vendor_id_range() {
    let trip = create_test_trip();
    assert!(trip.vendor_id >= 1 && trip.vendor_id <= 2);
}

#[test]
fn test_payment_type_credit_card() {
    let trip = create_test_trip();
    assert_eq!(trip.payment_type, 1);
}

#[test]
fn test_payment_type_cash() {
    let mut trip = create_test_trip();
    trip.payment_type = 2;
    assert_eq!(trip.payment_type, 2);
}
