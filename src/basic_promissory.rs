use std::sync::mpsc::{self, Sender, Receiver};

/// Object which allows a one-shot fulfillment of the promissory
pub struct Fulfiller<T>(pub(crate) Sender<T>);

pub trait Awaiter<T: Send> {
    /// Attempt to retrive the computed value, blocking if nessesary
    fn await_value(self) -> T;
}

/// Construct a Fullfiller / Awaiter pair
pub fn promissory<T>() -> (Fulfiller<T>, BaseAwaiter<T>)
where
    T: Send,
{
    let (send, recv) = mpsc::channel();

    (Fulfiller(send), BaseAwaiter(recv))
}

impl<T> Fulfiller<T>
where
    T: Send,
{
    /// Consume the fulfiller and awake any waiters / mark the Promissory as fulfilled
    pub fn fulfill(self, t: T) {
        self.0.send(t).expect("should be impossible")
    }
}

/// An await that cannot be cloned
pub struct BaseAwaiter<T>(Receiver<T>);

impl<T: Send> Awaiter<T> for BaseAwaiter<T> {
    fn await_value(self) -> T {
        self.0.recv().expect("should be impossile")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn basic_exchange() {
        let (send, recv) = promissory();
        thread::spawn(move || send.fulfill(42));
        assert_eq!(42, recv.await_value());
    }
}
