// Original single-threaded indexer. Kept for architecture; main uses ConcurrentIndexer in Phase 9.
#![allow(dead_code)]

use crate::models::{Block, Event};
use crate::state::BalanceState;

pub struct BlockIndexer {
    state: BalanceState,
}

pub trait Indexer {
    fn index_block(&mut self, block: &Block);
}

impl BlockIndexer {
    pub fn new() -> Self {
        Self {
            state: BalanceState::new(),
        }
    }

    pub fn get_state(&self) -> &BalanceState {
        &self.state
    }
}

impl Indexer for BlockIndexer {
    fn index_block(&mut self, block: &Block) {
        for transaction in &block.transactions {
            for event in &transaction.events {
                match event {
                    Event::Transfer { from, to, amount } => {
                        self.state.apply_transfer(from, to, *amount);
                    }
                }
            }
        }
    }
}

