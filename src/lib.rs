//! ticker provides a periodic tick to rate limit an Iterator.
//!
//! E.g. print 0-9, one number per second:
//!
//! ````
//! let ticker = Ticker::new((0..10), Duration::from_secs(1));
//! for i in ticker {
//!     println!("{:?}", i)
//! }
//! ````

use std::time::Duration;
use std::thread;
use std::sync::mpsc::{Receiver, Sender, channel, RecvTimeoutError};

/// Ticker rate limits an Iterator. A ticking Iterator unblocks at most once per
/// interval.
///
/// Print 0-9, one number per second:
///
/// ````
/// let ticker = Ticker::new((0..10), Duration::from_secs(1));
/// for i in ticker {
///     println!("{:?}", i)
/// }
/// ````
///
/// Run some function every second infinitely:
///
/// ````
/// for _ in Ticker::new((0..), Duration::from_secs(1)) {
///     somefunc()
/// }
/// ````
pub struct Ticker<I: Iterator> {
    src: I,
    recv: Receiver<()>,
    kill: Sender<()>,
}

impl<I: Iterator> Ticker<I> {
    /// new creates a Ticker which will rate limit returns from ````src````,
    /// returning from ````.next()```` at most once every ````interval````.
    pub fn new(src: I, interval: Duration) -> Self {
        let (send, recv) = channel::<()>();
        let (kill, kill_recv) = channel::<()>();
        thread::spawn(move || loop {
                          match kill_recv.recv_timeout(interval) {
                              Err(RecvTimeoutError::Timeout) => {
                                  if let Err(_) = send.send(()) {
                                      return;
                                  }
                              }
                              _ => return,
                          }
                      });

        Ticker { src, recv, kill }
    }
}

impl<I: Iterator> Drop for Ticker<I> {
    fn drop(&mut self) {
        let _ = self.kill.send(());
    }
}

impl<I: Iterator> IntoIterator for Ticker<I> {
    type Item = I::Item;
    type IntoIter = TickIter<I>;

    fn into_iter(self) -> Self::IntoIter {
        TickIter { ticker: self }
    }
}

/// TickIter implements a rate limited Iterator; derive this from Ticker using
/// for loop syntax or ````.into_iter()````.
pub struct TickIter<I: Iterator> {
    ticker: Ticker<I>,
}

impl<I: Iterator> Iterator for TickIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let _ = self.ticker.recv.recv().expect("ticker channel to live");
        self.ticker.src.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec: Vec<usize> = (0..10).collect();
        let ticker = Ticker::new(vec.iter(), Duration::from_secs(1));
        for i in ticker {
            println!("{:?}", i);
        }
    }
}
