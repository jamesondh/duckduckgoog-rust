# duckduckgoog-rust
Use DuckDuckGo for !bangs and Google for everything else.

Built in Rust and [Warp](https://docs.rs/warp/latest/warp/) to optimize for speed. No interface, only a `/search` endpoint using HTTP GET. Inspired by [the original DuckDuckGoog](https://github.com/crittermike/duckduckgoog).

```bash
cargo run --release
```
