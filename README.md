# SeqChunking

[![Crates.io](https://img.shields.io/crates/v/seq-chunking.svg)](https://crates.io/crates/seq-chunking)
[![docs.rs](https://docs.rs/seq-chunking/badge.svg)](https://docs.rs/seq-chunking)
[![License](https://img.shields.io/crates/l/seq-chunking.svg)](https://github.com/puntakana/seqcdc-rs#license)
![Test](https://github.com/puntakana/seqcdc-rs/workflows/Test/badge.svg)

A Rust library for sequence-based data chunking using slope detection algorithms.

This library provides efficient algorithms for dividing data streams into chunks based on byte sequence patterns (increasing or decreasing slopes). It's particularly useful for content-defined chunking applications, data deduplication, and stream processing.

## Features

- **Sequence-based chunking**: Detects increasing or decreasing byte sequences to determine chunk boundaries
- **Configurable parameters**: Customizable sequence thresholds, block sizes, and jump parameters
- **Multiple operation modes**: Support for both increasing and decreasing sequence detection
- **High performance**: Efficient algorithms with jump-ahead optimization for better performance
- **Iterator interface**: Memory-efficient streaming through large datasets
- **Comprehensive validation**: Built-in data integrity verification utilities
- **File I/O utilities**: Helper functions for file-based chunking operations

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
seq-chunking = "0.1.0"
```

### Basic Usage

```rust
use seq_chunking::{SeqChunking, ChunkingConfig, SeqOpMode};

// Create a chunker with default settings
let chunker = SeqChunking::new();

// Chunk some data
let data = b"your data here";
let chunks: Vec<_> = chunker.chunk_all(data).collect();

// Verify integrity
for chunk in &chunks {
    println!("Chunk: {} bytes at position {}", chunk.len, chunk.start);
}
```

### Custom Configuration

```rust
use seq_chunking::{SeqChunking, ChunkingConfig, SeqOpMode};

// Build a custom configuration
let config = ChunkingConfig::builder()
    .seq_threshold(10)                    // Longer sequences needed
    .min_block_size(2048)                 // 2KB minimum chunks
    .max_block_size(32768)                // 32KB maximum chunks
    .op_mode(SeqOpMode::Decreasing)       // Look for decreasing sequences
    .jump_trigger(100)                    // Jump after 100 opposing slopes
    .build()
    .expect("Invalid configuration");

let chunker = SeqChunking::from_config(config);
let chunks: Vec<_> = chunker.chunk_all(data).collect();
```

### File Processing

```rust
use seq_chunking::{SeqChunking, utils::FileUtils};

// Read a file and chunk it
let data = FileUtils::read_file("input.dat")?;
let chunker = SeqChunking::new();
let chunks: Vec<_> = chunker.chunk_all(&data).collect();

// Write chunks back to a file
FileUtils::write_chunks_to_file("output.dat", &chunks)?;
```

## Algorithm Overview

The SeqChunking algorithm works by:

1. **Scanning** through the data starting from the minimum block size
2. **Detecting** sequences of increasing or decreasing bytes
3. **Counting** consecutive bytes that follow the pattern
4. **Creating** a chunk boundary when the sequence threshold is reached
5. **Jumping** ahead when too many opposing slopes are encountered (optimization)

### Operation Modes

- **Increasing Mode**: Detects sequences where each byte is greater than or equal to the previous byte
- **Decreasing Mode**: Detects sequences where each byte is less than or equal to the previous byte

### Key Parameters

- `seq_threshold`: Number of consecutive sequence bytes needed to trigger a cut
- `min_block_size`: Minimum chunk size in bytes
- `max_block_size`: Maximum chunk size in bytes  
- `jump_trigger`: Number of opposing slopes before jumping ahead
- `jump_size`: Number of bytes to skip when jumping

## Performance

The library is designed for high performance with several optimizations:

- **Jump-ahead mechanism**: Skips regions with many opposing slopes
- **Low entropy absorption**: Efficiently handles runs of identical bytes
- **Bounded scanning**: Respects minimum and maximum block size limits
- **Iterator-based API**: Memory-efficient processing of large datasets

Typical performance on modern hardware:
- **Throughput**: 100-500 MB/s depending on data patterns
- **Memory usage**: O(1) for streaming, O(n) for collecting all chunks

## Use Cases

- **Data deduplication**: Content-defined chunking for backup systems
- **Stream processing**: Dividing continuous data streams into manageable chunks
- **Network protocols**: Packet boundary detection in network streams
- **File synchronization**: Efficient diff algorithms for large files
- **Database systems**: Variable-length record processing

## API Documentation

### Core Types

- `SeqChunking`: Main chunking algorithm implementation
- `ChunkingConfig`: Configuration parameters for the algorithm
- `Chunk`: Represents a single chunk with data and position information
- `ChunkIterator`: Iterator for streaming through chunks

### Utility Modules

- `utils::FileUtils`: File I/O operations
- `utils::ValidationUtils`: Data integrity verification
- `utils::TestDataGenerator`: Generate test data with specific patterns
- `utils::PerfUtils`: Performance measurement utilities

## Examples

The library includes several examples demonstrating different use cases:

```bash
# Basic usage example
cargo run --example basic_usage

# File processing example  
cargo run --example file_processing
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under either of MIT license

## Changelog

### Version 0.1.0

- Initial release
- Basic sequence-based chunking algorithm
- Support for increasing and decreasing modes
- Configuration builder pattern
- File I/O utilities
- Comprehensive test suite