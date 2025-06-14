use criterion::{criterion_group, criterion_main, Criterion};
use seq_chunking::SeqChunking;
use seq_chunking::utils::TestDataGenerator;
use std::fs::File;
use std::io::Write;

const FILE_SIZE: usize = 1_000_000_000; // 1GB

fn create_test_file() -> String {
    let file_path = "test_file.dat".to_string();
    let mut file = File::create(&file_path).unwrap();
    let data = TestDataGenerator::generate_pseudo_random(FILE_SIZE, 12345);
    file.write_all(&data).unwrap();
    file_path
}

fn chunking_benchmark(c: &mut Criterion) {
    let chunker = SeqChunking::new();
    let file_path = create_test_file();
    let data = std::fs::read(&file_path).unwrap();

    c.bench_function("chunking 1GB data", |b| {
        b.iter(|| chunker.chunk_all_vec(&data))
    });

    std::fs::remove_file(&file_path).unwrap();
}

criterion_group!(benches, chunking_benchmark);
criterion_main!(benches);