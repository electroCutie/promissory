use std::sync::{Arc, Condvar, Mutex};

pub(crate) struct Promissory<T> {
    pub(crate) mutex: Mutex<Option<T>>,
    pub(crate) cond: Condvar,
}

/// Object which allows a one-shot fulfillment of the promissory
pub struct Fulfiller<T>(pub(crate) Arc<Promissory<T>>);

pub trait Awaiter<T: Send> {
    /// Attempt to retrive the computed value, blocking if nessesary
    fn await_value(self) -> T;
}

/// Construct a Fullfiller / Awaiter pair
pub fn promissory<T>() -> (Fulfiller<T>, BaseAwaiter<T>)
where
    T: Send,
{
    let p = Arc::new(Promissory {
        mutex: Mutex::new(None),
        cond: Condvar::new(),
    });

    (Fulfiller(p.clone()), BaseAwaiter(p))
}

impl<T> Fulfiller<T>
where
    T: Send,
{
    /// Consume the fulfiller and awake any waiters / mark the Promissory as fulfilled
    pub fn fulfill(self, t: T) {
        *(self.0.mutex.lock().expect("broken lock")) = Some(t);
        self.0.cond.notify_all() // All because we use this one impl for this and clone
    }
}

/// An await that cannot be cloned
pub struct BaseAwaiter<T>(Arc<Promissory<T>>);

impl<T: Send> Awaiter<T> for BaseAwaiter<T> {
    fn await_value(self) -> T {
        let lock = self.0.mutex.lock().expect("broken lock");
        let cond = &self.0.cond;
        let mut lock = cond
            .wait_while(lock, |o| o.is_none())
            .expect("broken condition");

        let mut val = None;
        // Can't destroy the object so just pull the ol' switcheroo
        std::mem::swap(&mut val, &mut *lock);

        val.unwrap()
    }
}

/// An Awaiter that can be cloned
#[derive(Clone)]
pub struct CloneAwaiter<T: Clone>(Arc<Promissory<T>>);

/// Construct a Fullfiller / Awaiter pair where the Awaiter implements Clone
pub fn cloneable_promissory<T: Clone>() -> (Fulfiller<T>, CloneAwaiter<T>)
where
    T: Send,
{
    let p = Arc::new(Promissory {
        mutex: Mutex::new(None),
        cond: Condvar::new(),
    });

    (Fulfiller(p.clone()), CloneAwaiter(p))
}

impl<T> Awaiter<T> for CloneAwaiter<T>
where
    T: Send + Clone,
{
    fn await_value(self) -> T {
        let lock = self.0.mutex.lock().expect("broken lock");
        let cond = &self.0.cond;
        let lock = cond
            .wait_while(lock, |o| o.is_none())
            .expect("broken condition");

        lock.clone().unwrap()
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

    #[test]
    fn cloning() {
        let (send, recv) = cloneable_promissory();

        let recv_b = recv.clone();

        thread::spawn(move || send.fulfill(42));

        let joiner = thread::spawn(move || assert_eq!(42, recv_b.await_value()));

        assert_eq!(42, recv.await_value());
        joiner.join().expect("err in thread b");
    }
}
