#![doc(html_logo_url = "https://avatars3.githubusercontent.com/u/15439811?v=3&s=200",
       html_favicon_url = "https://iorust.github.io/favicon.ico",
       html_root_url = "https://iorust.github.io",
       html_playground_url = "https://play.rust-lang.org",
       issue_tracker_base_url = "https://github.com/iorust/thunks/issues")]

#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(fnbox)]

//! Asynchronous composer for Rust.

// use std::thread;
use std::sync::mpsc::{Receiver, sync_channel};
use std::boxed::{Box, FnBox};
use std::ops::FnOnce;

pub struct Thunk<T, E>(Box<Fn(Box<Fn(Result<T, E>) + Send + 'static>) + Send + 'static>);

impl<T, E> Thunk<T, E> where T: Send + 'static, E: Send + 'static {
    pub fn new<F>(task: F) -> Thunk<T, E>
    where F: Fn(Box<Fn(Result<T, E>) + Send + 'static>) + Send + 'static {
        Thunk(Box::new(task))
    }

    // pub fn seq(thunk_vec: Vec<Thunk<T, E>>) -> Thunk<Vec<T>, E> {
    //     let thunk_vec = Box::new(thunk_vec);
    //     Thunk::new(move |cb| {
    //         let mut res: Vec<T> = Vec::new();
    //         for thunk in thunk_vec.into_iter() {
    //             match thunk() {
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

    // pub fn all(thunk_vec: Vec<Thunk<T, E>>) -> Thunk<Vec<T>, E> {
    //     let thunk_vec = Box::new(thunk_vec);
    //     Thunk::new(move |cb| {
    //         let mut res: Vec<T> = Vec::new();
    //         let rx_vec: Vec<Receiver<Result<T, E>>> = thunk_vec.iter()
    //             .map(|t| t.call_thunk()).collect();
    //
    //         for rx in rx_vec.iter() {
    //             match rx.recv().unwrap() {
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

    // pub fn await(&self) -> Result<T, E> {
    //     self.call_thunk().recv().unwrap()
    // }
    //
    // fn call_thunk(&self) -> Receiver<Result<T, E>> {
    //     let (tx, rx) = sync_channel::<Result<T, E>>(1);
    //     (self.0)(Box::new(move |res| {
    //         tx.try_send(res).unwrap();
    //     }));
    //     rx
    // }
}

// impl<'a, A, R> FnOnce<A> for Box<FnBox<A, Output = R> + 'a> {
//     type Output = R;
//
//     extern "rust-call" fn call_once(self, args: A) -> R {
//         self.call_box(args)
//     }
// }

impl<T, E> FnOnce<()> for Thunk<T, E> where T: Send + 'static, E: Send + 'static {
    type Output = Result<T, E>;
    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        let (tx, rx) = sync_channel::<Result<T, E>>(1);
        let cb: Box<Fn(Result<T, E>) + Send + 'static> = Box::new(move |res| {
            tx.try_send(res).unwrap();
        });
        (self.0)(cb);
        rx.recv().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use std::boxed::{Box, FnBox};
    use super::*;

    #[test]
    fn thunk() {
        let thunk: Thunk<i32, &str> = Thunk::new(|cb| {
            thread::spawn(move || {
                thread::sleep(Duration::new(3, 0));
                cb(Ok(1));
            });
        });
        let res = thunk().unwrap();
        println!("{:?}", res);
        assert_eq!(res, 1);
    }

    // #[test]
    // fn thunk_seq() {
    //     let thunk_vec: Vec<Thunk<i32, &str>> = vec![
    //         Thunk::new(|cb| {
    //             thread::spawn(move || {
    //                 thread::sleep(Duration::new(1, 0));
    //                 cb(Ok(1));
    //             });
    //         }),
    //         Thunk::new(|cb| {
    //             thread::spawn(move || {
    //                 thread::sleep(Duration::new(1, 0));
    //                 cb(Ok(2));
    //             });
    //         }),
    //         Thunk::new(|cb| {
    //             thread::spawn(move || {
    //                 thread::sleep(Duration::new(1, 0));
    //                 cb(Ok(3));
    //             });
    //         })
    //     ];
    //     let res = Thunk::seq(thunk_vec).await().unwrap();
    //     assert_eq!(res, vec![1, 2, 3]);
    // }
    //
    // #[test]
    // fn thunk_all() {
    //     let thunk_vec: Vec<Thunk<i32, &str>> = vec![
    //         Thunk::new(|cb| {
    //             thread::spawn(move || {
    //                 thread::sleep(Duration::new(1, 0));
    //                 cb(Ok(1));
    //             });
    //         }),
    //         Thunk::new(|cb| {
    //             thread::spawn(move || {
    //                 thread::sleep(Duration::new(1, 0));
    //                 cb(Ok(2));
    //             });
    //         }),
    //         Thunk::new(|cb| {
    //             thread::spawn(move || {
    //                 thread::sleep(Duration::new(1, 0));
    //                 cb(Ok(3));
    //             });
    //         })
    //     ];
    //     let res = Thunk::all(thunk_vec).await().unwrap();
    //     assert_eq!(res, vec![1, 2, 3]);
    // }
}
