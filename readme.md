# ticker

[![crates.io](https://img.shields.io/crates/v/ticker.svg)](https://crates.io/crates/ticker)
[![docs](https://img.shields.io/badge/docs-reference-orange.svg)](https://docs.rs/ticker/0.1.1/ticker/)

Rate limited Iterators!

Print 0-9, one number per second:

````rust
let ticker = Ticker::new((0..10), Duration::from_secs(1));
for i in ticker {
    println!("{:?}", i)
}
````
