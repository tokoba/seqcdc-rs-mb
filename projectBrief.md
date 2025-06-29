# Project Brief: seq_chunking

## 1. Project Overview

**`seq_chunking`** is a pure Rust library for content-defined chunking (CDC) based on sequence detection. As described in `Cargo.toml`, its purpose is "SeqCDC (content defined chunking) in pure Rust."

The library implements an efficient algorithm to divide data streams or byte arrays into variable-sized chunks. Chunk boundaries are determined by detecting patterns in the byte data itself—specifically, sequences of increasing or decreasing byte values (slopes). This makes it particularly suitable for applications like data deduplication, file synchronization, and stream processing, where chunk boundaries need to be stable across insertions or deletions.

## 2. Core Functionality

The core functionality revolves around the `SeqChunking` struct, which scans a byte slice (`&[u8]`) and splits it into a series of `Chunk`s. The process is exposed via a memory-efficient `ChunkIterator`.

The underlying algorithm works as follows:

1. It starts scanning the data from a configured `min_block_size`.
2. It compares adjacent bytes to detect a "slope" (whether the value is increasing or decreasing).
3. It counts consecutive bytes that follow the same slope direction.
4. When the count of consecutive slope bytes reaches a `seq_threshold`, a chunk boundary is created.
5. To optimize performance in data without clear patterns, it implements a "jump-ahead" mechanism. If too many (`jump_trigger`) opposing slopes are found, the scanner jumps forward by a `jump_size`.
6. The process is bounded by `min_block_size` and `max_block_size` to ensure chunks are within a reasonable size range.

## 3. Key Features

- **Sequence-based Chunking**: Uses slope detection (increasing or decreasing byte sequences) to define chunk boundaries.
- **Configurable Parameters**: Allows customization of sequence thresholds, block sizes, operation modes, and jump parameters through the `ChunkingConfig` builder.
- **Operation Modes**: Supports both `Increasing` and `Decreasing` sequence detection via the `SeqOpMode` enum.
- **High Performance**: Optimized with a jump-ahead mechanism for regions with low sequence-correlation and low-entropy absorption for runs of identical bytes.
- **Iterator Interface**: Provides a `ChunkIterator` for memory-efficient streaming over large datasets without loading the entire file into memory at once.
- **Comprehensive Utilities**: Includes helpers for file I/O (`FileUtils`), data integrity validation (`ValidationUtils`), performance measurement (`PerfUtils`), and test data generation (`TestDataGenerator`).

## 4. Project Structure

The project is organized into several modules, each with a distinct responsibility.

```text
/
├── Cargo.toml        # Project metadata and dependencies
├── README.md         # High-level documentation
├── src/
│   ├── lib.rs        # Crate root, public API, and module declarations
│   ├── chunker.rs    # Core chunking logic and data structures
│   ├── config.rs     # Configuration structs and builder
│   ├── error.rs      # Custom error types and Result alias
│   └── utils.rs      # Helper modules for I/O, validation, etc.
├── examples/
│   ├── basic_usage.rs # Demonstrates various features
│   └── file_processing.rs # Focuses on file I/O
└── benches/
    └── chunking_benchmark.rs # Performance benchmarks
```

- **`src/lib.rs`**: The entry point of the library. It re-exports the primary public types like `SeqChunking`, `ChunkingConfig`, `Chunk`, and `Result`.
- **`src/chunker.rs`**: Contains the main algorithm implementation (`SeqChunking`), the `Chunk` struct representing a piece of data, the `ChunkIterator` for iterating over chunks, and `ChunkingStats` for statistics.
- **`src/config.rs`**: Defines `ChunkingConfig` for setting up the chunker's parameters and `ChunkingConfigBuilder` for a fluent configuration experience. It also defines the `SeqOpMode` enum.
- **`src/error.rs`**: Defines the crate's error enum `ChunkingError` and the `Result<T>` type alias.
- **`src/utils.rs`**: Provides several utility structs: `FileUtils`, `ValidationUtils`, `TestDataGenerator`, and `PerfUtils`.

## 5. Core Components

- **`SeqChunking`**: The main struct that orchestrates the chunking process. It is initialized with a `ChunkingConfig` and provides the `chunk_all()` method, which returns an iterator.
- **`ChunkingConfig`**: A struct holding all configuration parameters (`seq_threshold`, `min_block_size`, `max_block_size`, `op_mode`, etc.). It can be constructed using the `ChunkingConfigBuilder`.
- **`Chunk<'a>`**: A struct representing a single data chunk. It holds a reference to the original data (`&'a [u8]`), its `start` position, and its `len`. It does not own the data, making it lightweight.
- **`ChunkIterator<'a>`**: An iterator that lazily yields `Chunk`s from the input data, making the process memory-efficient.
- **`SeqOpMode`**: An enum with two variants, `Increasing` and `Decreasing`, which controls the algorithm's behavior.

## 6. Public API and Usage

The library is designed to be straightforward to use.

### Basic Usage

Create a chunker with default settings and iterate over the chunks.

```rust
use seq_chunking::{SeqChunking, ChunkingConfig, SeqOpMode};

// Create a chunker with default settings
let chunker = SeqChunking::new();

// Chunk some data
let data = b"your data here";
let chunks: Vec<_> = chunker.chunk_all(data).collect();

for chunk in &chunks {
    println!("Chunk: {} bytes at position {}", chunk.len, chunk.start);
}
```

### Custom Configuration

Use the `ChunkingConfigBuilder` to tailor the chunking parameters.

```rust
use seq_chunking::{SeqChunking, ChunkingConfig, SeqOpMode};

// Build a custom configuration
let config = ChunkingConfig::builder()
    .seq_threshold(10)                    // Longer sequences needed
    .min_block_size(2048)                 // 2KB minimum chunks
    .max_block_size(32768)                // 32KB maximum chunks
    .op_mode(SeqOpMode::Decreasing)       // Look for decreasing sequences
    .build()
    .expect("Invalid configuration");

let chunker = SeqChunking::from_config(config);
let data = b"some other data";
let chunks: Vec<_> = chunker.chunk_all(data).collect();
```

### File Processing

The `utils::FileUtils` module simplifies reading from and writing to files.

```rust
use seq_chunking::{SeqChunking, utils::FileUtils};

// Read a file and chunk it
let data = FileUtils::read_file("input.dat")?;
let chunker = SeqChunking::new();
let chunks: Vec<_> = chunker.chunk_all(&data).collect();

// Write chunks back to a file, reconstructing the original content
FileUtils::write_chunks_to_file("output.dat", &chunks)?;
```

## 7. Build and Test

The project uses standard Cargo commands for building, testing, and running examples.

- **Build**: `cargo build`
- **Run tests**: `cargo test`
- **Run benchmarks**: `cargo bench`
- **Run an example**: `cargo run --example basic_usage`
