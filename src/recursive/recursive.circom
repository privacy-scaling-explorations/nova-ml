pragma circom 2.0.3;

include "../utils/model.circom";
include "../utils/cid.circom";
include "../utils/encrypt.circom";
include "../../node_modules/circomlib/circuits/poseidon.circom";

template Recursive () {
    signal input step_in[3]; // modelHash, runningDataHash, runningOutHash

    signal output step_out[3];
    
    // private inputs
    signal input in[797*8];
    signal input conv2d_weights[3][3][1][4];
    signal input conv2d_bias[4];
    signal input batch_normalization_a[4];
    signal input batch_normalization_b[4];
    signal input conv2d_1_weights[3][3][4][16];
    signal input conv2d_1_bias[16];
    signal input batch_normalization_1_a[16];
    signal input batch_normalization_1_b[16];
    signal input dense_weights[16][10];
    signal input dense_bias[10];
    
    // components
    component model  = Model();
    component pixel = getPixels();
    component cid = getCid();
    component poseidon[3];

    // intermediate signals
    signal cid_out[2];
    signal model_out;

    for (var i0 = 0; i0 < 797*8; i0++) {
        pixel.in[i0] <== in[i0];
        cid.in[i0] <== in[i0];
    }

    // get cid
    for (var i0 = 0; i0 < 2; i0++) {
        cid_out[i0] <== cid.out[i0];
    }

    for (var i0 = 0; i0 < 28; i0++) {
        for (var i1 = 0; i1 < 28; i1++) {
            for (var i2 = 0; i2 < 1; i2++) {
                model.in[i0][i1][i2] <== pixel.out[i0][i1][i2];
    }}}

    for (var i0 = 0; i0 < 3; i0++) {
        for (var i1 = 0; i1 < 3; i1++) {
            for (var i2 = 0; i2 < 1; i2++) {
                for (var i3 = 0; i3 < 4; i3++) {
                    model.conv2d_weights[i0][i1][i2][i3] <== conv2d_weights[i0][i1][i2][i3];
    }}}}
    for (var i0 = 0; i0 < 4; i0++) {
        model.conv2d_bias[i0] <== conv2d_bias[i0];
    }

    for (var i0 = 0; i0 < 4; i0++) {
        model.batch_normalization_a[i0] <== batch_normalization_a[i0];
    }
    for (var i0 = 0; i0 < 4; i0++) {
        model.batch_normalization_b[i0] <== batch_normalization_b[i0];
    }

    for (var i0 = 0; i0 < 3; i0++) {
        for (var i1 = 0; i1 < 3; i1++) {
            for (var i2 = 0; i2 < 4; i2++) {
                for (var i3 = 0; i3 < 16; i3++) {
                    model.conv2d_1_weights[i0][i1][i2][i3] <== conv2d_1_weights[i0][i1][i2][i3];
    }}}}
    for (var i0 = 0; i0 < 16; i0++) {
        model.conv2d_1_bias[i0] <== conv2d_1_bias[i0];
    }

    for (var i0 = 0; i0 < 16; i0++) {
        model.batch_normalization_1_a[i0] <== batch_normalization_1_a[i0];
    }
    for (var i0 = 0; i0 < 16; i0++) {
        model.batch_normalization_1_b[i0] <== batch_normalization_1_b[i0];
    }

    for (var i0 = 0; i0 < 16; i0++) {
        for (var i1 = 0; i1 < 10; i1++) {
            model.dense_weights[i0][i1] <== dense_weights[i0][i1];
    }}
    for (var i0 = 0; i0 < 10; i0++) {
        model.dense_bias[i0] <== dense_bias[i0];
    }

    model_out <== model.out[0];

    // hash model weights

    component mimc = hash1000();
    var idx = 0;

    for (var i0 = 0; i0 < 3; i0++) {
        for (var i1 = 0; i1 < 3; i1++) {
            for (var i2 = 0; i2 < 1; i2++) {
                for (var i3 = 0; i3 < 4; i3++) {
                    mimc.in[idx] <== conv2d_weights[i0][i1][i2][i3];
                    idx++;
    }}}}

    for (var i0 = 0; i0 < 4; i0++) {
        mimc.in[idx] <== conv2d_bias[i0];
        idx++;
    }

    for (var i0 = 0; i0 < 4; i0++) {
            mimc.in[idx] <== batch_normalization_a[i0];
            idx++;
    }
    for (var i0 = 0; i0 < 4; i0++) {
        mimc.in[idx] <== batch_normalization_b[i0];
        idx++;
    }

    for (var i0 = 0; i0 < 3; i0++) {
        for (var i1 = 0; i1 < 3; i1++) {
            for (var i2 = 0; i2 < 4; i2++) {
                for (var i3 = 0; i3 < 16; i3++) {
                    mimc.in[idx] <== conv2d_1_weights[i0][i1][i2][i3];
                    idx++;
    }}}}
    for (var i0 = 0; i0 < 16; i0++) {
        mimc.in[idx] <== conv2d_1_bias[i0];
        idx++;
    }

    for (var i0 = 0; i0 < 16; i0++) {
        mimc.in[idx] <== batch_normalization_1_a[i0];
        idx++;
    }
    for (var i0 = 0; i0 < 16; i0++) {
        mimc.in[idx] <== batch_normalization_1_b[i0];
        idx++;
    }

    for (var i0 = 0; i0 < 16; i0++) {
        for (var i1 = 0; i1 < 10; i1++) {
            mimc.in[idx] <== dense_weights[i0][i1];
            idx++;
    }}
    for (var i0 = 0; i0 < 10; i0++) {
        mimc.in[idx] <== dense_bias[i0];
        idx++;
    }

    // padding
    for (var i = idx; i < 1000; i++) {
        mimc.in[i] <== 0;
    }
    step_out[0] <== mimc.out;
    log(step_out[0]);
    step_in[0] === step_out[0];

    poseidon[0] = Poseidon(3);
    poseidon[1] = Poseidon(2);

    poseidon[0].inputs[0] <== step_in[1];
    poseidon[0].inputs[1] <== cid_out[0];
    poseidon[0].inputs[2] <== cid_out[1];
    step_out[1] <== poseidon[0].out;
    log(step_out[1]);

    poseidon[1].inputs[0] <== step_in[2];
    poseidon[1].inputs[1] <== model_out;
    step_out[2] <== poseidon[1].out;
    log(step_out[2]);
}

component main { public [step_in] } = Recursive();