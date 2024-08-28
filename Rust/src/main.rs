mod rsa;
use rsa::SigAlg;

pub fn rsa_process(bits: usize) {
    let (e, d, n) = SigAlg::generate_key_pair(bits);
    let msg = "Message for RSA signing";

    let sign = SigAlg::sign_message(msg, &n, &d);
    assert!(SigAlg::verify_signature(msg, &sign, &n, &e));
}

fn main() {
    rsa_process(1024);
}
