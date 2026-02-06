#!/bin/bash

THREADS=""
BATCH_SIZE=""
BENCHMARK_MODE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -j|--threads) THREADS="-j $2"; shift; shift ;;
        -b|--batch-size) BATCH_SIZE="-b $2"; shift; shift ;;
        --benchmark) BENCHMARK_MODE=true; shift ;;
        -h|--help)
            echo "Usage: $0 [-j threads] [-b batch_size] [--benchmark]"
            exit 0 ;;
        *) echo "Unknown option $1"; exit 1 ;;
    esac
done

mkdir -p output

if [ "$BENCHMARK_MODE" = true ]; then
    mkdir -p output/benchmark
    CPU_COUNTS=(1 2 4 8)
    
    cargo build --release
    
    BENCHMARK_FILE="output/benchmark/benchmark_results.txt"
    echo "Performance Benchmark Results" > "$BENCHMARK_FILE"
    echo "===============================" >> "$BENCHMARK_FILE"
    echo "" >> "$BENCHMARK_FILE"
    printf "%-8s %-12s %-15s\n" "CPUs" "Time (sec)" "Peak Memory (MB)" >> "$BENCHMARK_FILE"
    echo "----------------------------------------" >> "$BENCHMARK_FILE"
        
    for cpu_count in "${CPU_COUNTS[@]}"; do
        TIME_OUTPUT=$(mktemp)
        
        /usr/bin/time -v ./target/release/nyc-taxi-processor -j $cpu_count -b 50000 batch-process -d data -o "output/benchmark" 2> "$TIME_OUTPUT"
        
        elapsed_time=$(grep "Elapsed (wall clock) time" "$TIME_OUTPUT" | awk '{print $NF}')
        duration_seconds=$(echo "$elapsed_time" | awk -F: '{
            if (NF == 3) {
                print ($1 * 3600) + ($2 * 60) + $3
            } else if (NF == 2) {
                print ($1 * 60) + $2
            } else {
                print $1
            }
        }')
        memory_kb=$(grep "Maximum resident set size" "$TIME_OUTPUT" | awk '{print $NF}')
        memory_mb=$(echo "scale=2; $memory_kb / 1024" | bc -l)
        
        printf "%-8s %-12s %-15s\n" "$cpu_count" "$duration_seconds" "$memory_mb" >> "$BENCHMARK_FILE"
        
        rm "$TIME_OUTPUT"
    done
    
else
    cargo build --release
    CLI_OPTIONS="$THREADS $BATCH_SIZE"
    
    start_time=$(date +%s.%N)
    ./target/release/nyc-taxi-processor $CLI_OPTIONS batch-process -d data -o output
    end_time=$(date +%s.%N)
    duration_seconds=$(echo "$end_time - $start_time" | bc -l)
    
    printf "Analysis completed in %.2f seconds\n" "$duration_seconds"
fi