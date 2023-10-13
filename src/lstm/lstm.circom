pragma circom 2.0.0;

include "../../node_modules/circomlib-ml/circuits/circomlib-matrix/matMul.circom";
include "../../node_modules/circomlib-ml/circuits/Zanh.circom";
include "../../node_modules/circomlib-ml/circuits/Zigmoid.circom";

template LSTM (n) { // n is 10 to the number of decimal places
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

    signal input i_out[3];
    signal input i_remainder[3];

    signal input i_zigmoid_out[3];
    signal input i_zigmoid_remainder[3];

    signal input f_out[3];
    signal input f_remainder[3];

    signal input f_zigmoid_out[3];
    signal input f_zigmoid_remainder[3];

    signal input candidate_out[3];
    signal input candidate_remainder[3];

    signal input candidate_zanh_out[3];
    signal input candidate_zanh_remainder[3];

    signal input o_out[3];
    signal input o_remainder[3];

    signal input o_zigmoid_out[3];
    signal input o_zigmoid_remainder[3];

    signal input c_out[3];
    signal input c_remainder[3];

    signal input c_zanh_out[3];
    signal input c_zanh_remainder[3];

    signal input h_out[3];
    signal input h_remainder[3];

    component mat_mul_ui = matMul(1,3,3);
    component mat_mul_uf = matMul(1,3,3);
    component mat_mul_uc = matMul(1,3,3);
    component mat_mul_uo = matMul(1,3,3);

    for (var j=0; j<3; j++) {
        assert(i_remainder[j] < n);
        assert(f_remainder[j] < n);
        assert(candidate_remainder[j] < n);
        assert(o_remainder[j] < n);
        assert(c_remainder[j] < n);
        assert(h_remainder[j] < n);

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
    signal c_tmp[3];
    signal h_tmp[3];

    component i_zigmoid[3];
    component f_zigmoid[3];
    component candidate_zanh[3];
    component o_zigmoid[3];
    component c_zanh[3];

    for (var j=0; j<3; j++) {
        i[j] <== in * wi[j] + mat_mul_ui.out[0][j] + bi[j];
        i_out[j] * n + i_remainder[j] === i[j];

        i_zigmoid[j] = Zigmoid(n);
        i_zigmoid[j].in <== i_out[j];
        i_zigmoid[j].out <== i_zigmoid_out[j];
        i_zigmoid[j].remainder <== i_zigmoid_remainder[j];

        f[j] <== in * wf[j] + mat_mul_uf.out[0][j] + bf[j];
        f_out[j] * n + f_remainder[j] === f[j];

        f_zigmoid[j] = Zigmoid(n);
        f_zigmoid[j].in <== f_out[j];
        f_zigmoid[j].out <== f_zigmoid_out[j];
        f_zigmoid[j].remainder <== f_zigmoid_remainder[j];

        c[j] <== in * wc[j] + mat_mul_uc.out[0][j] + bc[j];
        candidate_out[j] * n + candidate_remainder[j] === c[j];

        candidate_zanh[j] = Zanh(n);
        candidate_zanh[j].in <== candidate_out[j];
        candidate_zanh[j].out <== candidate_zanh_out[j];
        candidate_zanh[j].remainder <== candidate_zanh_remainder[j];

        o[j] <== in * wo[j] + mat_mul_uo.out[0][j] + bo[j];
        o_out[j] * n + o_remainder[j] === o[j];

        o_zigmoid[j] = Zigmoid(n);
        o_zigmoid[j].in <== o_out[j];
        o_zigmoid[j].out <== o_zigmoid_out[j];
        o_zigmoid[j].remainder <== o_zigmoid_remainder[j];

        tmp[j] <== f_zigmoid_out[j]*step_in[j+3];
        c_tmp[j] <== tmp[j] + i_zigmoid_out[j]*candidate_zanh_out[j];
        c_out[j] * n + c_remainder[j] === c_tmp[j];

        step_out[j+3] <== c_out[j];

        c_zanh[j] = Zanh(n);
        c_zanh[j].in <== c_out[j];
        c_zanh[j].out <== c_zanh_out[j];
        c_zanh[j].remainder <== c_zanh_remainder[j];

        h_tmp[j] <== o_zigmoid_out[j] * c_zanh_out[j];
        h_out[j] * n + h_remainder[j] === h_tmp[j];
        step_out[j] <== h_out[j];
    }

}

component main { public [step_in] } = LSTM(10**9);