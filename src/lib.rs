//! # SeqChunking
//!
//! A Rust library for sequence-based data chunking using slope detection algorithms.
//!
//! This library provides efficient algorithms for dividing data streams into chunks
//! based on byte sequence patterns (increasing or decreasing slopes). It's particularly
//! useful for content-defined chunking applications.
//!
//! ## Quick Start
//!
//! ```rust
//! use seq_chunking::{SeqChunking, ChunkingConfig};
//!
//! // Create a chunker with default settings
//! let chunker = SeqChunking::new();
//!
//! // Or with custom configuration
//! let config = ChunkingConfig::builder()
//!     .seq_threshold(10)
//!     .min_block_size(2048)
//!     .build().expect("Failed to build ChunkingConfig");
//! let chunker = SeqChunking::from_config(config);
//!
//! // Chunk some data
//! let data = b"your data here";
//! let chunks: Vec<_> = chunker.chunk_all(data).collect();
//! ```

pub mod config;
pub mod chunker;
pub mod error;
pub mod utils;

pub use config::{ChunkingConfig, SeqOpMode};
pub use chunker::{SeqChunking, Chunk, ChunkIterator};
pub use error::{ChunkingError, Result};

/// Default sequence length threshold
pub const DEFAULT_SEQ_THRESHOLD: u64 = 5;

/// Default jump trigger count
pub const DEFAULT_JUMP_TRIGGER: u64 = 50;

/// Default jump size when trigger is hit
pub const DEFAULT_JUMP_SIZE: u64 = 256;

/// Default minimum block size
pub const DEFAULT_MIN_BLOCK_SIZE: u64 = 4096;

/// Default average block size
pub const DEFAULT_AVG_BLOCK_SIZE: u64 = 8192;

/// Default maximum block size
pub const DEFAULT_MAX_BLOCK_SIZE: u64 = 16384;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_basic_usage() {
        let chunker = SeqChunking::new();
        let data = b"Hello, World! This is a test of the chunking library.";
        
        let chunks: Vec<_> = chunker.chunk_all(data).collect();
        
        // Verify that we got at least one chunk
        assert!(!chunks.is_empty());
        
        // Verify that all chunks combined equal the original data
        let reconstructed: Vec<u8> = chunks.iter()
            .flat_map(|chunk| chunk.data.iter())
            .copied()
            .collect();
        
        assert_eq!(reconstructed, data);
    }
}