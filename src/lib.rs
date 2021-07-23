use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use digest::{
    consts::{U32, U64},
    generic_array::GenericArray,
    Digest, FixedOutput, Reset, Update,
};

/// RistrettoHash represents a hash function for multi-sets.
///
/// A multi-set is a collection of objects, where any given object may appear
/// multiple times. This hash function will accept the objects in different
/// orders, and with different groupings, but always return the same result
/// for the same multi-set.
///
/// This allows incrementally calculating such a multi-set hash without keeping
/// the entire set in memory.
///
/// This struct is parameterized by a hash function with 512 bits of output.
/// For example, SHA-512.
///
/// This is called `RistrettoHash`, because this function internally uses
/// the Ristretto group to implement its commutative hashing.
///
/// # Examples
///
/// In the usual case, you add in different byte slices, with multiplicity For example,
/// to hash the set `{cat, cat, dog, dog}`, you could do:
///
/// ```
/// # use multiset_hash::RistrettoHash;
/// use digest::Digest;
/// use sha2::Sha512;
///
/// let mut hash = RistrettoHash::<Sha512>::default();
/// hash.add(b"cat", 2);
/// hash.add(b"dog", 2);
/// ```
///
/// This is equivalent to hashing the same number of objects, with a different
/// grouping and order:
///
/// ```
/// # use multiset_hash::RistrettoHash;
/// # use sha2::Sha512;
/// use digest::Digest;
///
/// let mut hash = RistrettoHash::<Sha512>::default();
/// hash.add(b"dog", 1);
/// hash.add(b"cat", 1);
/// hash.add(b"cat", 1);
/// hash.add(b"dog", 1);
/// ```
///
/// You can also hash objects in multiple pieces using the `update` method:
///
/// ```
/// # use multiset_hash::RistrettoHash;
/// # use sha2::Sha512;
/// use digest::Digest;
///
/// let mut hash = RistrettoHash::<Sha512>::default();
/// hash.update(b"d");
/// hash.update(b"og");
/// hash.end_update(2);
/// hash.add(b"cat", 2);
/// ```
///
/// If you use `update`, you must call `end_update` before adding another object,
/// or calling `finalize` to get the output of the hash function.
#[derive(Clone, Default)]
pub struct RistrettoHash<H> {
    hash: H,
    // This flag will get set after we call update, and indicates that
    // we need to finish that update with an explicit call to `end_update`.
    updating: bool,
    acc: RistrettoPoint,
}

impl<H: Digest<OutputSize = U64> + Default> RistrettoHash<H> {
    /// This function adds a complete object to the hash.
    ///
    /// This function takes a multiplicity, which is equivalent to calling the function
    /// multiple times, but is much more efficient.
    pub fn add(&mut self, data: impl AsRef<[u8]>, multiplicity: u64) {
        if self.updating {
            panic!("add called before end_update");
        }
        self.hash.update(data);
        self.end_update(multiplicity);
    }

    /// This function should be called to mark the end of an object provided with `update`.
    ///
    /// This must always be called after calls to `update`, otherwise panics will happen
    /// when finalizing or adding new objects.
    ///
    /// If called without any prior calls to `update`, this function is equivalent
    /// to calling `add` with an empty slice.
    pub fn end_update(&mut self, multiplicity: u64) {
        self.updating = false;

        let old = std::mem::replace(&mut self.hash, H::default());
        let h_point = RistrettoPoint::from_hash(old);
        self.acc += Scalar::from(multiplicity) * h_point;
    }
}

impl<H: Reset> FixedOutput for RistrettoHash<H> {
    type OutputSize = U32;

    fn finalize_into(self, out: &mut GenericArray<u8, Self::OutputSize>) {
        if self.updating {
            panic!("end_update not called before finalizing");
        }
        out.copy_from_slice(&self.acc.compress().as_bytes()[..]);
    }

    fn finalize_into_reset(&mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
        if self.updating {
            panic!("end_update not called before finalizing");
        }
        out.copy_from_slice(&self.acc.compress().as_bytes()[..]);
        self.reset();
    }
}

impl<H: Reset> Reset for RistrettoHash<H> {
    fn reset(&mut self) {
        self.hash.reset();
        self.updating = false;
        self.acc = RistrettoPoint::default();
    }
}

impl<H: Update> Update for RistrettoHash<H> {
    /// update hashes in part of an object.
    ///
    /// This method is used to hash an object in multiple different parts.
    /// These updates must be finished by calling `end_update` to mark the end
    /// of the object, and its multiplicity.
    ///
    /// Failing to call `end_update` before adding a new object with `add` or
    /// finalizing the hash will panic.
    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.updating = true;
        self.hash.update(data);
    }
}

#[cfg(test)]
mod test {
    use sha2::Sha512;

    use super::RistrettoHash;
    use digest::Digest;

    #[test]
    fn test_add_with_multiplicity() {
        let data = b"test data";

        let mut hash1 = RistrettoHash::<Sha512>::default();
        let mut hash2 = hash1.clone();

        hash1.add(data, 3);
        hash2.add(data, 1);
        hash2.add(data, 1);
        hash2.add(data, 1);

        let output1 = hash1.finalize();
        let output2 = hash2.finalize();
        assert_eq!(output1, output2)
    }

    #[test]
    fn test_hash_commutative() {
        let data_a = b"test data A";
        let data_b = b"test data B";

        let mut hash1 = RistrettoHash::<Sha512>::default();
        let mut hash2 = hash1.clone();

        hash1.add(data_a, 1);
        hash1.add(data_b, 1);

        hash2.add(data_b, 1);
        hash2.add(data_a, 1);

        let output1 = hash1.finalize();
        let output2 = hash2.finalize();
        assert_eq!(output1, output2)
    }

    #[test]
    fn test_partial_updates() {
        let mut hash1 = RistrettoHash::<Sha512>::default();
        let mut hash2 = hash1.clone();

        hash1.add("the full data", 3);
        hash2.update("the");
        hash2.update(" full");
        hash2.update(" data");
        hash2.end_update(3);

        let output1 = hash1.finalize();
        let output2 = hash2.finalize();
        assert_eq!(output1, output2)
    }

    #[test]
    #[should_panic]
    fn test_add_before_end_update_panics() {
        let mut hash = RistrettoHash::<Sha512>::default();
        hash.update("some data");
        hash.add("more data", 1);
    }

    #[test]
    #[should_panic]
    fn test_finalize_before_end_update_panics() {
        let mut hash = RistrettoHash::<Sha512>::default();
        hash.update("some data");
        hash.finalize();
    }
}
