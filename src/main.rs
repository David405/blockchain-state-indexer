mod concurrent_indexer;
mod errors;
mod indexer;
mod loader;
mod models;
mod state;

use std::path::Path;
use std::thread;

use concurrent_indexer::ConcurrentIndexer;
use loader::load_blocks_from_file;

fn main() {
    let path = Path::new("data/sample_blocks.json");

    let blocks = match load_blocks_from_file(&path) {
        Ok(blocks) => blocks,
        Err(e) => {
            eprintln!("Error loading blocks: {}", e);
            std::process::exit(1);
        }
    };

    let indexer = ConcurrentIndexer::new();
    let blocks_for_thread = blocks;

     // tokio::spawn creates an async task (lightweight, not an OS thread).
    // spawn_blocking runs CPU-bound work on a thread pool so we don't block the runtime.
    let indexer = tokio::task::spawn_blocking(move || {
        for block in &blocks_for_task {
            indexer.index_block(block);
        }
        indexer
    })
    .await
    .expect("task panicked");

    println!("Balances after indexing: {:?}", indexer.get_balances());
}