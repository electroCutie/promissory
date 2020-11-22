use std::sync::{Arc, Condvar, Mutex};

use super::basic_promissory::{Fulfiller, Promissory};

#[derive(Clone)]
pub struct CloneAwaiter<T: Clone>(Arc<Promissory<T>>);

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

impl<T> CloneAwaiter<T>
where
    T: Send + Clone,
{
    pub fn await_value(self) -> T {
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
    fn cloning() {
        let (send, recv) = cloneable_promissory();

        let recv_b = recv.clone();

        thread::spawn(move || send.fulfill(42));

        let joiner = thread::spawn(move || assert_eq!(42, recv_b.await_value()));

        assert_eq!(42, recv.await_value());
        joiner.join().expect("err in thread b");
    }
}
