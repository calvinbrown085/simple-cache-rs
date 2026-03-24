# BYTECHEF.md

`simple-cache-rs` is a Rust library that provides a straightforward in-memory cache implemented atop a `HashMap`. Its primary feature is optional expiration control for cached entries, allowing developers to store key-value pairs that automatically become inaccessible and are lazily removed after a specified duration.

## Architecture

The core of `simple-cache-rs` is the `SimpleCache` struct, which encapsulates a standard `std::collections::HashMap`. Each value stored in the `HashMap` is wrapped within an `Entry` struct, which additionally records the `Instant` when the item was inserted. The `SimpleCache` itself holds an optional `timeout` `Duration`. When an item is retrieved using the `get` method, the cache checks if a timeout is configured and if the item's elapsed time since insertion exceeds this timeout. If an item has expired, it is lazily removed from the cache, and `None` is returned. All other operations, such as `insert`, `delete`, `keys`, `values`, `tuples`, and `insert_batch`, manage the underlying `HashMap` entries directly, adhering to the optional expiration policy during retrieval.

## Key Files

*   **`.github/workflows/rust.yml`**: Configures the continuous integration (CI) workflow using GitHub Actions. It ensures that the Rust project builds and its unit tests pass on every push and pull request, specifically targeting the `rust-unit-test` workflow.
*   **`Cargo.toml`**: The package manifest for the Rust project. It defines the crate's name (`simple-cache-rs`), version (`0.4.0`), authors, description, Apache-2.0 license, and keywords (`cache`, `mem-cache`). It explicitly states its compatibility with `edition = "2018"`.
*   **`README.md`**: Provides a concise overview of the library, including build status, documentation, and license badges. It features clear, runnable examples demonstrating how to initialize and use the `SimpleCache` both with and without expiration timers.
*   **`src/lib.rs`**: Contains the entire implementation of the `simple-cache-rs` library. It defines the `Entry` struct for internal use, the public `SimpleCache` struct, and all its associated methods (`new`, `get`, `keys`, `values`, `tuples`, `insert_batch`, `insert`, `delete`). This file also houses comprehensive unit tests for the cache's functionalities, including expiration logic.

## How to Run

`simple-cache-rs` is a library crate designed to be integrated into other Rust projects.

1.  **Add to your project**: Add `simple-cache-rs` as a dependency in your `Cargo.toml` file:

    ```bash
    cargo add simple-cache-rs
    ```

    Alternatively, manually add the following to your `Cargo.toml`:

    ```toml
    [dependencies]
    simple-cache-rs = "0.4" # Use the appropriate version
    ```

2.  **Use in your code**: Import and utilize the `SimpleCache` struct in your Rust application.

    **Example without expiration:**

    ```rust
    use simple_cache_rs::SimpleCache;

    fn main() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);
        scache.insert(1, String::from("test"));
        println!("{:?}", scache.get(&1)); // Output: Some("test")
    }
    ```

    **Example with expiration:**

    ```rust
    use simple_cache_rs::SimpleCache;
    use std::time::Duration;
    use std::thread;

    fn main() {
        let timeout = Duration::new(1, 0);
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(Some(timeout));

        scache.insert(1, String::from("test"));
        assert_eq!(Some(String::from("test")), scache.get(&1));

        thread::sleep(Duration::new(1, 1)); // Wait for longer than timeout
        assert_ne!(Some(String::from("test")), scache.get(&1)); // Value should be expired
    }
    ```

## How to Test

The repository includes a suite of unit tests located within `src/lib.rs`. These tests cover various functionalities of the `SimpleCache`, including item insertion, retrieval, deletion, batch operations, key/value/tuple enumeration, and the core expiration logic.

To execute all tests:

```bash
cargo test
```