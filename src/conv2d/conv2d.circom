pragma circom 2.0.0;

include "../../node_modules/circomlib-ml/circuits/Conv2D.circom";

template Main (nRows, nCols, nChannels, nFilters, kernelSize, strides) {
    signal input step_in[(nRows-kernelSize)\strides+1][(nCols-kernelSize)\strides+1][nFilters];
    signal output step_out[(nRows-kernelSize)\strides+1][(nCols-kernelSize)\strides+1][nFilters];

    // private inputs
    signal input in[nRows][nCols][nChannels];
    signal input weights[kernelSize][kernelSize][nChannels][nFilters];
    signal input bias[nFilters];

    component conv2d = Conv2D(nRows, nCols, nChannels, nFilters, kernelSize, strides);

    for (var i=0; i<nRows; i++) {
        for (var j=0; j<nCols; j++) {
            for (var k=0; k<nChannels; k++) {
                conv2d.in[i][j][k] <== in[i][j][k];
            }
        }
    }

    for (var i=0; i<kernelSize; i++) {
        for (var j=0; j<kernelSize; j++) {
            for (var k=0; k<nChannels; k++) {
                for (var l=0; l<nFilters; l++) {
                    conv2d.weights[i][j][k][l] <== weights[i][j][k][l];
                }
            }
        }
    }

    for (var i=0; i<nFilters; i++) {
        conv2d.bias[i] <== bias[i];
    }

    for (var i=0; i<(nRows-kernelSize)\strides+1; i++) {
        for (var j=0; j<(nCols-kernelSize)\strides+1; j++) {
            for (var k=0; k<nFilters; k++) {
                step_out[i][j][k] <== conv2d.out[i][j][k];
            }
        }
    }
}

component main { public [step_in] } = Main(32,32,3,2,3,1);