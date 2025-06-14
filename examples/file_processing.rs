//! File processing example for the seq-chunking library

use seq_chunking::{
    SeqChunking,
    utils::{FileUtils, TestDataGenerator}
};

fn main() -> seq_chunking::Result<()> {
    println!("=== SeqChunking File Processing Example ===\n");

    // Example 3: File processing
    example_file_processing()?;

    println!("\n=== Example completed successfully! ===");
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