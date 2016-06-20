thunks
====
Asynchronous composer for Rust.

[![Crates version][version-image]][version-url]
[![Build Status][travis-image]][travis-url]
[![Coverage Status][coveralls-image]][coveralls-url]
[![Crates downloads][downloads-image]][downloads-url]

## Demo

**primitive thunk:**
```rust
let thunk: Thunk<i32, &str> = Thunk::new(|cb| {
    thread::spawn(move || {
        thread::sleep(Duration::new(3, 0));
        cb(Ok(1));
    });
});
let res = thunk.await().unwrap();
assert_eq!(res, 1);
```

**Sequence control:**
```rust
let thunk_vec: Vec<Thunk<i32, &str>> = vec![
    Thunk::new(|cb| {
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            cb(Ok(1));
        });
    }),
    Thunk::new(|cb| {
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            cb(Ok(2));
        });
    }),
    Thunk::new(|cb| {
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            cb(Ok(3));
        });
    })
];
let res = Thunk::seq(thunk_vec).await().unwrap();
assert_eq!(res, vec![1, 2, 3]);
```

**Parallel control:**
```rust
let thunk_vec: Vec<Thunk<i32, &str>> = vec![
    Thunk::new(|cb| {
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            cb(Ok(1));
        });
    }),
    Thunk::new(|cb| {
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            cb(Ok(2));
        });
    }),
    Thunk::new(|cb| {
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            cb(Ok(3));
        });
    })
];
let res = Thunk::all(thunk_vec).await().unwrap();
assert_eq!(res, vec![1, 2, 3]);
```

## JavaScript Version: https://github.com/thunks/thunks

## API
### Documentation https://iorust.github.io/thunks/thunks

```rust
extern crate thunks;
use thunks::Thunk;
```

### Struct thunks::Thunk
```rust
pub struct Thunk<T, E>(_)
```

### Methods
#### `impl<T, E> Thunk<T, E> where T: Send + 'static, E: Send + 'static`

```rust
fn new<F>(task: F) -> Thunk<T, E>
where F: Fn(Box<Fn(Result<T, E>) + Send + 'static>) + Send + 'static
```

```rust
fn seq(thunk_vec: Vec<Thunk<T, E>>) -> Thunk<Vec<T>, E>
```

```rust
fn all(thunk_vec: Vec<Thunk<T, E>>) -> Thunk<Vec<T>, E>
```

```rust
fn await(&self) -> Result<T, E>
```

[version-image]: https://img.shields.io/crates/v/thunks.svg
[version-url]: https://crates.io/crates/thunks

[travis-image]: http://img.shields.io/travis/iorust/thunks.svg
[travis-url]: https://travis-ci.org/iorust/thunks

[coveralls-image]: https://coveralls.io/repos/github/iorust/thunks/badge.svg?branch=master
[coveralls-url]: https://coveralls.io/github/iorust/thunks?branch=master

[downloads-image]: https://img.shields.io/crates/d/thunks.svg
[downloads-url]: https://crates.io/crates/thunks
