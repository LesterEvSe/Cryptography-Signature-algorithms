mod ec_ops;
mod ecdsa;
mod rsa;
mod sig_alg;

use crate::ecdsa::ECDSA;
use crate::rsa::RSA;
use crate::sig_alg::SigAlg;

pub fn rsa_process(bits: usize) {
    let (e, d, n) = RSA::generate_key_pair(bits);
    let msg = "Message for RSA signing";

    let sign = RSA::sign_message(msg, &n, &d);
    assert!(RSA::verify_signature(msg, &sign, &n, &e));
}

pub fn ecdsa_process() {}

fn main() {
    rsa_process(1024);
}
