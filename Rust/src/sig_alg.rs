use num_bigint::BigUint;

pub trait SigAlg {
    type KeyPair;
    type Signature;

    fn generate_key_pair(bits: usize) -> Self::KeyPair;
    fn sign_message(msg: &str, n: &BigUint, d: &BigUint) -> Self::Signature;
    fn verify_signature(msg: &str, sign: &BigUint, n: &BigUint, e: &BigUint) -> bool;
}
