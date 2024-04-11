# Metadata Fetcher
Metadata Fetcher is a utility for grabbing website metadata; useful for tasks like generating link 
previews. Its built ontop of [ureq](https://crates.io/crates/ureq).

### Behavior
Metadata Fetcher first looks for a site's Open Graph Protocol (OGP) metadata and if not found, it
looks for the standard HTML metadata. If no metadata is found, it returns `None` for the missing 
field. This module also respects a site's `robots.txt` file.

### Usage
```rust
use meta_fetcher::fetch_metadata;

// Grab the metadata for some URL
let meta = fetch_metadata("http://example.com").unwrap();

assert_eq!(meta.title, Some("Example Title".to_string()));
assert_eq!(meta.description, Some("Example Description".to_string()));
assert_eq!(meta.image, Some("Image URL".to_string()));
```

#### Running Tests
The Makefile specifies two types of tests. `make test-ci` is the same as running `cargo test` and
will run all tests that do not require network activity to run. `make test` is the same as running
`cargo test --features network-tests` and will run all tests, including those that require a network
connection.

### License
MIT
