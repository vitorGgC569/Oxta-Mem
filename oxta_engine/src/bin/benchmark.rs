use oxta_mem::engine::GeodesicEngine;
use std::time::Instant;

fn main() {
    println!("--- BENCHMARK: The Flash Boys Test ---");

    // Setup
    let path = "bench_store.db";
    let _ = std::fs::remove_file(path);
    let mut engine = GeodesicEngine::new(path, 1024).expect("Failed to create engine");

    let iterations = 100_000;
    println!("Inserting {} records...", iterations);

    let start = Instant::now();
    for i in 0..iterations {
        let val = format!("value_{}", i).into_bytes();
        engine.write("HFT_TICKER", val).unwrap();
    }
    let duration = start.elapsed();
    println!("Write Time: {:?}", duration);
    println!("Writes/sec: {:.2}", iterations as f64 / duration.as_secs_f64());

    println!("\n--- BENCHMARK: Latency P99 ---");
    let mut latencies = Vec::with_capacity(iterations);

    // We will read random depths
    for _ in 0..10_000 {
        let start_read = Instant::now();
        engine.read_latest("HFT_TICKER");
        latencies.push(start_read.elapsed().as_micros());
    }

    latencies.sort();
    let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];
    println!("P99 Read Latency: {} us", p99);

    let _ = std::fs::remove_file(path);
}
