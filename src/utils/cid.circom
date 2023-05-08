pragma circom 2.0.0;

include "../sha256/sha256.circom";
include "../../node_modules/circomlib-ml/circuits/circomlib/bitify.circom";

// convert a 797x8 bit array (pgm) to 28x28x1 array (pixels)
template getPixels() {
    signal input in[797*8];
    signal output out[28][28][1];

    component pixels[28][28][1];

    for (var i=0; i<28; i++) {
        for (var j=0; j<28; j++) {
            pixels[i][j][0] = Bits2Num(8);
            for (var k=0; k<8; k++) {
                pixels[i][j][0].in[k] <== in[13*8+i*28*8+j*8+k]; // the pgm header is 13 bytes
            }
            out[i][j][0] <== pixels[i][j][0].out;
        }
    }
}

// convert a 797x8 bit array (pgm) to the corresponding CID (in two parts)
template getCid() {
    signal input in[797*8];
    signal output out[2];

    component sha = Sha256(797*8);
    for (var i=0; i<797*8; i++) {
        sha.in[i] <== in[i];
    }
    
    component b2n[2];

    for (var i=1; i>=0; i--) {
        b2n[i] = Bits2Num(128);
        for (var j=127; j>=0; j--) {
            b2n[i].in[127-j] <== sha.out[i*128+j];
        }
        out[i] <== b2n[i].out;
    }
}