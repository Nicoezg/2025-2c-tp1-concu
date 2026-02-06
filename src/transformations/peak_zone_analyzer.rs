//! Peak zones analysis module
//!
//! Analyzes taxi trip data to identify peak zones based on pickup locations and times.
//! Calculates total revenue, average fare, amount of trips for each zone and hour and
//! determines the center coordinates of each zone. Identifies the top 50 zones in
//! specific hours with the highest revenue.
use crate::error::ProcessingError;
use crate::models::TaxiTrip;
use chrono::Timelike;
use rayon::prelude::*;
use std::collections::HashMap;

use super::batch_aggregator::BatchAggregator;
use super::peak_zone::PeakZone;

type ZoneKey = (String, u32);
type ZoneStatsData = (usize, f64, f64, f64, f64, usize);
type ZoneStatsMap = HashMap<ZoneKey, ZoneStatsData>;

/// Batch aggregator for peak zone analysis
#[derive(Debug, Default)]
pub struct PeakZoneAnalyzer {
    zone_stats: ZoneStatsMap,
}

impl BatchAggregator<Vec<PeakZone>> for PeakZoneAnalyzer {
    type Accumulator = ZoneStatsMap;

    /// Process a single batch and accumulate intermediate results
    /// Filters out trips with invalid coordinates or non-positive total amounts
    /// Accumulates trip count, total revenue, total fare, and sums of coordinates per
    /// zone and hour
    fn process_batch(&mut self, batch: &[TaxiTrip]) -> Result<Self::Accumulator, ProcessingError> {
        use crate::utils::get_zone_id;

        let batch_stats: ZoneStatsMap = batch
            .par_iter()
            .filter(|trip| {
                trip.pickup_latitude != 0.0
                    && trip.pickup_longitude != 0.0
                    && trip.total_amount > 0.0
            })
            .fold(HashMap::new, |mut acc, trip| {
                let hour = trip.pickup_datetime.hour();
                let zone_id = get_zone_id(trip.pickup_latitude, trip.pickup_longitude);
                let key = (zone_id, hour);
                let entry = acc.entry(key).or_insert((0, 0.0, 0.0, 0.0, 0.0, 0));
                entry.0 += 1; // trip count
                entry.1 += trip.total_amount; // total revenue
                entry.2 += trip.fare_amount; // total fare
                entry.3 += trip.pickup_latitude; // lat sum for averaging
                entry.4 += trip.pickup_longitude; // lng sum for averaging
                entry.5 += 1; // coordinate count for averaging
                acc
            })
            .reduce(HashMap::new, |mut acc1, acc2| {
                for ((zone_id, hour), (count, total, fare, lat_sum, lng_sum, coord_count)) in acc2 {
                    let entry = acc1
                        .entry((zone_id, hour))
                        .or_insert((0, 0.0, 0.0, 0.0, 0.0, 0));
                    entry.0 += count;
                    entry.1 += total;
                    entry.2 += fare;
                    entry.3 += lat_sum;
                    entry.4 += lng_sum;
                    entry.5 += coord_count;
                }
                acc1
            });

        Ok(batch_stats)
    }

    /// Merges multiple accumulators into the main state
    fn merge_accumulators(
        &mut self,
        accumulators: Vec<Self::Accumulator>,
    ) -> Result<(), ProcessingError> {
        for accumulator in accumulators {
            for ((zone_id, hour), (count, total, fare, lat_sum, lng_sum, coord_count)) in
                accumulator
            {
                let entry = self
                    .zone_stats
                    .entry((zone_id, hour))
                    .or_insert((0, 0.0, 0.0, 0.0, 0.0, 0));
                entry.0 += count;
                entry.1 += total;
                entry.2 += fare;
                entry.3 += lat_sum;
                entry.4 += lng_sum;
                entry.5 += coord_count;
            }
        }
        Ok(())
    }

    /// Finalizes the analysis and produces the top 50 peak zones by total revenue
    fn finalize(self) -> Result<Vec<PeakZone>, ProcessingError> {
        use crate::utils::round_to_2_decimals;

        let mut peak_zones: Vec<PeakZone> = self
            .zone_stats
            .into_iter()
            .map(
                |(
                    (zone_id, hour),
                    (count, total_revenue, total_fare, lat_sum, lng_sum, coord_count),
                )| PeakZone {
                    zone_name: zone_id,
                    hour,
                    trip_count: count,
                    total_revenue: round_to_2_decimals(total_revenue),
                    avg_fare: round_to_2_decimals(total_fare / count as f64),
                    center_lat: round_to_2_decimals(lat_sum / coord_count as f64),
                    center_lng: round_to_2_decimals(lng_sum / coord_count as f64),
                },
            )
            .collect();

        peak_zones.sort_by(|a, b| b.total_revenue.partial_cmp(&a.total_revenue).unwrap());
        peak_zones.truncate(50); // Top 50 zones

        Ok(peak_zones)
    }
}
