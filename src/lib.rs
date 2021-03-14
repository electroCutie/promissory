//! One-shot value exhange between threads. The consumer thread may await the producer thread
//! Both the producer and consumer are single use
//!
//! Just a wrapper around a std::mpsc channel with an API that makes it impossible to use more than once
//!
//! # Example
//!
//! ```
//! use promissory::{promissory, Awaiter};
//! let (send, recv) = promissory::promissory();
//! std::thread::spawn(move || send.fulfill(42u32));
//! assert_eq!(42, recv.await_value().expect("this thread cannot die"));
//! ```
//!

mod basic_promissory;

pub use crate::basic_promissory::{promissory, Awaiter, Fulfiller};
