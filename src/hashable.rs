pub type Hash = Vec<u8>;

pub trait Hashable {
    //Converted to bytes in Big-endian notation
    fn bytes (&self) -> Hash;
    //Generate SHA256 hash based on the byte representation of the object
    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}