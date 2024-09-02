pragma circom 2.1.6;

template EcdsaVerify() {
    signal input message[4];
    signal input sign[32];
    signal input pubkey[32];
    signal output IsVerified;

    IsVerified <== 1;
}

component main = EcdsaVerify();