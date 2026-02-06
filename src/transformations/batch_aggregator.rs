//! A trait for batch aggregation of taxi trip data
use crate::error::ProcessingError;
use crate::models::TaxiTrip;

pub trait BatchAggregator<T> {
    /// The type of intermediate state accumulated during batch processing
    type Accumulator: Send + Default;

    /// Process a single batch and accumulate intermediate results
    fn process_batch(&mut self, batch: &[TaxiTrip]) -> Result<Self::Accumulator, ProcessingError>;

    /// Merge multiple accumulators from parallel batch processing
    fn merge_accumulators(
        &mut self,
        accumulators: Vec<Self::Accumulator>,
    ) -> Result<(), ProcessingError>;

    /// Generate the final result from accumulated state
    fn finalize(self) -> Result<T, ProcessingError>;
}
