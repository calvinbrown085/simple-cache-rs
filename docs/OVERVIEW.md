# Overview for simple-cache-rs

simple-cache-rs is a Rust library that provides a basic in-memory cache implementation, leveraging a `HashMap` internally. Its key feature is optional expiration control for cached items, allowing developers to store and retrieve data with a configurable time-to-live, making it suitable for scenarios requiring temporary data storage.

## Architecture

The `simple-cache-rs` library is built around a single primary struct, `SimpleCache<K, V>`, which manages all caching logic.

*   **`SimpleCache<K, V>`**: This generic struct serves as the main entry point for cache operations. It internally holds a `std::collections::HashMap<K, Entry<V>>` to store key-value pairs and an `Option<Duration>` (`timeout`) to define a global expiration for items. `K` must implement `Eq`, `Hash`, `Clone`, `Debug`, and `V` must implement `Clone`, `Debug`.
*   **`Entry<V>`**: A private struct used internally by `SimpleCache`. Each `Entry` wraps the actual cached `value` (`V`) and records its `insert_time` as a `std::time::Instant`. This timestamp is crucial for determining if an item has expired.
*   **Expiration Logic**: When a `timeout` is configured during `SimpleCache` instantiation, the `get()` method performs a lazy expiration check. If an item's `insert_time` plus the configured `timeout` is less than or equal to the current time, the item is considered expired, automatically removed from the internal `HashMap`, and `None` is returned. If no `timeout` is set, items persist indefinitely until explicitly deleted.

## Key Files

*   **`Cargo.toml`**: This manifest file defines the package's metadata, including its name (`simple-cache-rs`), a concise description, version (`0.4.0`), authors, and license (`Apache-2.0`). It also declares `keywords` for discoverability and specifies the `edition = "2018"` for Rust syntax compatibility. This project currently has no external dependencies.
*   **`README.md`**: Provides a high-level introduction to the `simple-cache-rs` library. It includes badges for documentation status, license, and CI pipeline results. Crucially, it features practical code examples demonstrating how to initialize and use the `SimpleCache` both with and without item expiration.
*   **`src/lib.rs`**: This file contains the complete source code for the `simple-cache-rs` library. It defines the core `Entry` and `SimpleCache` structs, implementing all public API methods such as `new` (constructor), `get` (retrieve with expiration check), `insert` (add single item), `insert_batch` (add multiple items), `delete` (remove item), `keys` (get all keys), `values` (get all values), and `tuples` (get all key-value pairs). The file also includes an `#[cfg(test)]` module with comprehensive unit tests to ensure the cache's functionality and expiration logic work as expected.

## How to Use

To incorporate `simple-cache-rs` into your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
simple-cache-rs = "0.4.0" # Use the latest version
```

Then, you can use the `SimpleCache` in your Rust code:

### Basic Usage (without expiration)

```rust
use simple_cache_rs::SimpleCache;

// Create a cache without a global timeout
let mut cache: SimpleCache<i32, String> = SimpleCache::new(None);

// Insert an item
cache.insert(1, String::from("hello world"));

// Retrieve an item
println!("{:?}", cache.get(&1)); // Output: Some("hello world")

// Delete an item
cache.delete(&1);
println!("{:?}", cache.get(&1)); // Output: None
```

### Usage with Expiration

```rust
use simple_cache_rs::SimpleCache;
use std::time::Duration;
use std::thread; // For demonstration purposes

// Create a cache with a 1-second timeout
let timeout = Duration::new(1, 0);
let mut cache: SimpleCache<i32, String> = SimpleCache::new(Some(timeout));

// Insert an item
cache.insert(1, String::from("expiring value"));
assert_eq!(Some(String::from("expiring value")), cache.get(&1));

// Wait for the item to expire
thread::sleep(Duration::new(1, 1)); // Sleep for 1 second and 100 milliseconds

// Attempt to retrieve the item; it will be expired and removed
assert_eq!(None, cache.get(&1));
```

## How to Test

The library includes a suite of unit tests to verify its functionality. You can run these tests using Cargo:

```bash
cargo test
```

The tests are defined within the `#[cfg(test)]` module in `src/lib.rs` and cover various scenarios, including item insertion, retrieval, deletion, batch operations, and expiration logic. The `.github/workflows/rust.yml` file also configures GitHub Actions to automatically run these tests on every push, ensuring continuous integration and code quality.