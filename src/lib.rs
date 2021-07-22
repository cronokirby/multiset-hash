use digest::{FixedOutput, Reset, Update, consts::U32, generic_array::GenericArray};

#[derive(Clone, Default)]
struct RistrettoHash {}

impl RistrettoHash {
    pub fn add(data: impl AsRef<u8>, multiplicity: u64) {
        unimplemented!()
    }

    pub fn end_update(multiplicity: u64) {
        unimplemented!()
    }
}

impl FixedOutput for RistrettoHash {
    type OutputSize = U32;

    fn finalize_into(self, out: &mut GenericArray<u8, Self::OutputSize>) {
        unimplemented!()
    }

    fn finalize_into_reset(&mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
        unimplemented!()
    }
}

impl Reset for RistrettoHash {
    fn reset(&mut self) {
        unimplemented!()
    }
}

impl Update for RistrettoHash {
    fn update(&mut self, data: impl AsRef<[u8]>) {
        unimplemented!()
    }
}
