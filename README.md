# multiset-hash

This crate provides a hash function for multi-sets, which can be updated
incrementally. This function takes in a collection of objects, with
each object potentially appearing multiple times, and outputs a digest. This result
depends only on which objects are in the set, and how many times they appear.
Calling the function with the objects in a different order, or grouped in
a different way, will not change the result of the function.

This allows the function to be incrementally fed with new objects as they arrive,
without having to keep the entire set in memory.

# Implementation

This crate was inspired by [a 2003 paper](https://link.springer.com/chapter/10.1007%2F978-3-540-40061-5_12)
from Clarke et al. They define a function for hashing objects `x` with multiplicity `m(x)` as:

```
sum_x m(x) * H(x)
```

Where `H` is a hash function into a suitable abelian group. In their case, they used
the multiplicative group of a finite field. Since then, we've developed much more elegant
(in my opinion) Elliptic Curve groups. This crate uses [Ristretto](https://ristretto.group/)
as implemented by the [curve25519-dalek](https://docs.rs/curve25519-dalek) crate.

# Examples

In the usual case, you add in different byte slices, with multiplicity For example,
to hash the set `{cat, cat, dog, dog}`, you could do:

```rust
# use multiset_hash::RistrettoHash;
use digest::Digest;
use sha2::Sha512;

let mut hash = RistrettoHash::<Sha512>::default();
hash.add(b"cat", 2);
hash.add(b"dog", 2);
```

This is equivalent to hashing the same number of objects, with a different
grouping and order:

```rust
use digest::Digest;

let mut hash = RistrettoHash::<Sha512>::default();
hash.add(b"dog", 1);
hash.add(b"cat", 1);
hash.add(b"cat", 1);
hash.add(b"dog", 1);
```

You can also hash objects in multiple pieces using the `update` method:

```rust
use digest::Digest;

let mut hash = RistrettoHash::<Sha512>::default();
hash.update(b"d");
hash.update(b"og");
hash.end_update(2);
hash.add(b"cat", 2);
```

If you use `update`, you must call `end_update` before adding another object,
or calling `finalize` to get the output of the hash function.
