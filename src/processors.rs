//! Module for processing NYC Taxi data with memory-efficient and parallel processing
use crate::error::ProcessingError;
use crate::models::TaxiTrip;
use crate::transformations::{BatchAggregator, MultiAnalyzer};
use csv::Reader;
use rayon::prelude::*;
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::time::Instant;

pub struct TaxiProcessor {
    pub chunk_size: usize,
}

impl TaxiProcessor {
    pub fn new() -> Self {
        Self { chunk_size: 50_000 }
    }

    pub fn with_chunk_size(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    pub fn process_in_batches<F>(
        &self,
        file_path: &str,
        mut batch_processor: F,
    ) -> Result<(), ProcessingError>
    where
        F: FnMut(&[TaxiTrip]) -> Result<(), ProcessingError>,
    {
        let file = File::open(file_path)?;
        let mut reader = Reader::from_reader(BufReader::new(file));
        let mut batch = Vec::with_capacity(self.chunk_size);

        for result in reader.deserialize::<TaxiTrip>() {
            let trip = result?;

            if crate::utils::validate_trip(&trip).is_ok() {
                batch.push(trip);
            }

            if batch.len() >= self.chunk_size {
                batch_processor(&batch)?;
                batch.clear(); // Free memory immediately
            }
        }

        // Process remaining batch
        if !batch.is_empty() {
            batch_processor(&batch)?;
        }

        Ok(())
    }

    /// Generic streaming transformation runner using batch aggregators
    fn run_streaming_transformation<A, T>(&self, input_path: &str) -> Result<T, ProcessingError>
    where
        A: BatchAggregator<T> + Default + Send,
        A::Accumulator: Send,
        T: Send,
    {
        let mut batch_accumulators = Vec::new();

        // Process file in batches sequentially
        self.process_in_batches(input_path, |batch| {
            let mut local_aggregator = A::default();
            let accumulator = local_aggregator.process_batch(batch)?;
            batch_accumulators.push(accumulator);
            Ok(())
        })?;

        // Merge accumulators and finalize
        let mut final_aggregator = A::default();
        final_aggregator.merge_accumulators(batch_accumulators)?;
        final_aggregator.finalize()
    }

    /// Process a single file and run all transformations simultaneously using streaming batch processing
    pub fn run_all_transformations(
        &self,
        input_path: &str,
        output_dir: Option<&str>,
    ) -> Result<(), ProcessingError> {
        let start_time = Instant::now();
        let output_dir = output_dir.unwrap_or(".");

        println!(
            "Processing {} using streaming batches of {} records",
            input_path, self.chunk_size
        );
        println!("Running all transformations: peak_zones, payment_analysis, hourly_patterns");

        // Run single-pass transformation using MultiAnalyzer
        let results = self.run_streaming_transformation::<MultiAnalyzer, _>(input_path)?;

        let processing_time = start_time.elapsed();
        println!(
            "Streaming processing completed in: {:.2} seconds",
            processing_time.as_secs_f64()
        );

        // Write all three output files with thread count in filename
        let thread_count = rayon::current_num_threads();
        let peak_zones_json = serde_json::to_string_pretty(&results.peak_zones)?;
        let hourly_patterns_json = serde_json::to_string_pretty(&results.hourly_patterns)?;
        let payment_analysis_json = serde_json::to_string_pretty(&results.payment_analysis)?;

        std::fs::write(
            format!("{}/peak_zones_{}_cpus.json", output_dir, thread_count),
            peak_zones_json,
        )?;
        std::fs::write(
            format!("{}/hourly_patterns_{}_cpus.json", output_dir, thread_count),
            hourly_patterns_json,
        )?;
        std::fs::write(
            format!("{}/payment_analysis_{}_cpus.json", output_dir, thread_count),
            payment_analysis_json,
        )?;

        println!("Results saved to:");
        println!("  - {}/peak_zones_{}_cpus.json", output_dir, thread_count);
        println!(
            "  - {}/hourly_patterns_{}_cpus.json",
            output_dir, thread_count
        );
        println!(
            "  - {}/payment_analysis_{}_cpus.json",
            output_dir, thread_count
        );

        Ok(())
    }

    /// Process all CSV files in a directory and run all transformations simultaneously using streaming batch processing
    pub fn run_directory_all_transformations(
        &self,
        directory_path: &str,
        output_dir: &str,
    ) -> Result<(), ProcessingError> {
        let start_time = Instant::now();

        // Collect all CSV files in the directory
        let csv_files = self.get_csv_files(directory_path)?;

        if csv_files.is_empty() {
            return Err(ProcessingError::Processing {
                message: format!("No CSV files found in directory: {}", directory_path),
            });
        }

        println!("Found {} CSV files to process:", csv_files.len());
        for file in &csv_files {
            println!("  - {}", file);
        }

        println!(
            "Processing {} files in parallel using streaming batches of {} records each",
            csv_files.len(),
            self.chunk_size
        );

        // Run directory-wide streaming transformation using MultiAnalyzer
        let results =
            self.run_directory_streaming_transformation::<MultiAnalyzer, _>(&csv_files)?;

        let processing_time = start_time.elapsed();
        println!(
            "Directory parallel streaming processing completed in: {:.2} seconds",
            processing_time.as_secs_f64()
        );

        std::fs::create_dir_all(output_dir)?;

        // Write all three output files with thread count in filename
        let thread_count = rayon::current_num_threads();
        let peak_zones_json = serde_json::to_string_pretty(&results.peak_zones)?;
        let hourly_patterns_json = serde_json::to_string_pretty(&results.hourly_patterns)?;
        let payment_analysis_json = serde_json::to_string_pretty(&results.payment_analysis)?;

        std::fs::write(
            format!("{}/peak_zones_all_{}_cpus.json", output_dir, thread_count),
            peak_zones_json,
        )?;
        std::fs::write(
            format!(
                "{}/hourly_patterns_all_{}_cpus.json",
                output_dir, thread_count
            ),
            hourly_patterns_json,
        )?;
        std::fs::write(
            format!(
                "{}/payment_analysis_all_{}_cpus.json",
                output_dir, thread_count
            ),
            payment_analysis_json,
        )?;

        println!("Results saved to:");
        println!(
            "  - {}/peak_zones_all_{}_cpus.json",
            output_dir, thread_count
        );
        println!(
            "  - {}/hourly_patterns_all_{}_cpus.json",
            output_dir, thread_count
        );
        println!(
            "  - {}/payment_analysis_all_{}_cpus.json",
            output_dir, thread_count
        );

        Ok(())
    }

    /// Generic streaming transformation runner for multiple files with parallel processing
    fn run_directory_streaming_transformation<A, T>(
        &self,
        csv_files: &[String],
    ) -> Result<T, ProcessingError>
    where
        A: BatchAggregator<T> + Default + Send + Sync,
        A::Accumulator: Send,
        T: Send,
    {
        // Process files in parallel and collect accumulators from each file
        let file_accumulators: Result<Vec<Vec<A::Accumulator>>, ProcessingError> = csv_files
            .par_iter()
            .map(|file_path| {
                println!("Processing file: {}", file_path);

                let mut batch_accumulators = Vec::new();

                // Process this file in batches using streaming approach
                self.process_in_batches(file_path, |batch| {
                    let mut local_aggregator = A::default();
                    let accumulator = local_aggregator.process_batch(batch)?;
                    batch_accumulators.push(accumulator);
                    Ok(())
                })?;

                Ok(batch_accumulators)
            })
            .collect();

        // Flatten all accumulators from all files and batches
        let all_accumulators: Vec<A::Accumulator> =
            file_accumulators?.into_iter().flatten().collect();

        // Merge all accumulators and finalize
        let mut final_aggregator = A::default();
        final_aggregator.merge_accumulators(all_accumulators)?;
        final_aggregator.finalize()
    }

    /// Get all CSV files in a directory
    fn get_csv_files(&self, directory_path: &str) -> Result<Vec<String>, ProcessingError> {
        let mut csv_files = Vec::new();

        let dir = read_dir(directory_path)?;

        for entry in dir {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "csv" {
                        if let Some(path_str) = path.to_str() {
                            csv_files.push(path_str.to_string());
                        }
                    }
                }
            }
        }

        csv_files.sort(); // Process files in sorted order for consistency
        Ok(csv_files)
    }
}

impl Default for TaxiProcessor {
    fn default() -> Self {
        Self::new()
    }
}
