pragma circom 2.0.0;

include "../../node_modules/circomlib-ml/circuits/circomlib-matrix/matElemMul.circom";
include "../../node_modules/circomlib-ml/circuits/circomlib-matrix/matElemSum.circom";
include "../../node_modules/circomlib-ml/circuits/util.circom";
include "../../node_modules/circomlib/circuits/Mux1.circom";

template Main (nRows, nCols, nChannels, nFilters, kernelSize, strides) {
    signal input step_in[(nRows-kernelSize)\strides+1][(nCols-kernelSize)\strides+1][nFilters];
    signal output step_out[(nRows-kernelSize)\strides+1][(nCols-kernelSize)\strides+1][nFilters];

    // private inputs
    signal input in[kernelSize][kernelSize][nChannels];
    signal input weights[kernelSize][kernelSize][nChannels]; // will vary per filter;
    signal input bias;
    signal input sel[(nRows-kernelSize)\strides+1][(nCols-kernelSize)\strides+1][nFilters]; // assume binary

    // intermediate output
    signal out;

    component mul[nChannels];
    component elemSum[nChannels];
    
    // perform convolution
    for (var i=0; i< nChannels; i++) {
        mul[i] = matElemMul(kernelSize, kernelSize);
        for (var x=0; x<kernelSize; x++) {
            for (var y=0; y<kernelSize; y++) {
                mul[i].a[x][y] <== in[x][y][i];
                mul[i].b[x][y] <== weights[x][y][i];
            }
        }
        elemSum[i] = matElemSum(kernelSize, kernelSize);
        for (var x=0; x<kernelSize; x++) {
            for (var y=0; y<kernelSize; y++) {
                elemSum[i].a[x][y] <== mul[i].out[x][y];
            }
        }
    }

    component sum = Sum(nChannels);

    for (var i=0; i< nChannels; i++) {
        sum.in[i] <== elemSum[i].out;
    }
    
    out <== sum.out + bias;

    // TODO: put output in the correct position of step_out
    
    for (var i=0; i<(nRows-kernelSize)\strides+1; i++) {
        for (var j=0; j<(nCols-kernelSize)\strides+1; j++) {
            for (var k=0; k<nFilters; k++) {
                // ensure the selected position is zero
                step_in[i][j][k]*sel[i][j][k] === 0;
                step_out[i][j][k] <== step_in[i][j][k] + out*sel[i][j][k];
            }
        }
    }
}

component main { public [step_in] } = Main(50,50,3,2,3,1);