//! Payment analysis transformation module
//!
//! Analyzes the amount of trips, the total amount charged, the average amount per trip,
//! and the percentage of trips for each payment type (like credit card, cash, etc.)
use crate::error::ProcessingError;
use crate::models::TaxiTrip;
use rayon::prelude::*;
use std::collections::HashMap;

use super::batch_aggregator::BatchAggregator;
use super::payment_stats::PaymentStats;

type PaymentStatsData = (usize, f64);
type PaymentStatsMap = HashMap<i32, PaymentStatsData>;

/// Batch aggregator for payment analysis
#[derive(Debug, Default)]
pub struct PaymentAnalyzer {
    payment_stats: PaymentStatsMap,
    total_valid_trips: usize,
}

impl BatchAggregator<Vec<PaymentStats>> for PaymentAnalyzer {
    type Accumulator = (PaymentStatsMap, usize);

    /// Process a single batch and accumulate intermediate results
    /// Filters out trips with invalid payment types or non-positive total amounts
    /// Accumulates trip count and total amount per payment type
    fn process_batch(&mut self, batch: &[TaxiTrip]) -> Result<Self::Accumulator, ProcessingError> {
        let valid_trips: Vec<&TaxiTrip> = batch
            .iter()
            .filter(|trip| trip.payment_type > 0 && trip.total_amount > 0.0)
            .collect();

        let total_valid = valid_trips.len();

        let batch_stats: PaymentStatsMap = valid_trips
            .par_iter()
            .fold(HashMap::new, |mut acc, trip| {
                let entry = acc.entry(trip.payment_type).or_insert((0, 0.0));
                entry.0 += 1;
                entry.1 += trip.total_amount;
                acc
            })
            .reduce(HashMap::new, |mut acc1, acc2| {
                for (payment_type, (count, amount)) in acc2 {
                    let entry = acc1.entry(payment_type).or_insert((0, 0.0));
                    entry.0 += count;
                    entry.1 += amount;
                }
                acc1
            });

        Ok((batch_stats, total_valid))
    }

    /// Merge multiple accumulators from batch processing
    fn merge_accumulators(
        &mut self,
        accumulators: Vec<Self::Accumulator>,
    ) -> Result<(), ProcessingError> {
        for (accumulator, valid_count) in accumulators {
            self.total_valid_trips += valid_count;
            for (payment_type, (count, amount)) in accumulator {
                let entry = self.payment_stats.entry(payment_type).or_insert((0, 0.0));
                entry.0 += count;
                entry.1 += amount;
            }
        }
        Ok(())
    }

    /// Generate the final result from accumulated state
    fn finalize(self) -> Result<Vec<PaymentStats>, ProcessingError> {
        use crate::utils::round_to_2_decimals;

        let payment_results: Vec<PaymentStats> = self
            .payment_stats
            .into_iter()
            .map(|(payment_type, (count, total_amount))| PaymentStats {
                payment_type,
                trip_count: count,
                total_amount: round_to_2_decimals(total_amount),
                avg_amount: round_to_2_decimals(total_amount / count as f64),
                percentage: round_to_2_decimals(
                    (count as f64 / self.total_valid_trips as f64) * 100.0,
                ),
            })
            .collect();

        Ok(payment_results)
    }
}
