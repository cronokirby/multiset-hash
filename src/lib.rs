use curve25519_dalek::ristretto;
use digest::{Digest, FixedOutput, Reset, Update, consts::{U32, U64}, generic_array::GenericArray};

#[derive(Clone, Default)]
struct RistrettoHash<H> {
    hash: H,
    acc: ristretto::RistrettoPoint,
}

impl <H: Digest<OutputSize = U64> + Default> RistrettoHash<H> {
    pub fn add(data: impl AsRef<u8>, multiplicity: u64) {
        unimplemented!()
    }

    pub fn end_update(multiplicity: u64) {
        unimplemented!()
    }
}

impl <H> FixedOutput for RistrettoHash<H> {
    type OutputSize = U32;

    fn finalize_into(self, out: &mut GenericArray<u8, Self::OutputSize>) {
        unimplemented!()
    }

    fn finalize_into_reset(&mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
        unimplemented!()
    }
}

impl <H: Reset> Reset for RistrettoHash<H> {
    fn reset(&mut self) {
        unimplemented!()
    }
}

impl <H: Update> Update for RistrettoHash<H> {
    fn update(&mut self, data: impl AsRef<[u8]>) {
        unimplemented!()
    }
}
