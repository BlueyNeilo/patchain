pub trait Hashable {
    //Converted to bytes in Big-endian notation
    fn bytes (&self) -> Vec<u8>;
    //Generate SHA256 hash based on the byte representation of the object
    fn hash (&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}