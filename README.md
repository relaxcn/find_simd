This is a SIMD implementation of `std::str::find` in Rust.

## How to run

Just test it!

```bash
cargo test --release -- --nocapture
```

You will see the difference of the two implementations.

Or you want to use some other tools like `hyperfine`, just use `--simd` parameter!

```bash
hyperfine --warmup 1 -r 1 'cargo run --release' 'cargo run --release -- --simd'
```