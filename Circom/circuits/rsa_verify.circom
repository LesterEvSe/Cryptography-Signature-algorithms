pragma circom 2.1.6;


include "helper.circom";
include "../node_modules/circomlib/circuits/sha256/sha256.circom";
include "../node_modules/circomlib/circuits/bitify.circom";

// Bits for Sha256
template RsaVerify(MSG_BYTES, MSG_BITS, DWORDS) {
    signal input message[4];
    signal input sign[DWORDS];
    signal input pubkey[DWORDS];
    signal output IsVerified;

    component res = PowerMod(64, 32, 17);
    for (var i = 0; i < 32; i++) {
        res.base[i] <== sign[i];
        res.modulus[i] <== pubkey[i];
    }

    for (var i = 0; i < 4; i++) {
        message[i] === res.out[i];
    }
    IsVerified <== 1;
}

component main {public [pubkey]} = RsaVerify(11, 11*8, 32);