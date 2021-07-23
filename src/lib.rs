use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use digest::{
    consts::{U32, U64},
    generic_array::GenericArray,
    Digest, FixedOutput, Reset, Update,
};

#[derive(Clone, Default)]
pub struct RistrettoHash<H> {
    hash: H,
    updating: bool,
    acc: RistrettoPoint,
}

impl<H: Digest<OutputSize = U64> + Default> RistrettoHash<H> {
    pub fn add(&mut self, data: impl AsRef<[u8]>, multiplicity: u64) {
        if self.updating {
            panic!("add called before end_update");
        }
        self.hash.update(data);
        self.end_update(multiplicity);
    }

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
        out.copy_from_slice(&self.acc.compress().as_bytes()[..]);
    }

    fn finalize_into_reset(&mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
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
    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.updating = true;
        self.hash.update(data);
    }
}

#[cfg(test)]
mod test {
    use sha2::Sha512;

    use super::*;

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
}
