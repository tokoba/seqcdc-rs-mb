//! Utility functions for the chunking library.

use crate::{Chunk, ChunkingError, Result};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

/// Utility functions for file operations
pub struct FileUtils;

impl FileUtils {
    /// Read a file and return its contents as a Vec<u8>
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
        let mut file = File::open(path.as_ref())
            .map_err(|e| ChunkingError::io_error(format!("Failed to open file: {}", e)))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| ChunkingError::io_error(format!("Failed to read file: {}", e)))?;

        Ok(buffer)
    }

    /// Write data to a file
    pub fn write_file<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<()> {
        let mut file = File::create(path.as_ref())
            .map_err(|e| ChunkingError::io_error(format!("Failed to create file: {}", e)))?;

        file.write_all(data)
            .map_err(|e| ChunkingError::io_error(format!("Failed to write file: {}", e)))?;

        file.flush()
            .map_err(|e| ChunkingError::io_error(format!("Failed to flush file: {}", e)))?;

        Ok(())
    }

    /// Write chunks to a file, reconstructing the original data
    pub fn write_chunks_to_file<P: AsRef<Path>>(path: P, chunks: &[Chunk<'_>]) -> Result<()> {
        let mut file = BufWriter::new(
            File::create(path.as_ref())
                .map_err(|e| ChunkingError::io_error(format!("Failed to create file: {}", e)))?,
        );

        for chunk in chunks {
            file.write_all(chunk.data)
                .map_err(|e| ChunkingError::io_error(format!("Failed to write chunk: {}", e)))?;
        }

        file.flush()
            .map_err(|e| ChunkingError::io_error(format!("Failed to flush file: {}", e)))?;

        Ok(())
    }

    /// Read a file with buffered I/O for better performance on large files
    pub fn read_file_buffered<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
        let file = File::open(path.as_ref())
            .map_err(|e| ChunkingError::io_error(format!("Failed to open file: {}", e)))?;

        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .map_err(|e| ChunkingError::io_error(format!("Failed to read file: {}", e)))?;

        Ok(buffer)
    }
}

/// Utility functions for data validation and verification
pub struct ValidationUtils;

impl ValidationUtils {
    /// Verify that chunks can be reconstructed to match original data
    pub fn verify_chunks(original: &[u8], chunks: &[Chunk<'_>]) -> Result<bool> {
        let reconstructed: Vec<u8> = chunks
            .iter()
            .flat_map(|chunk| chunk.data.iter())
            .copied()
            .collect();

        Ok(reconstructed == original)
    }

    /// Check if chunks are contiguous and cover the entire data
    pub fn validate_chunk_coverage(data_len: usize, chunks: &[Chunk<'_>]) -> Result<()> {
        if chunks.is_empty() {
            if data_len == 0 {
                return Ok(());
            } else {
                return Err(ChunkingError::processing_error(
                    "No chunks found for non-empty data",
                ));
            }
        }

        let mut expected_start = 0;

        for (i, chunk) in chunks.iter().enumerate() {
            if chunk.start != expected_start {
                return Err(ChunkingError::processing_error(format!(
                    "Chunk {} starts at {} but expected {}",
                    i, chunk.start, expected_start
                )));
            }

            if chunk.is_empty() {
                return Err(ChunkingError::processing_error(format!(
                    "Chunk {} is empty",
                    i
                )));
            }

            expected_start = chunk.end();
        }

        if expected_start != data_len {
            return Err(ChunkingError::processing_error(format!(
                "Chunks end at {} but data length is {}",
                expected_start, data_len
            )));
        }

        Ok(())
    }
}

/// Utility functions for generating test data
pub struct TestDataGenerator;

impl TestDataGenerator {
    /// Generate test data with increasing sequences
    pub fn generate_increasing_sequences(
        size: usize,
        seq_length: usize,
        seq_count: usize,
    ) -> Vec<u8> {
        let mut data = vec![0u8; size];
        let seq_spacing = size / seq_count.max(1);

        for seq_idx in 0..seq_count {
            let start_pos = seq_idx * seq_spacing;
            let end_pos = (start_pos + seq_length).min(size);

            for (i, pos) in (start_pos..end_pos).enumerate() {
                if pos < data.len() {
                    data[pos] = (i % 256) as u8;
                }
            }
        }

        data
    }

    /// Generate test data with decreasing sequences
    pub fn generate_decreasing_sequences(
        size: usize,
        seq_length: usize,
        seq_count: usize,
    ) -> Vec<u8> {
        let mut data = vec![255u8; size];
        let seq_spacing = size / seq_count.max(1);

        for seq_idx in 0..seq_count {
            let start_pos = seq_idx * seq_spacing;
            let end_pos = (start_pos + seq_length).min(size);

            for (i, pos) in (start_pos..end_pos).enumerate() {
                if pos < data.len() {
                    data[pos] = (255 - (i % 256)) as u8;
                }
            }
        }

        data
    }

    /// Generate mixed pattern test data
    pub fn generate_mixed_patterns(size: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);

        for i in 0..size {
            let value = match i % 10 {
                0..=4 => (i % 256) as u8,         // Increasing sequence
                5..=7 => (255 - (i % 256)) as u8, // Decreasing sequence
                _ => ((i * 7) % 256) as u8,       // Pseudo-random pattern
            };
            data.push(value);
        }

        data
    }

    /// Generate random-like data using a simple PRNG
    pub fn generate_pseudo_random(size: usize, seed: u64) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);
        let mut state = seed;

        for _ in 0..size {
            // Simple linear congruential generator
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            data.push((state >> 16) as u8);
        }

        data
    }
}

/// Performance measurement utilities
pub struct PerfUtils;

impl PerfUtils {
    /// Measure the time taken to execute a closure
    pub fn measure_time<F, R>(f: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Calculate throughput in MB/s
    pub fn calculate_throughput_mb_s(bytes: usize, duration: std::time::Duration) -> f64 {
        if duration.as_secs_f64() == 0.0 {
            return 0.0;
        }
        (bytes as f64) / (duration.as_secs_f64() * 1_000_000.0)
    }

    /// Calculate throughput in bytes/s
    pub fn calculate_throughput_bytes_s(bytes: usize, duration: std::time::Duration) -> f64 {
        if duration.as_secs_f64() == 0.0 {
            return 0.0;
        }
        (bytes as f64) / duration.as_secs_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SeqChunking;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_read_write() {
        let test_data = b"Hello, World! This is test data.";

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(test_data).unwrap();
        temp_file.flush().unwrap();

        let read_data = FileUtils::read_file(temp_file.path()).unwrap();
        assert_eq!(read_data, test_data);
    }

    #[test]
    fn test_chunk_verification() {
        let chunker = SeqChunking::new();
        let data = b"Test data for verification";

        let chunks: Vec<_> = chunker.chunk_all(data).collect();
        let is_valid = ValidationUtils::verify_chunks(data, &chunks).unwrap();

        assert!(is_valid);
    }

    #[test]
    fn test_chunk_coverage_validation() {
        let chunker = SeqChunking::new();
        let data = b"Test data for coverage validation";

        let chunks: Vec<_> = chunker.chunk_all(data).collect();
        ValidationUtils::validate_chunk_coverage(data.len(), &chunks).unwrap();
    }

    #[test]
    fn test_test_data_generation() {
        let data = TestDataGenerator::generate_increasing_sequences(1000, 10, 5);
        assert_eq!(data.len(), 1000);

        let data = TestDataGenerator::generate_decreasing_sequences(1000, 10, 5);
        assert_eq!(data.len(), 1000);

        let data = TestDataGenerator::generate_mixed_patterns(1000);
        assert_eq!(data.len(), 1000);

        let data = TestDataGenerator::generate_pseudo_random(1000, 42);
        assert_eq!(data.len(), 1000);
    }

    #[test]
    fn test_performance_utils() {
        let (result, duration) = PerfUtils::measure_time(|| {
            std::thread::sleep(std::time::Duration::from_millis(1));
            42
        });

        assert_eq!(result, 42);
        assert!(duration >= std::time::Duration::from_millis(1));

        let throughput =
            PerfUtils::calculate_throughput_mb_s(1_000_000, std::time::Duration::from_secs(1));
        assert_eq!(throughput, 1.0);
    }

    #[test]
    fn test_write_chunks_to_file() {
        let chunker = SeqChunking::new();
        let original_data = b"Test data for chunk file writing";
        let chunks: Vec<_> = chunker.chunk_all(original_data).collect();

        let temp_file = NamedTempFile::new().unwrap();
        FileUtils::write_chunks_to_file(temp_file.path(), &chunks).unwrap();

        let reconstructed_data = FileUtils::read_file(temp_file.path()).unwrap();
        assert_eq!(reconstructed_data, original_data);
    }
}
