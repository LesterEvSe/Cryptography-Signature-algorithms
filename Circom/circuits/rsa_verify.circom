pragma circom 2.1.9;

template RsaVerify(MSG_LEN, SIGN_LEN, BYTES) {
    signal input message[MSG_LEN];
    signal input sign[SIGN_LEN];
    signal input pubkey[2][BYTES];
    signal output out;

    signal temp <== message[0] * sign[0];
    out <== temp * pubkey[1][0];
}

component main {public [pubkey]} = RsaVerify(3, 3, 3);