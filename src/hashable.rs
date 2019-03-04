use std::ops::Deref;

pub type Hash = Vec<u8>;
/*
    Implement hash for:
Vec<T: Hashable>
u8
u32
u64
u128
String
*/
pub trait Hashable {
    //Converted to bytes in Big-endian notation
    fn bytes (&self) -> Hash;
    //Generate SHA256 hash based on the byte representation of the object
    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}

impl<T: Hashable> Hashable for Vec<T> {
    fn bytes(&self) -> Hash {
        self.iter()
            .flat_map(|data| data.bytes())
            .collect::<Hash>()
    }
}

impl Hashable for u8 {
    fn bytes(&self) -> Hash {
        vec![self.clone()]
    }
}

impl Hashable for u32 {
    fn bytes(&self) -> Hash {
        self.to_be_bytes().to_vec()
    }
}
impl Hashable for u64 {
    fn bytes(&self) -> Hash {
        self.to_be_bytes().to_vec()
    }
}
impl Hashable for u128 {
    fn bytes(&self) -> Hash {
        self.to_be_bytes().to_vec()
    }
}
impl Hashable for String {
    fn bytes(&self) -> Hash {
        self.as_bytes().to_vec()
    }
}