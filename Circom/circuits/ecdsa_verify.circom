pragma circom 2.1.6;

include "BigInt.circom";

template EcdsaVerify() {
    signal input message[4];
    signal input sign[2][4];
    signal input pubkey[2][4];
    signal output IsVerified;

    signal e[4] <== [65537, 0, 0, 0];
    signal order[4] <== [17562291160714782033, 13611842547513532036, 18446744073709551615, 18446744069414584320];


    component w = BigModInv(64, 4);
    w.in <== sign[1];
    w.p <== order;

    component u1 = BigMultModP(64, 4);
    u1.a <== e;
    u1.b <== w.out;
    u1.p <== order;

    component u2 = BigMultModP(64, 4);
    u2.a <== sign[0];
    u2.b <== w.out;
    u2.p <== order;

    IsVerified <== 1;
}

component main {public [pubkey]} = EcdsaVerify();