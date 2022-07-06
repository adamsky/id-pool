# id-pool

Create and recycle integer ids using a ranged pool.

```rust
// create a new id pool with an available range
let mut pool = IdPool::new_ranged(1..10);
// request ids from the pool
let id1 = pool.request_id()); // 1
let id2 = pool.request_id()); // 2
let id3 = pool.request_id()); // 3
// return arbitrary id back into the pool
pool.return_id(2)?;
// recycle the returned id during subsequent request
let id4 = pool.request_id()); // 2
let id5 = pool.request_id()); // 4
```

This crate is fairly minimalistic and so currently only
deals with single type of numerical ids. These can be
either `usize` (default), `u64`, `u32` or `u16`, chosen
with the use of appropriate crate feature.

The main exported structure `IdPool` can be
initialized with a custom range and then queried for
new ids that are contained within that range. During
the course of the program, ids can be returned to the
pool to be reused for subsequent id request calls.

