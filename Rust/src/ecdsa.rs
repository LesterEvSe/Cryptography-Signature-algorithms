use crate::sig_alg::SigAlg;
use num_bigint::BigUint;
use num_primes::Generator;
use num_traits::One;

pub struct ECDSA;

impl SigAlg for ECDSA {
    type KeyPair = (BigUint, BigUint); // Point on curve
    type Signature = (BigUint, BigUint); // two scalars

    fn generate_key_pair(bits: usize) -> Self::KeyPair {
        todo!()
    }
    fn sign_message(msg: &str, n: &BigUint, d: &BigUint) -> Self::Signature {
        todo!()
    }
    fn verify_signature(msg: &str, sign: &BigUint, n: &BigUint, e: &BigUint) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
