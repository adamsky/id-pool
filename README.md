# id-pool

Create and recycle integer ids using a ranged pool.

This crate is fairly minimalistic and so only deals
with single type of numerical ids. These can be either
`usize` (default), `u64`, `u32` or `u16`, chosen with
the use of appropriate crate feature.

The main exported structure [`IdPool`] can be
initialized with a custom range and then queried for
new ids that are contained within that range. During
the course of the program, ids can be returned to the
pool to be reused for subsequent id request calls.

