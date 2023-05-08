pragma circom 2.0.0;

include "../../node_modules/circomlib-ml/circuits/crypto/encrypt.circom";
include "../../node_modules/circomlib-ml/circuits/crypto/ecdh.circom";

// encrypt 1000 inputs
template encrypt1000() {
    // public inputs
    signal input public_key[2];

    // private inputs
    signal input in[1000];
    signal input private_key;

    // outputs
    signal output shared_key;
    signal output out[1001];

    component ecdh = Ecdh();

    ecdh.private_key <== private_key;
    ecdh.public_key[0] <== public_key[0];
    ecdh.public_key[1] <== public_key[1];

    component enc = EncryptBits(1000);
    enc.shared_key <== ecdh.shared_key;

    for (var i = 0; i < 1000; i++) {
        enc.plaintext[i] <== in[i];
    }

    for (var i = 0; i < 1001; i++) {
        out[i] <== enc.out[i];
    }

    shared_key <== ecdh.shared_key;
}

// fixed MultiMiMC7 with 1000 inputs
template hash1000() {
    signal input in[1000];
    signal output out;

    component mimc = MultiMiMC7(1000, 91);
    mimc.k <== 0;

    for (var i = 0; i < 1000; i++) {
        mimc.in[i] <== in[i];
    }

    out <== mimc.out;
}