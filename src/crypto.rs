//! Basic cryptographic primitives
//!
//! ALL dependencies come from [Rust Crypto] with strong security
//!
//! [Rust Crypto]: https://github.com/RustCrypto

pub mod prelude {
    pub use super::hash::Digest;
}

pub use hash::{prf, prf_int};
pub mod hash {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    pub type Digest = [u8; 32];
    type HmacSha256 = Hmac<Sha256>;

    /// HMAC-SHA256
    // TODO: We can use non-crypto-secure PRF here. Not sure about the performance improvement.
    pub fn prf(key: &[u8], data: &[u8]) -> Digest {
        let mut mac = HmacSha256::new_from_slice(key).unwrap();
        mac.update(data);
        mac.finalize().into_bytes().into()
    }

    /// HMAC-SHA256 to int `[0, b - 1]`.
    /// Because a little probability bias only causes a little bin unbalance, we simply use `x % b`.
    pub fn prf_int(key: &[u8], data: &[u8], b: u16) -> u16 {
        let res_u16 = u16::from_le_bytes(prf(key, data)[..2].try_into().unwrap());
        res_u16 % b
    }
}
