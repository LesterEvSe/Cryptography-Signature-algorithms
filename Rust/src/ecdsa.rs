use crate::ec_ops::{EcOps, Point};
use num_bigint::BigUint;
use num_traits::One;
use rand::{rngs::OsRng, RngCore};

pub struct ECDSA {
    curve: EcOps,
    G: Point,
    n: BigUint,
}

impl ECDSA {
    pub fn new(curve: EcOps, generator: Point, generator_order: BigUint) -> ECDSA {
        ECDSA {
            curve,
            G: generator,
            n: generator_order,
        }
    }

    // Q, d
    pub fn generate_key_pair(&self) -> (Point, BigUint) {
        let mut rng = OsRng;
        let num_bytes = self.n.to_bytes_le().len();
        let mut d;

        loop {
            let mut bytes = vec![0u8; num_bytes];
            rng.fill_bytes(&mut bytes);
            d = BigUint::from_bytes_le(bytes.as_slice());

            if d >= BigUint::one() && d < self.n {
                break;
            }
        }
        (self.curve.point_mul(&d, self.G.clone()), d)
    }

    // (r, s)
    pub fn sign_message(&self, msg: &str, d: &BigUint) -> (BigUint, BigUint) {
        todo!()
    }

    pub fn verify_signature(
        &self,
        msg: &str,
        r: &BigUint,
        s: &BigUint,
        Q: &(BigUint, BigUint),
    ) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecdsa_signing_and_verification() {
        let secp256k1 = EcOps::new(BigUint::from_bytes_le(&[
            47, 252, 255, 255, 254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ]));

        let ecdsa = ECDSA::new(
            secp256k1,
            (
                BigUint::from_bytes_le(&[
                    152, 23, 248, 22, 91, 129, 242, 89, 217, 40, 206, 45, 219, 252, 155, 2, 7, 11,
                    135, 206, 149, 98, 160, 85, 172, 187, 220, 249, 126, 102, 190, 121,
                ]),
                BigUint::from_bytes_le(&[
                    184, 212, 16, 251, 143, 208, 71, 156, 25, 84, 133, 166, 72, 180, 23, 253, 168,
                    8, 17, 14, 252, 251, 164, 93, 101, 196, 163, 38, 119, 218, 58, 72,
                ]),
            ),
            BigUint::from_bytes_le(&[
                65, 65, 54, 208, 140, 94, 210, 191, 59, 160, 72, 175, 230, 220, 174, 186, 254, 255,
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            ]),
        );

        let msg = "Hello ECDSA";
        let another = "Another message";

        let (q, d) = ecdsa.generate_key_pair();
        let (r, s) = ecdsa.sign_message(msg, &d);

        assert_eq!(ecdsa.verify_signature(msg, &r, &s, &q), true);
        assert_eq!(ecdsa.verify_signature(another, &r, &s, &q), false);
        assert_eq!(ecdsa.verify_signature(msg, &d, &s, &q), false);
    }
}
