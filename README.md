# A simple in-mem cache for Rust
[![Docs of original repo](https://docs.rs/simple-cache-rs/badge.svg)](https://docs.rs/simple-cache-rs/)
[![Apache-2 licensed](https://img.shields.io/crates/l/simple-cache-rs.svg)](https://github.com/calvinbrown085/simple-cache-rs/blob/main/LICENSE)
[![CI](https://img.shields.io/github/workflow/status/calvinbrown085/simple-cache-rs/rust-unit-test)](https://github.com/calvinbrown085/simple-cache-rs/actions?query=workflow%3ARust)

A simple rust implementation of HashMap with expiration control.

## Example
### Without expiration:
```rust
use simple_cache_rs::SimpleCache;

let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);

scache.insert(1, String::from("test"));
println!("{:?}", scache.get(&1));
```

### With expiration:
```rust
use simple_cache_rs::SimpleCache;
use std::time::Duration;
use std::thread; // For example purposes only

let timeout = Duration::new(1, 0);
let mut scache: SimpleCache<i32, String> = SimpleCache::new(Some(timeout));

let str_test = String::from("test");

scache.insert(1, str_test);
assert_eq!(Some(str_test), scache.get(&1));

thread::sleep(Duration::new(1, 1)); // For example purposes only
assert_ne!(Some(str_test), scache.get(&1));
```
