mod ec_ops;
mod ecdsa;
mod rsa;

use crate::ec_ops::EcOps;
use crate::ecdsa::ECDSA;
use crate::rsa::RSA;
use num_bigint::BigUint;

pub fn rsa_process(bits: usize) {
    let (e, d, n) = RSA::generate_key_pair(bits);
    let msg = "Message for RSA signing";

    let sign = RSA::sign_message(msg, &n, &d);
    assert!(RSA::verify_signature(msg, &sign, &n, &e));
}

pub fn ecdsa_process() {
    let secp256k1 = EcOps::new(BigUint::from_bytes_le(&[
        47, 252, 255, 255, 254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ]));

    let ecdsa = ECDSA::new(
        secp256k1,
        (
            BigUint::from_bytes_le(&[
                152, 23, 248, 22, 91, 129, 242, 89, 217, 40, 206, 45, 219, 252, 155, 2, 7, 11, 135,
                206, 149, 98, 160, 85, 172, 187, 220, 249, 126, 102, 190, 121,
            ]),
            BigUint::from_bytes_le(&[
                184, 212, 16, 251, 143, 208, 71, 156, 25, 84, 133, 166, 72, 180, 23, 253, 168, 8,
                17, 14, 252, 251, 164, 93, 101, 196, 163, 38, 119, 218, 58, 72,
            ]),
        ),
        BigUint::from_bytes_le(&[
            65, 65, 54, 208, 140, 94, 210, 191, 59, 160, 72, 175, 230, 220, 174, 186, 254, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ]),
    );

    let (q, d) = ecdsa.generate_key_pair();
    let msg = "ECDSA signature here!";
    let (r, s) = ecdsa.sign_message(msg, &d);
    assert!(ecdsa.verify_signature(msg, &r, &s, &q));
}

fn main() {
    rsa_process(1024);
    ecdsa_process();
}
