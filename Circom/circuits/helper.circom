pragma circom 2.1.9;

include "../node_modules/circomlib/circuits/comparators.circom";

template CheckBytes(N) {
    signal input in[N];

    component lte255[N];
    component gte0[N];

    for (var i = 0; i < N; i++) {
        lte255[i] = LessEqThan(8);
        lte255[i].in[0] <== in[i];
        lte255[i].in[1] <== 255;

        gte0[i] = LessEqThan(8);
        gte0[i].in[0] <== 0;
        gte0[i].in[1] <== in[i];
        1 === gte0[i].out * lte255[i].out;
    }
}