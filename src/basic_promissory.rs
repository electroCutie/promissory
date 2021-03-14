use std::sync::mpsc::{self, Receiver, RecvError, SendError, Sender};

/// Object which allows a one-shot fulfillment of the promissory
pub struct Fulfiller<T>(pub(crate) Sender<T>);
/// An await that cannot be cloned
pub struct Awaiter<T>(Receiver<T>);

/// Construct a Fullfiller / Awaiter pair
pub fn promissory<T>() -> (Fulfiller<T>, Awaiter<T>)
where
    T: Send,
{
    let (send, recv) = mpsc::channel();

    (Fulfiller(send), Awaiter(recv))
}

impl<T: Send> Awaiter<T> {
    pub fn await_value(self) -> Result<T, RecvError> {
        self.0.recv()
    }
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
