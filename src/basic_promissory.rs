use std::sync::{Arc, Condvar, Mutex};

pub(crate) struct Promissory<T> {
    pub(crate) mutex: Mutex<Option<T>>,
    pub(crate) cond: Condvar,
}

pub struct Fulfiller<T>(pub(crate) Arc<Promissory<T>>);
pub struct Awaiter<T>(Arc<Promissory<T>>);

pub fn promissory<T>() -> (Fulfiller<T>, Awaiter<T>)
where
    T: Send,
{
    let p = Arc::new(Promissory {
        mutex: Mutex::new(None),
        cond: Condvar::new(),
    });

    (Fulfiller(p.clone()), Awaiter(p))
}

impl<T> Fulfiller<T>
where
    T: Send,
{
    pub fn fulfill(self, t: T) {
        *(self.0.mutex.lock().expect("broken lock")) = Some(t);
        self.0.cond.notify_all() // All because we use this one impl for this and clone
    }
}

impl<T> Awaiter<T>
where
    T: Send,
{
    pub fn await_value(self) -> T {
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
