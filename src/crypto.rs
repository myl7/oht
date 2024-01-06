//! Basic cryptographic primitives
//!
//! ALL dependencies come from [Rust Crypto] with strong security
//!
//! [Rust Crypto]: https://github.com/RustCrypto

pub mod prelude {
    pub use super::hash::Digest;
}

pub use hash::{prf, prf_pow2};
pub mod hash {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    pub type Digest = [u8; 32];
    type HmacSha256 = Hmac<Sha256>;

    /// HMAC-SHA256
    pub fn prf(key: &[u8], data: &[u8]) -> Digest {
        let mut mac = HmacSha256::new_from_slice(key).unwrap();
        mac.update(data);
        mac.finalize().into_bytes().into()
    }

    /// HMAC-SHA256 to int `[0, b - 1]`.
    /// `b` must be a power of 2.
    pub fn prf_pow2(key: &[u8], data: &[u8], b: u16) -> u16 {
        assert!(b > 0 && (b & (b - 1)) == 0, "b must be a power of 2");
        let mask = b - 1;
        let res_u32 = u16::from_le_bytes(prf(key, data)[..2].try_into().unwrap());
        res_u32 & mask
    }
}
