//! Concurrent indexer using Arc and Mutex.
#![allow(dead_code)]
//!
//! Demonstrates:
//! - Arc: shared ownership across threads
//! - Mutex: interior mutability for thread-safe mutation
//! - std::thread: spawning and joining threads

use std::sync::{Arc, Mutex};

use crate::models::{Block, Event};
use crate::state::BalanceState;

/// Indexer that holds state behind Arc<Mutex<>> for thread-safe shared access.
pub struct ConcurrentIndexer {
    state: Arc<Mutex<BalanceState>>,
}

impl ConcurrentIndexer {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(BalanceState::new())),
        }
    }

    /// Returns a clone of the Arc so it can be shared with threads.
    /// Each thread gets its own Arc pointer; all point to the same BalanceState.
    pub fn state_handle(&self) -> Arc<Mutex<BalanceState>> {
        Arc::clone(&self.state)
    }

    /// Index a block into the shared state. Acquires the lock, applies transfers, releases.
    pub fn index_block(&self, block: &Block) {
        for transaction in &block.transactions {
            for event in &transaction.events {
                match event {
                    Event::Transfer { from, to, amount } => {
                        let mut state = self.state.lock().expect("mutex poisoned");
                        state.apply_transfer(from, to, *amount);
                    }
                }
            }
        }
    }

    /// Get the current balances. Acquires lock for read.
    pub fn get_balances(&self) -> std::collections::HashMap<String, u64> {
        let state = self.state.lock().expect("mutex poisoned");
        state.get_balances().clone()
    }
}
