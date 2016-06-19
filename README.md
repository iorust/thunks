thunks
====
Asynchronous composer for Rust.

[![Crates version][version-image]][version-url]
[![Build Status][travis-image]][travis-url]
[![Coverage Status][coveralls-image]][coveralls-url]
[![Crates downloads][downloads-image]][downloads-url]

## API
### Documentation https://iorust.github.io/thunks/thunks

```Rust
extern crate thunks;
use thunks::Thunk;
```

```
pub struct Thunk<T, E>(_)
```

### impl<T, E> Thunk<T, E> where T: Send + 'static, E: Send + 'static
#### fn new<F>(task: F) -> Thunk<T, E> where F: Fn(Box<Fn(Result<T, E>) + Send + 'static>) + Send + 'static
#### fn await(&self) -> Result<T, E>


[version-image]: https://img.shields.io/crates/v/thunks.svg
[version-url]: https://crates.io/crates/thunks

[travis-image]: http://img.shields.io/travis/iorust/thunks.svg
[travis-url]: https://travis-ci.org/iorust/thunks

[coveralls-image]: https://coveralls.io/repos/github/iorust/thunks/badge.svg?branch=master
[coveralls-url]: https://coveralls.io/github/iorust/thunks?branch=master

[downloads-image]: https://img.shields.io/crates/d/thunks.svg
[downloads-url]: https://crates.io/crates/thunks
