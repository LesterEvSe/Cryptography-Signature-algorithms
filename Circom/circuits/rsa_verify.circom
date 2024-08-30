pragma circom 2.1.9;

include "helper.circom";
include "../node_modules/circomlib/circuits/sha256/sha256.circom";
include "../node_modules/circomlib/circuits/bitify.circom";

// Bits for Sha256
template RsaVerify(MSG_BYTES, MSG_BITS, SIGN_LEN, BYTES) {
    signal input message[MSG_BYTES];
    signal input sign[SIGN_LEN];
    signal input pubkey[2][BYTES];
    signal output out[256];

    // Check bytes in range [0; 255]
    component msg_check = CheckBytes(MSG_BYTES);
    component sign_check = CheckBytes(SIGN_LEN);
    component pubkey_check[2];

    for (var i = 0; i < 2; i++) {
        pubkey_check[i] = CheckBytes(BYTES);
        pubkey_check[i].in <== pubkey[i];
    }
    msg_check.in <== message;
    sign_check.in <== sign;

    // Get sha256 bits in Big-Endian format
    component hashed = Sha256(MSG_BITS);
    component n2b[MSG_BYTES];

    for (var i = 0; i < MSG_BYTES; i++) {
        n2b[i] = Num2Bits(8);
        n2b[i].in <== message[i];

        hashed.in[i*8 + 0] <== n2b[i].out[7];
        hashed.in[i*8 + 1] <== n2b[i].out[6];
        hashed.in[i*8 + 2] <== n2b[i].out[5];
        hashed.in[i*8 + 3] <== n2b[i].out[4];
        hashed.in[i*8 + 4] <== n2b[i].out[3];
        hashed.in[i*8 + 5] <== n2b[i].out[2];
        hashed.in[i*8 + 6] <== n2b[i].out[1];
        hashed.in[i*8 + 7] <== n2b[i].out[0];
    }
    out <== hashed.out;
}

component main {public [pubkey]} = RsaVerify(3, 3*8, 4, 5);