//! One-shot value exhange between threads. The consumer thread may await the producer thread
//! Both the producer and consumer are single use, though there is a version of the consumer that may be cloned
//! 
//! # Example
//! 
//! ```
//! let (send, recv) = promissory();
//! thread::spawn(move || send.fulfill(42));
//! assert_eq!(42, recv.await_value());
//! ```
//! 


mod basic_promissory;

pub use crate::basic_promissory::{
    cloneable_promissory, promissory, Awaiter, BaseAwaiter, CloneAwaiter, Fulfiller,
};
