# ticker

Rate limited Iterators!

Print 0-9, one number per second:

````rust
let ticker = Ticker::new((0..10), Duration::from_secs(1));
for i in ticker {
    println!("{:?}", i)
}
````
