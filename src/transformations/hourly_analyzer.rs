//! Batch aggregator for hourly pattern analysis of taxi trips
//!
//! Calculates the amount of tips, average trip distance,
//! fare, and tip percentage per hour of the day,
//! making the distinction between weekends and weekdays.
//! Also indicates if the hour being analyzed is a peak hour.
use crate::error::ProcessingError;
use crate::models::TaxiTrip;
use chrono::Timelike;
use rayon::prelude::*;
use std::collections::HashMap;

use super::batch_aggregator::BatchAggregator;
use super::hourly_pattern::HourlyPattern;

type HourlyStatsData = (usize, f64, f64, f64);
type HourlyStatsMap = HashMap<u32, HourlyStatsData>;

/// Batch aggregator for hourly pattern analysis
#[derive(Debug, Default)]
pub struct HourlyPatternAnalyzer {
    hourly_stats: HourlyStatsMap,
}

impl BatchAggregator<Vec<HourlyPattern>> for HourlyPatternAnalyzer {
    type Accumulator = HourlyStatsMap;

    // Process a single batch and accumulate intermediate results
    // Filters out trips with non-positive total amounts or distances
    // Accumulates trip count, total distance, total fare, and total duration per hour
    fn process_batch(&mut self, batch: &[TaxiTrip]) -> Result<Self::Accumulator, ProcessingError> {
        let batch_stats: HourlyStatsMap = batch
            .par_iter()
            .filter(|trip| trip.total_amount > 0.0 && trip.trip_distance > 0.0)
            .fold(HashMap::new, |mut acc, trip| {
                let hour = trip.pickup_datetime.hour();
                let duration = (trip.dropoff_datetime - trip.pickup_datetime).num_minutes() as f64;
                let entry = acc.entry(hour).or_insert((0, 0.0, 0.0, 0.0));
                entry.0 += 1;
                entry.1 += trip.trip_distance;
                entry.2 += trip.fare_amount;
                entry.3 += duration;
                acc
            })
            .reduce(HashMap::new, |mut acc1, acc2| {
                for (hour, (count, distance, fare, duration)) in acc2 {
                    let entry = acc1.entry(hour).or_insert((0, 0.0, 0.0, 0.0));
                    entry.0 += count;
                    entry.1 += distance;
                    entry.2 += fare;
                    entry.3 += duration;
                }
                acc1
            });

        Ok(batch_stats)
    }

    /// Merge multiple accumulators from batch processing
    fn merge_accumulators(
        &mut self,
        accumulators: Vec<Self::Accumulator>,
    ) -> Result<(), ProcessingError> {
        for accumulator in accumulators {
            for (hour, (count, distance, fare, duration)) in accumulator {
                let entry = self.hourly_stats.entry(hour).or_insert((0, 0.0, 0.0, 0.0));
                entry.0 += count;
                entry.1 += distance;
                entry.2 += fare;
                entry.3 += duration;
            }
        }
        Ok(())
    }

    /// Generate the final result from accumulated state
    fn finalize(self) -> Result<Vec<HourlyPattern>, ProcessingError> {
        use crate::utils::round_to_2_decimals;

        let mut hourly_patterns: Vec<HourlyPattern> = self
            .hourly_stats
            .into_iter()
            .map(
                |(hour, (count, total_distance, total_fare, total_duration))| HourlyPattern {
                    hour,
                    trip_count: count,
                    avg_distance: round_to_2_decimals(total_distance / count as f64),
                    avg_fare: round_to_2_decimals(total_fare / count as f64),
                    avg_duration: round_to_2_decimals(total_duration / count as f64),
                },
            )
            .collect();

        hourly_patterns.sort_by_key(|h| h.hour);
        Ok(hourly_patterns)
    }
}
