# Blockchain state indexer

This is a concurrent event-driven blockchain indexer written in Rust.

It simulates block ingestion, event decoding, and state reconstruction
to demonstrate production-grade Rust systems design patterns:

- Ownership & borrowing
- Traits and generics
- Pattern matching
- Concurrency with Arc + Mutex
- Async task execution (Tokio)
- Error propagation
- Modular architecture

This project is a foundational step toward building a multi-chain,
reorg-safe, production-grade blockchain indexer.