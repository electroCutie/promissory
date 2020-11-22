//! One-shot value exhange between threads. The consumer thread may await the producer thread
//! Both the producer and consumer are single use, though there is a version of the consumer that may be cloned
//!
//! # Example
//!
//! ```
//! use promissory::{promissory, Awaiter};
//! let (send, recv) = promissory::promissory();
//! std::thread::spawn(move || send.fulfill(42u32));
//! assert_eq!(42, recv.await_value());
//! ```
//!

mod basic_promissory;

pub use crate::basic_promissory::{
    promissory, Awaiter, BaseAwaiter, Fulfiller,
};
