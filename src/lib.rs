
// use std::thread;
use std::sync::{mpsc};
use std::boxed::{Box};
// use std::vec::Vec;

pub struct Thunk<T, E>(Box<Fn(&(Fn(Result<T, E>) + 'static + Send)) + 'static>);

impl<T, E> Thunk<T, E> where T: 'static + Sync + Send, E: 'static + Sync + Send {
    pub fn new<F>(task: F) -> Thunk<T, E>
        where F: Fn(&(Fn(Result<T, E>) + 'static + Send)) + 'static
    {
        Thunk(Box::new(task))
    }

    pub fn await(&self) -> Result<T, E> {
        self.call_thunk()
    }

    fn call_thunk(&self) -> Result<T, E> {
        let (tx, rx) = mpsc::channel::<Result<T, E>>();
        (self.0)(&move |res| {
            tx.send(res).unwrap();
        });
        rx.recv().unwrap()
    }
}

#[cfg(test)]
mod tests {
    // use std::thread;
    use super::*;

    #[test]
    fn thunk() {
        let t: Thunk<i32, i32> = Thunk::new(|cb| {
            // thread::spawn(move|| {
            //     cb(Ok(1 as i32));
            // });
            cb(Ok(1));
        });
        let res = t.await().unwrap();
        // println!("{:?}", res);
        assert_eq!(res, 1);
    }
}
