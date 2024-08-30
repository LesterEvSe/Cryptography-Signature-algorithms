pragma circom 2.1.9;

include "helper.circom";

template RsaVerify(MSG_LEN, SIGN_LEN, BYTES) {
    signal input message[MSG_LEN];
    signal input sign[SIGN_LEN];
    signal input pubkey[2][BYTES];
    signal output out;

    component msg_check = CheckBytes(MSG_LEN);
    component sign_check = CheckBytes(SIGN_LEN);
    component pubkey_check[2];
    
    for (var i = 0; i < 2; i++) {
        pubkey_check[i] = CheckBytes(BYTES);
        pubkey_check[i].in <== pubkey[i];
    }
    msg_check.in <== message;
    sign_check.in <== sign;

    signal temp <== message[0] * sign[0];
    out <== temp * pubkey[1][0];
}

component main {public [pubkey]} = RsaVerify(5, 3, 3);