//! Basic usage example for the seq-chunking library
//! 
//! This example demonstrates how to use the library to chunk data
//! and validates the results.

use seq_chunking::{
    SeqChunking, ChunkingConfig, SeqOpMode,
    utils::{FileUtils, ValidationUtils, TestDataGenerator, PerfUtils}
};
use std::time::Instant;

fn main() -> seq_chunking::Result<()> {
    println!("=== SeqChunking Library Example ===\n");

    // Example 1: Basic usage with default configuration
    example_basic_usage()?;
    
    // Example 2: Custom configuration
    example_custom_config()?;
    
    // Example 3: File processing
    example_file_processing()?;
    
    // Example 4: Performance measurement
    example_performance_measurement()?;
    
    // Example 5: Different operation modes
    example_operation_modes()?;

    println!("\n=== All examples completed successfully! ===");
    Ok(())
}

fn example_basic_usage() -> seq_chunking::Result<()> {
    println!("--- Example 1: Basic Usage ---");
    
    // Create a chunker with default settings
    let chunker = SeqChunking::new();
    
    // Some test data
    let data = b"Hello, World! This is a test of the chunking library with some data patterns.";
    
    println!("Original data size: {} bytes", data.len());
    println!("Chunker: {}", chunker.technique_name());
    println!("Min block size: {} bytes", chunker.min_block_size());
    println!("Max block size: {} bytes", chunker.max_block_size());
    
    // Chunk the data
    let chunks: Vec<_> = chunker.chunk_all(data).collect();
    
    println!("Number of chunks: {}", chunks.len());
    
    // Display first few chunks
    for (i, chunk) in chunks.iter().enumerate().take(5) {
        println!("  Chunk {}: {} bytes (pos {}-{})", 
                 i + 1, chunk.len, chunk.start, chunk.end());
    }
    
    // Verify the chunks can reconstruct the original data
    let is_valid = ValidationUtils::verify_chunks(data, &chunks)?;
    println!("Chunks valid: {}\n", if is_valid { "✓ YES" } else { "✗ NO" });
    
    Ok(())
}

fn example_custom_config() -> seq_chunking::Result<()> {
    println!("--- Example 2: Custom Configuration ---");
    
    // Create a custom configuration
    let config = ChunkingConfig::builder()
        .seq_threshold(10)           // Longer sequences needed
        .min_block_size(2048)        // Larger minimum blocks
        .max_block_size(32768)       // Larger maximum blocks
        .jump_trigger(100)           // More opposing slopes before jumping
        .op_mode(SeqOpMode::Decreasing) // Look for decreasing sequences
        .build()?;
    
    let chunker = SeqChunking::from_config(config);
    
    // Generate test data with decreasing sequences
    let data = TestDataGenerator::generate_decreasing_sequences(50000, 15, 10);
    
    println!("Generated data size: {} bytes", data.len());
    println!("Operation mode: {:?}", chunker.config().op_mode());
    
    // Chunk the data and get statistics
    let stats = chunker.stats(&data);
    
    println!("Chunking statistics:");
    println!("  - Total chunks: {}", stats.chunk_count);
    println!("  - Average chunk size: {:.1} bytes", stats.avg_chunk_size);
    println!("  - Min chunk size: {} bytes", stats.min_chunk_size);
    println!("  - Max chunk size: {} bytes", stats.max_chunk_size);
    println!("  - Std deviation: {:.1} bytes\n", stats.chunk_size_stddev);
    
    Ok(())
}

fn example_file_processing() -> seq_chunking::Result<()> {
    println!("--- Example 3: File Processing ---");
    
    // Create test data and write to a temporary file
    let test_data = TestDataGenerator::generate_mixed_patterns(100000);
    let input_file = "example_input.dat";
    let output_file = "example_output.dat";
    
    println!("Writing test data to file...");
    FileUtils::write_file(input_file, &test_data)?;
    
    // Read and chunk the file
    println!("Reading and chunking file...");
    let file_data = FileUtils::read_file(input_file)?;
    
    let chunker = SeqChunking::new();
    let chunks: Vec<_> = chunker.chunk_all(&file_data).collect();
    
    // Write chunks back to another file
    println!("Writing chunks to output file...");
    FileUtils::write_chunks_to_file(output_file, &chunks)?;
    
    // Verify the files are identical
    let output_data = FileUtils::read_file(output_file)?;
    let files_identical = file_data == output_data;
    
    println!("Input file size: {} bytes", file_data.len());
    println!("Output file size: {} bytes", output_data.len());
    println!("Files identical: {}", if files_identical { "✓ YES" } else { "✗ NO" });
    
    // Clean up
    let _ = std::fs::remove_file(input_file);
    let _ = std::fs::remove_file(output_file);
    
    println!();
    Ok(())
}

fn example_performance_measurement() -> seq_chunking::Result<()> {
    println!("--- Example 4: Performance Measurement ---");
    
    let chunker = SeqChunking::new();
    let data = TestDataGenerator::generate_pseudo_random(1_000_000, 12345); // 1MB of data
    
    println!("Data size: {} bytes (1 MB)", data.len());
    
    // Measure chunking performance
    let (chunks, duration) = PerfUtils::measure_time(|| {
        chunker.chunk_all_vec(&data)
    });
    
    let throughput_mb_s = PerfUtils::calculate_throughput_mb_s(data.len(), duration);
    let throughput_bytes_s = PerfUtils::calculate_throughput_bytes_s(data.len(), duration);
    
    println!("Chunking completed in: {:?}", duration);
    println!("Throughput: {:.2} MB/s", throughput_mb_s);
    println!("Throughput: {:.0} bytes/s", throughput_bytes_s);
    println!("Number of chunks: {}", chunks.len());
    
    // Verify integrity
    let is_valid = ValidationUtils::verify_chunks(&data, &chunks)?;
    println!("Data integrity: {}\n", if is_valid { "✓ PASSED" } else { "✗ FAILED" });
    
    Ok(())
}

fn example_operation_modes() -> seq_chunking::Result<()> {
    println!("--- Example 5: Operation Modes Comparison ---");
    
    // Create test data with both increasing and decreasing patterns
    let mut data = Vec::new();
    
    // Add increasing sequences
    for i in 0..100 {
        data.push(i as u8);
    }
    
    // Add some random data
    for i in 100..200 {
        data.push(((i * 7) % 256) as u8);
    }
    
    // Add decreasing sequences
    for i in 0..100 {
        data.push((255 - i) as u8);
    }
    
    println!("Test data size: {} bytes", data.len());
    
    // Test with increasing mode
    let config_inc = ChunkingConfig::builder()
        .op_mode(SeqOpMode::Increasing)
        .seq_threshold(5)
        .build()?;
    let chunker_inc = SeqChunking::from_config(config_inc);
    let chunks_inc: Vec<_> = chunker_inc.chunk_all(&data).collect();
    
    // Test with decreasing mode
    let config_dec = ChunkingConfig::builder()
        .op_mode(SeqOpMode::Decreasing)
        .seq_threshold(5)
        .build()?;
    let chunker_dec = SeqChunking::from_config(config_dec);
    let chunks_dec: Vec<_> = chunker_dec.chunk_all(&data).collect();
    
    println!("Increasing mode: {} chunks", chunks_inc.len());
    println!("Decreasing mode: {} chunks", chunks_dec.len());
    
    // Show first few cutpoints for each mode
    println!("\nIncreasing mode cutpoints:");
    for (i, chunk) in chunks_inc.iter().enumerate().take(5) {
        println!("  Chunk {}: {} bytes at position {}", i + 1, chunk.len, chunk.start);
    }
    
    println!("\nDecreasing mode cutpoints:");
    for (i, chunk) in chunks_dec.iter().enumerate().take(5) {
        println!("  Chunk {}: {} bytes at position {}", i + 1, chunk.len, chunk.start);
    }
    
    // Verify both modes preserve data integrity
    let valid_inc = ValidationUtils::verify_chunks(&data, &chunks_inc)?;
    let valid_dec = ValidationUtils::verify_chunks(&data, &chunks_dec)?;
    
    println!("\nData integrity:");
    println!("  Increasing mode: {}", if valid_inc { "✓ PASSED" } else { "✗ FAILED" });
    println!("  Decreasing mode: {}", if valid_dec { "✓ PASSED" } else { "✗ FAILED" });
    
    Ok(())
}