pragma circom 2.0.0;

include "../../node_modules/circomlib-ml/circuits/circomlib-matrix/matMul.circom";

template LSTM (n, m) { // n is 10 to the number of decimal places for hidden states, m is 10 to the number of decimal places for cell states
    signal input step_in[6]; // hidden states + cell states
    signal output step_out[6];

    // private inputs
    signal input in;

    signal input wi[3];
    signal input wf[3];
    signal input wc[3];
    signal input wo[3];

    signal input ui[3][3];
    signal input uf[3][3];
    signal input uc[3][3];
    signal input uo[3][3];

    signal input bi[3];
    signal input bf[3];
    signal input bc[3];
    signal input bo[3];

    signal input out[6];
    signal input remainder[6];

    component mat_mul_ui = matMul(1,3,3);
    component mat_mul_uf = matMul(1,3,3);
    component mat_mul_uc = matMul(1,3,3);
    component mat_mul_uo = matMul(1,3,3);

    for (var j=0; j<3; j++) {
        assert(remainder[j] < n);
        assert(remainder[j+3] < m);

        mat_mul_ui.a[0][j] <== step_in[j];
        mat_mul_uf.a[0][j] <== step_in[j];
        mat_mul_uc.a[0][j] <== step_in[j];
        mat_mul_uo.a[0][j] <== step_in[j];

        for (var k=0; k<3; k++) {
            mat_mul_ui.b[j][k] <== ui[j][k];
            mat_mul_uf.b[j][k] <== uf[j][k];
            mat_mul_uc.b[j][k] <== uc[j][k];
            mat_mul_uo.b[j][k] <== uo[j][k];
        }
    }
    
    signal f[3];
    signal i[3];
    signal c[3]; // candidate, not cell
    signal o[3];
    signal tmp[3];
    signal tmp2[6];

    for (var j=0; j<3; j++) {
        i[j] <== in * wi[j] + mat_mul_ui.out[0][j] + bi[j];
        f[j] <== in * wf[j] + mat_mul_uf.out[0][j] + bf[j];
        c[j] <== in * wc[j] + mat_mul_uc.out[0][j] + bc[j];
        o[j] <== in * wo[j] + mat_mul_uo.out[0][j] + bo[j];

        tmp[j] <== f[j]*step_in[j+3];
        tmp2[j+3] <== tmp[j] + i[j]*c[j];
        out[j+3] * m + remainder[j+3] === tmp2[j+3];
        step_out[j+3] <== out[j+3];

        tmp2[j] <== o[j] * step_in[j+3];
        out[j] * n + remainder[j] === tmp2[j];
        step_out[j] <== out[j];
    }

}

component main { public [step_in] } = LSTM(10**12, 10**8);