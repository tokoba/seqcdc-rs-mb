//! Configuration module for sequence-based chunking.

use crate::error::{ChunkingError, Result};
use crate::*;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum SeqOpMode {
    /// Detect increasing byte sequences
    #[default]
    Increasing,
    /// Detect decreasing byte sequences
    Decreasing,
}

/// Configuration for the chunking algorithm
#[derive(Debug, Clone)]
pub struct ChunkingConfig {
    /// Number of consecutive sequence bytes needed to trigger a cut
    pub seq_threshold: u64,
    /// Number of opposing slopes before jumping ahead
    pub jump_trigger: u64,
    /// Number of bytes to jump when trigger is hit
    pub jump_size: u64,
    /// Sequence detection mode (increasing or decreasing)
    pub op_mode: SeqOpMode,
    /// Minimum chunk size in bytes
    pub min_block_size: u64,
    /// Target average chunk size in bytes
    pub avg_block_size: u64,
    /// Maximum chunk size in bytes
    pub max_block_size: u64,
}

impl ChunkingConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a builder for configuring chunking parameters
    pub fn builder() -> ChunkingConfigBuilder {
        ChunkingConfigBuilder::new()
    }

    /// Validate the configuration parameters
    pub fn validate(&self) -> Result<()> {
        if self.seq_threshold == 0 {
            return Err(ChunkingError::InvalidConfig("seq_threshold must be greater than 0".into()));
        }
        
        if self.min_block_size == 0 {
            return Err(ChunkingError::InvalidConfig("min_block_size must be greater than 0".into()));
        }
        
        if self.max_block_size < self.min_block_size {
            return Err(ChunkingError::InvalidConfig("max_block_size must be >= min_block_size".into()));
        }
        
        if self.jump_size == 0 {
            return Err(ChunkingError::InvalidConfig("jump_size must be greater than 0".into()));
        }
        
        Ok(())
    }

    // Getters
    pub fn seq_threshold(&self) -> u64 { self.seq_threshold }
    pub fn jump_trigger(&self) -> u64 { self.jump_trigger }
    pub fn jump_size(&self) -> u64 { self.jump_size }
    pub fn op_mode(&self) -> SeqOpMode { self.op_mode }
    pub fn min_block_size(&self) -> u64 { self.min_block_size }
    pub fn avg_block_size(&self) -> u64 { self.avg_block_size }
    pub fn max_block_size(&self) -> u64 { self.max_block_size }
}

impl Default for ChunkingConfig {
    fn default() -> Self {
        Self {
            seq_threshold: DEFAULT_SEQ_THRESHOLD,
            jump_trigger: DEFAULT_JUMP_TRIGGER,
            jump_size: DEFAULT_JUMP_SIZE,
            op_mode: SeqOpMode::default(),
            min_block_size: DEFAULT_MIN_BLOCK_SIZE,
            avg_block_size: DEFAULT_AVG_BLOCK_SIZE,
            max_block_size: DEFAULT_MAX_BLOCK_SIZE,
        }
    }
}

/// Builder pattern for ChunkingConfig
#[derive(Debug)]
pub struct ChunkingConfigBuilder {
    config: ChunkingConfig,
}

impl ChunkingConfigBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            config: ChunkingConfig::default(),
        }
    }

    /// Set the sequence threshold
    pub fn seq_threshold(mut self, threshold: u64) -> Self {
        self.config.seq_threshold = threshold;
        self
    }

    /// Set the jump trigger count
    pub fn jump_trigger(mut self, trigger: u64) -> Self {
        self.config.jump_trigger = trigger;
        self
    }

    /// Set the jump size
    pub fn jump_size(mut self, size: u64) -> Self {
        self.config.jump_size = size;
        self
    }

    /// Set the operation mode
    pub fn op_mode(mut self, mode: SeqOpMode) -> Self {
        self.config.op_mode = mode;
        self
    }

    /// Set the minimum block size
    pub fn min_block_size(mut self, size: u64) -> Self {
        self.config.min_block_size = size;
        self
    }

    /// Set the average block size
    pub fn avg_block_size(mut self, size: u64) -> Self {
        self.config.avg_block_size = size;
        self
    }

    /// Set the maximum block size
    pub fn max_block_size(mut self, size: u64) -> Self {
        self.config.max_block_size = size;
        self
    }

    /// Build the configuration, validating parameters
    pub fn build(self) -> Result<ChunkingConfig> {
        self.config.validate()?;
        Ok(self.config)
    }

    /// Build the configuration without validation
    pub fn build_unchecked(self) -> ChunkingConfig {
        self.config
    }
}

impl Default for ChunkingConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ChunkingConfig::new();
        assert_eq!(config.seq_threshold(), DEFAULT_SEQ_THRESHOLD);
        assert_eq!(config.op_mode(), SeqOpMode::Increasing);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_builder_pattern() {
        let config = ChunkingConfig::builder()
            .seq_threshold(10)
            .min_block_size(2048)
            .max_block_size(32768)
            .op_mode(SeqOpMode::Decreasing)
            .build()
            .unwrap();

        assert_eq!(config.seq_threshold(), 10);
        assert_eq!(config.min_block_size(), 2048);
        assert_eq!(config.max_block_size(), 32768);
        assert_eq!(config.op_mode(), SeqOpMode::Decreasing);
    }

    #[test]
    fn test_invalid_config() {
        let result = ChunkingConfig::builder()
            .seq_threshold(0)
            .build();
        
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_block_sizes() {
        let result = ChunkingConfig::builder()
            .min_block_size(8192)
            .max_block_size(4096) // max < min
            .build();
        
        assert!(result.is_err());
    }
}