use clap::{Parser, Subcommand};
use nyc_taxi_processor::error::ProcessingError;
use nyc_taxi_processor::*;
use rayon::ThreadPoolBuilder;

#[derive(Parser)]
#[command(name = "nyc-taxi-processor")]
#[command(about = "NYC Taxi data processor - runs all transformations automatically")]
struct Cli {
    /// Number of CPU threads to use for parallel processing
    #[arg(short = 'j', long)]
    threads: Option<usize>,

    /// Batch size
    #[arg(short, long, default_value_t = 10000)]
    batch_size: usize,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a single CSV file (runs all transformations)
    Process {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output_dir: Option<String>,
    },
    /// Process all CSV files in a directory (runs all transformations)
    BatchProcess {
        #[arg(short, long)]
        directory: String,

        #[arg(short, long)]
        output_dir: String,
    },
}

fn main() -> Result<(), ProcessingError> {
    let cli = Cli::parse();

    let max_threads = num_cpus::get();

    // Configure CPU threads
    if let Some(num_threads) = cli.threads {
        let threads = num_threads.min(max_threads);

        ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .expect("Failed to build thread pool");

        println!(
            "Using {} threads for parallel processing (max available: {})",
            threads, max_threads
        );
    } else {
        // Print default Rayon thread count
        let default_threads = rayon::current_num_threads();
        println!(
            "Using {} threads for parallel processing (default, max available: {})",
            default_threads, max_threads
        );
    }

    let processor = TaxiProcessor::with_chunk_size(cli.batch_size);

    match cli.command {
        Commands::Process { input, output_dir } => {
            println!(
                "Processing {} with batch size of {} records",
                input, cli.batch_size
            );
            println!("Running all transformations: peak_zones, payment_analysis, hourly_patterns");
            processor.run_all_transformations(&input, output_dir.as_deref())?;
        }
        Commands::BatchProcess {
            directory,
            output_dir,
        } => {
            println!(
                "Processing all CSV files in {} with batch size of {} records",
                directory, cli.batch_size
            );
            println!("Running all transformations: peak_zones, payment_analysis, hourly_patterns");
            processor.run_directory_all_transformations(&directory, &output_dir)?;
        }
    }

    Ok(())
}
