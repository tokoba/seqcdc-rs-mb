//! Core chunking implementation.

use crate::config::{ChunkingConfig, SeqOpMode};
use crate::error::Result;

/// Represents a single chunk of data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk<'a> {
    /// The chunk data
    pub data: &'a [u8],
    /// Starting position in the original data
    pub start: usize,
    /// Length of the chunk
    pub len: usize,
}

impl<'a> Chunk<'a> {
    /// Create a new chunk
    pub fn new(data: &'a [u8], start: usize, len: usize) -> Self {
        Self { data, start, len }
    }

    /// Get the end position of this chunk
    pub fn end(&self) -> usize {
        self.start + self.len
    }

    /// Check if this chunk is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Iterator over chunks produced by the chunking algorithm
pub struct ChunkIterator<'a> {
    data: &'a [u8],
    chunker: &'a SeqChunking,
    position: usize,
}

impl<'a> ChunkIterator<'a> {
    fn new(data: &'a [u8], chunker: &'a SeqChunking) -> Self {
        Self {
            data,
            chunker,
            position: 0,
        }
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.data.len() {
            return None;
        }

        let remaining = &self.data[self.position..];
        let cutpoint = self
            .chunker
            .find_cutpoint(remaining, remaining.len() as u64);
        let chunk_size = (cutpoint as usize).min(remaining.len());

        if chunk_size == 0 {
            return None;
        }

        let chunk_data = &remaining[..chunk_size];
        let chunk = Chunk::new(chunk_data, self.position, chunk_size);

        self.position += chunk_size;
        Some(chunk)
    }
}

/// Main chunking algorithm implementation
#[derive(Debug, Clone)]
pub struct SeqChunking {
    config: ChunkingConfig,
    technique_name: String,
}

impl SeqChunking {
    /// Create a new chunker with default configuration
    pub fn new() -> Self {
        Self::from_config(ChunkingConfig::new())
    }

    /// Create a new chunker with the given configuration
    pub fn from_config(config: ChunkingConfig) -> Self {
        Self {
            config,
            technique_name: "Seq Chunking".to_string(),
        }
    }

    /// Create a chunker with validation of the configuration
    pub fn try_from_config(config: ChunkingConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self::from_config(config))
    }

    /// Get the technique name
    pub fn technique_name(&self) -> &str {
        &self.technique_name
    }

    /// Get the minimum block size
    pub fn min_block_size(&self) -> u64 {
        self.config.min_block_size
    }

    /// Get the maximum block size
    pub fn max_block_size(&self) -> u64 {
        self.config.max_block_size
    }

    /// Get the current configuration
    pub fn config(&self) -> &ChunkingConfig {
        &self.config
    }

    /// Find the cutpoint for increasing sequences
    fn find_cutpoint_increasing(&self, buff: &[u8], size: u64) -> u64 {
        let mut curr_pos = self.config.min_block_size as usize;
        let mut opposing_slope_count: u64 = 0;
        let mut curr_seq_length: u64 = 0;
        let size_usize = size as usize;

        while curr_pos < size_usize && curr_pos < buff.len() && curr_pos > 0 {
            let cmp_result = buff[curr_pos] as i16 - buff[curr_pos - 1] as i16;

            // Low Entropy Absorption - skip equal bytes
            if cmp_result == 0 {
                curr_pos += 1;
                continue;
            }

            let cmp_sign = cmp_result < 0;

            if cmp_sign {
                opposing_slope_count += 1;
                curr_seq_length = 0;
            } else {
                curr_seq_length += 1;
            }

            if curr_seq_length >= self.config.seq_threshold {
                return curr_pos as u64;
            }

            if opposing_slope_count >= self.config.jump_trigger {
                curr_pos += self.config.jump_size as usize;
                opposing_slope_count = 0;
                curr_seq_length = 0;

                if curr_pos >= size_usize || curr_pos >= buff.len() {
                    break;
                }
            } else {
                curr_pos += 1;
            }
        }

        size
    }

    /// Find the cutpoint for decreasing sequences
    fn find_cutpoint_decreasing(&self, buff: &[u8], size: u64) -> u64 {
        let mut curr_pos = self.config.min_block_size as usize;
        let mut opposing_slope_count: u64 = 0;
        let mut curr_seq_length: u64 = 0;
        let size_usize = size as usize;

        while curr_pos < size_usize && curr_pos < buff.len() && curr_pos > 0 {
            let cmp_result = buff[curr_pos] as i16 - buff[curr_pos - 1] as i16;

            // Low Entropy Absorption - skip equal bytes
            if cmp_result == 0 {
                curr_pos += 1;
                continue;
            }

            let cmp_sign = cmp_result > 0;

            if cmp_sign {
                opposing_slope_count += 1;
                curr_seq_length = 0;
            } else {
                curr_seq_length += 1;
            }

            if curr_seq_length >= self.config.seq_threshold {
                return curr_pos as u64;
            }

            if opposing_slope_count >= self.config.jump_trigger {
                curr_pos += self.config.jump_size as usize;
                opposing_slope_count = 0;
                curr_seq_length = 0;

                if curr_pos >= size_usize || curr_pos >= buff.len() {
                    break;
                }
            } else {
                curr_pos += 1;
            }
        }

        size
    }

    /// Find the optimal cutpoint in the given buffer
    pub fn find_cutpoint(&self, buff: &[u8], size: u64) -> u64 {
        if size < self.config.min_block_size {
            return size;
        }

        let actual_size = size.min(self.config.max_block_size);

        match self.config.op_mode {
            SeqOpMode::Increasing => self.find_cutpoint_increasing(buff, actual_size),
            SeqOpMode::Decreasing => self.find_cutpoint_decreasing(buff, actual_size),
        }
    }

    /// Create an iterator over all chunks in the given data
    pub fn chunk_all<'a>(&'a self, data: &'a [u8]) -> ChunkIterator<'a> {
        ChunkIterator::new(data, self)
    }

    /// Chunk the data and collect all chunks into a Vec
    pub fn chunk_all_vec<'a>(&'a self, data: &'a [u8]) -> Vec<Chunk<'a>> {
        self.chunk_all(data).collect()
    }

    /// Get the first chunk from the data
    pub fn chunk_first<'a>(&'a self, data: &'a [u8]) -> Option<Chunk<'a>> {
        self.chunk_all(data).next()
    }

    /// Calculate chunking statistics for the given data
    pub fn stats(&self, data: &[u8]) -> ChunkingStats {
        let chunks: Vec<_> = self.chunk_all(data).collect();
        ChunkingStats::from_chunks(&chunks, data.len())
    }
}

impl Default for SeqChunking {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about chunking results
#[derive(Debug, Clone)]
pub struct ChunkingStats {
    /// Total number of chunks
    pub chunk_count: usize,
    /// Total size of data processed
    pub total_size: usize,
    /// Average chunk size
    pub avg_chunk_size: f64,
    /// Minimum chunk size
    pub min_chunk_size: usize,
    /// Maximum chunk size
    pub max_chunk_size: usize,
    /// Standard deviation of chunk sizes
    pub chunk_size_stddev: f64,
}

impl ChunkingStats {
    /// Create statistics from a collection of chunks
    pub fn from_chunks(chunks: &[Chunk<'_>], total_size: usize) -> Self {
        if chunks.is_empty() {
            return Self {
                chunk_count: 0,
                total_size,
                avg_chunk_size: 0.0,
                min_chunk_size: 0,
                max_chunk_size: 0,
                chunk_size_stddev: 0.0,
            };
        }

        let chunk_sizes: Vec<usize> = chunks.iter().map(|c| c.len).collect();
        let chunk_count = chunks.len();
        let sum: usize = chunk_sizes.iter().sum();
        let avg = sum as f64 / chunk_count as f64;

        let min_size = chunk_sizes.iter().min().copied().unwrap_or(0);
        let max_size = chunk_sizes.iter().max().copied().unwrap_or(0);

        // Calculate standard deviation
        let variance: f64 = chunk_sizes
            .iter()
            .map(|&size| {
                let diff = size as f64 - avg;
                diff * diff
            })
            .sum::<f64>()
            / chunk_count as f64;

        let stddev = variance.sqrt();

        Self {
            chunk_count,
            total_size,
            avg_chunk_size: avg,
            min_chunk_size: min_size,
            max_chunk_size: max_size,
            chunk_size_stddev: stddev,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SeqOpMode;

    #[test]
    fn test_seq_chunking_new() {
        let chunker = SeqChunking::new();
        assert_eq!(chunker.technique_name(), "Seq Chunking");
        assert_eq!(chunker.config().seq_threshold, 5);
    }

    #[test]
    fn test_seq_chunking_from_config() {
        let config = ChunkingConfig::builder().seq_threshold(10).build().unwrap();
        let chunker = SeqChunking::from_config(config);
        assert_eq!(chunker.config().seq_threshold, 10);
    }

    #[test]
    fn test_find_cutpoint_small_size() {
        let chunker = SeqChunking::new();
        let data = vec![1, 2, 3, 4, 5];
        let result = chunker.find_cutpoint(&data, 512);
        assert_eq!(result, 512);
    }

    #[test]
    fn test_find_cutpoint_increasing() {
        let chunker = SeqChunking::new();
        let mut data = vec![0u8; 8192];

        for i in 4096..4110 {
            data[i] = (i - 4096) as u8;
        }

        let result = chunker.find_cutpoint(&data, 8192);
        assert!(result < 8192);
        assert!(result > 4096);
    }

    #[test]
    fn test_chunk_iterator() {
        let chunker = SeqChunking::new();
        let data = b"Hello, World! This is a test.";

        let chunks: Vec<_> = chunker.chunk_all(data).collect();

        assert!(!chunks.is_empty());

        // Verify reconstruction
        let reconstructed: Vec<u8> = chunks
            .iter()
            .flat_map(|chunk| chunk.data.iter())
            .copied()
            .collect();

        assert_eq!(reconstructed, data);
    }

    #[test]
    fn test_chunking_stats() {
        let chunker = SeqChunking::new();
        let data = vec![42u8; 10000];

        let stats = chunker.stats(&data);
        assert_eq!(stats.total_size, 10000);
        assert!(stats.chunk_count > 0);
        assert!(stats.avg_chunk_size > 0.0);
    }

    #[test]
    fn test_decreasing_mode() {
        let config = ChunkingConfig::builder()
            .op_mode(SeqOpMode::Decreasing)
            .build()
            .unwrap();
        let chunker = SeqChunking::from_config(config);

        let mut data = vec![255u8; 8192];
        for i in 4096..4110 {
            data[i] = (255 - (i - 4096)) as u8;
        }

        let result = chunker.find_cutpoint(&data, 8192);
        assert!(result < 8192);
        assert!(result > 4096);
    }

    #[test]
    fn test_chunk_properties() {
        let data = b"test data";
        let chunk = Chunk::new(data, 0, data.len());

        assert_eq!(chunk.start, 0);
        assert_eq!(chunk.len, data.len());
        assert_eq!(chunk.end(), data.len());
        assert!(!chunk.is_empty());
        assert_eq!(chunk.data, data);
    }
}
