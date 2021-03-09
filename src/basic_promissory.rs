use std::sync::mpsc::{self, Receiver, RecvError, Sender, SendError};

/// Object which allows a one-shot fulfillment of the promissory
pub struct Fulfiller<T>(pub(crate) Sender<T>);

pub trait Awaiter<T: Send> {
    /// Attempt to retrive the computed value, blocking if nessesary
    fn await_value(self) -> Result<T, RecvError>;
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
    pub fn fulfill(self, t: T) -> Result<(), SendError<T>> {
        self.0.send(t)
    }
}

/// An await that cannot be cloned
pub struct BaseAwaiter<T>(Receiver<T>);

impl<T: Send> Awaiter<T> for BaseAwaiter<T> {
    fn await_value(self) -> Result<T, RecvError> {
        self.0.recv()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn basic_exchange() -> Result<(), RecvError> {
        let (send, recv) = promissory();
        thread::spawn(move || send.fulfill(42));
        let r = recv.await_value()?;
        assert_eq!(42, r);
        Ok(())
    }
}
