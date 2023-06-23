pragma circom 2.0.0;

include "../../node_modules/circomlib-ml/circuits/circomlib-matrix/matMul.circom";

template RNN () {
    signal input step_in[3]; // hidden states
    signal output step_out[3];

    // private inputs
    signal input in;
    signal input wx[3];
    signal input wh[3][3];
    signal input b[3];

    component mat_mul = matMul(1,3,3);

    for (var i=0; i<3; i++) {
        mat_mul.a[0][i] <== step_in[i];
        for (var j=0; j<3; j++) {
            mat_mul.b[i][j] <== wh[i][j];
        }
    }

    for (var i=0; i<3; i++) {
        step_out[i] <== in * wx[i] + mat_mul.out[0][i] + b[i];
        // log(step_out[i]);
    }

}

component main { public [step_in] } = RNN();