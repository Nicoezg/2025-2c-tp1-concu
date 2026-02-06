//! Multi-transformation analyzer that runs all transformations in a single pass
use crate::error::ProcessingError;
use crate::models::TaxiTrip;
use crate::transformations::{
    BatchAggregator, HourlyPattern, HourlyPatternAnalyzer, PaymentAnalyzer, PaymentStats, PeakZone,
    PeakZoneAnalyzer,
};
use serde::{Deserialize, Serialize};

/// Combined results from all three transformations
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiAnalysisResults {
    pub peak_zones: Vec<PeakZone>,
    pub hourly_patterns: Vec<HourlyPattern>,
    pub payment_analysis: Vec<PaymentStats>,
}

/// Accumulator for all three transformations
#[derive(Debug)]
pub struct MultiAccumulator {
    pub peak_zones_acc: <PeakZoneAnalyzer as BatchAggregator<Vec<PeakZone>>>::Accumulator,
    pub hourly_patterns_acc:
        <HourlyPatternAnalyzer as BatchAggregator<Vec<HourlyPattern>>>::Accumulator,
    pub payment_analysis_acc: <PaymentAnalyzer as BatchAggregator<Vec<PaymentStats>>>::Accumulator,
}

impl Default for MultiAccumulator {
    fn default() -> Self {
        use std::collections::HashMap;
        Self {
            peak_zones_acc: HashMap::new(),
            hourly_patterns_acc: HashMap::new(),
            payment_analysis_acc: (HashMap::new(), 0),
        }
    }
}

/// Processes all three transformations in a single pass
#[derive(Debug, Default)]
pub struct MultiAnalyzer {
    peak_zone_analyzer: PeakZoneAnalyzer,
    hourly_pattern_analyzer: HourlyPatternAnalyzer,
    payment_analyzer: PaymentAnalyzer,
}

impl BatchAggregator<MultiAnalysisResults> for MultiAnalyzer {
    type Accumulator = MultiAccumulator;

    fn process_batch(&mut self, batch: &[TaxiTrip]) -> Result<Self::Accumulator, ProcessingError> {
        // Process the same batch through all three analyzers
        let peak_zones_acc = self.peak_zone_analyzer.process_batch(batch)?;
        let hourly_patterns_acc = self.hourly_pattern_analyzer.process_batch(batch)?;
        let payment_analysis_acc = self.payment_analyzer.process_batch(batch)?;

        Ok(MultiAccumulator {
            peak_zones_acc,
            hourly_patterns_acc,
            payment_analysis_acc,
        })
    }

    fn merge_accumulators(
        &mut self,
        accumulators: Vec<Self::Accumulator>,
    ) -> Result<(), ProcessingError> {
        // Separate accumulators by type
        let mut peak_zones_accs = Vec::new();
        let mut hourly_patterns_accs = Vec::new();
        let mut payment_analysis_accs = Vec::new();

        for acc in accumulators {
            peak_zones_accs.push(acc.peak_zones_acc);
            hourly_patterns_accs.push(acc.hourly_patterns_acc);
            payment_analysis_accs.push(acc.payment_analysis_acc);
        }

        // Merge each transformation's accumulators
        self.peak_zone_analyzer
            .merge_accumulators(peak_zones_accs)?;
        self.hourly_pattern_analyzer
            .merge_accumulators(hourly_patterns_accs)?;
        self.payment_analyzer
            .merge_accumulators(payment_analysis_accs)?;

        Ok(())
    }

    fn finalize(self) -> Result<MultiAnalysisResults, ProcessingError> {
        // Finalize all three transformations
        let peak_zones = self.peak_zone_analyzer.finalize()?;
        let hourly_patterns = self.hourly_pattern_analyzer.finalize()?;
        let payment_analysis = self.payment_analyzer.finalize()?;

        Ok(MultiAnalysisResults {
            peak_zones,
            hourly_patterns,
            payment_analysis,
        })
    }
}
