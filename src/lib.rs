#![doc(html_logo_url = "https://avatars3.githubusercontent.com/u/15439811?v=3&s=200",
       html_favicon_url = "https://iorust.github.io/favicon.ico",
       html_root_url = "https://iorust.github.io",
       html_playground_url = "https://play.rust-lang.org",
       issue_tracker_base_url = "https://github.com/iorust/thunks/issues")]

//! Asynchronous composer for Rust.

// use std::thread;
use std::sync::{mpsc};
use std::boxed::{Box};

pub struct Thunk<T, E>(Box<Fn(Box<Fn(Result<T, E>) + Send + 'static>) + Send + 'static>);

impl<T, E> Thunk<T, E> where T: Send + 'static, E: Send + 'static {
    pub fn new<F>(task: F) -> Thunk<T, E>
    where F: Fn(Box<Fn(Result<T, E>) + Send + 'static>) + Send + 'static {
        Thunk(Box::new(task))
    }

    // pub fn seq(list: Vec<Thunk<T, E>>) -> Thunk<Vec<T>, E> {
    //     Thunk::new(|cb| {
    //         let mut res: Vec<T> = Vec::new();
    //         for t in list {
    //             match t.await() {
    //                 Ok(val) => res.push(val),
    //                 Err(err) => {
    //                     cb(Err(err));
    //                     return;
    //                 }
    //             }
    //         }
    //         cb(Ok(res));
    //     })
    // }

    pub fn await(&self) -> Result<T, E> {
        self.call_thunk()
    }

    fn call_thunk(&self) -> Result<T, E> {
        let (tx, rx) = mpsc::sync_channel::<Result<T, E>>(1);
        (self.0)(Box::new(move |res| {
            tx.try_send(res).unwrap();
        }));
        rx.recv().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use super::*;

    #[test]
    fn thunk() {
        let t: Thunk<i32, &str> = Thunk::new(|cb| {
            thread::spawn(move || {
                thread::sleep(Duration::new(3, 0));
                cb(Ok(1));
            });
        });
        let res = t.await().unwrap();
        // println!("{:?}", res);
        assert_eq!(res, 1);
    }
}
