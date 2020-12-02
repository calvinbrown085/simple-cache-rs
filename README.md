# simple-cache-rs
A simple im-mem cache for Rust

![Docs](https://docs.rs/simple-cache-rs/badge.svg)
[![Apache-2 licensed](https://img.shields.io/crates/l/simple-cache-rs.svg)](./LICENSE)
[![CI](https://github.com/calvinbrown085/simple-cache-rs/workflows/Rust/badge.svg)](https://github.com/calvinbrown085/simple-cache-rs/actions?query=workflow%3ARust)

A simple rust implementation of HashMap with expiration control.
## Example
```rust
use simple_cache_rs::SimpleCache;

let mut cache: SimpleCache<i32, String> = SimpleCache::new(None);

cache.insert(1, String::from("test"));
println!("{:?}", cache.get(&1));
```
