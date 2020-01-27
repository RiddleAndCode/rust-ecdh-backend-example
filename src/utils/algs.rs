use ring::{aead, agreement, hkdf, signature};

pub static ECDH_ALG: &'static agreement::Algorithm = &agreement::ECDH_P256;
pub static SYM_ENC_ALG: &'static aead::Algorithm = &aead::AES_128_GCM;
pub static ECDSA_ALG_SIGNING: &'static signature::EcdsaSigningAlgorithm =
    &signature::ECDSA_P256_SHA256_FIXED_SIGNING;
pub static ECDSA_ALG: &'static signature::EcdsaVerificationAlgorithm =
    &signature::ECDSA_P256_SHA256_FIXED;
pub static HKDF_ALG: hkdf::Algorithm = hkdf::HKDF_SHA256;
