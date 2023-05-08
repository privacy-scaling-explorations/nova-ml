pragma circom 2.0.0;

include "../../node_modules/circomlib-ml/circuits/BatchNormalization2D.circom";
include "../../node_modules/circomlib-ml/circuits/Dense.circom";
include "../../node_modules/circomlib-ml/circuits/Conv2D.circom";
include "../../node_modules/circomlib-ml/circuits/AveragePooling2D.circom";
include "../../node_modules/circomlib-ml/circuits/ArgMax.circom";
include "../../node_modules/circomlib-ml/circuits/Poly.circom";
include "../../node_modules/circomlib-ml/circuits/GlobalAveragePooling2D.circom";

template Model() {
signal input in[28][28][1];
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
signal output out[1];

component conv2d = Conv2D(28, 28, 1, 4, 3, 1);
component batch_normalization = BatchNormalization2D(26, 26, 4);
component lambda[26][26][4];
for (var i0 = 0; i0 < 26; i0++) {
    for (var i1 = 0; i1 < 26; i1++) {
        for (var i2 = 0; i2 < 4; i2++) {
            lambda[i0][i1][i2] = Poly(1000000);
}}}
component average_pooling2d = AveragePooling2D(26, 26, 4, 2, 2, 250);
component conv2d_1 = Conv2D(13, 13, 4, 16, 3, 1);
component batch_normalization_1 = BatchNormalization2D(11, 11, 16);
component lambda_1[11][11][16];
for (var i0 = 0; i0 < 11; i0++) {
    for (var i1 = 0; i1 < 11; i1++) {
        for (var i2 = 0; i2 < 16; i2++) {
            lambda_1[i0][i1][i2] = Poly(1000000000000000000000);
}}}
component average_pooling2d_1 = AveragePooling2D(11, 11, 16, 2, 2, 250);
component global_average_pooling2d = GlobalAveragePooling2D(5, 5, 16, 40);
component dense = Dense(16, 10);
component softmax = ArgMax(10);

for (var i0 = 0; i0 < 28; i0++) {
    for (var i1 = 0; i1 < 28; i1++) {
        for (var i2 = 0; i2 < 1; i2++) {
            conv2d.in[i0][i1][i2] <== in[i0][i1][i2];
}}}
for (var i0 = 0; i0 < 3; i0++) {
    for (var i1 = 0; i1 < 3; i1++) {
        for (var i2 = 0; i2 < 1; i2++) {
            for (var i3 = 0; i3 < 4; i3++) {
                conv2d.weights[i0][i1][i2][i3] <== conv2d_weights[i0][i1][i2][i3];
}}}}
for (var i0 = 0; i0 < 4; i0++) {
    conv2d.bias[i0] <== conv2d_bias[i0];
}
for (var i0 = 0; i0 < 26; i0++) {
    for (var i1 = 0; i1 < 26; i1++) {
        for (var i2 = 0; i2 < 4; i2++) {
            batch_normalization.in[i0][i1][i2] <== conv2d.out[i0][i1][i2];
}}}
for (var i0 = 0; i0 < 4; i0++) {
    batch_normalization.a[i0] <== batch_normalization_a[i0];
}
for (var i0 = 0; i0 < 4; i0++) {
    batch_normalization.b[i0] <== batch_normalization_b[i0];
}
for (var i0 = 0; i0 < 26; i0++) {
    for (var i1 = 0; i1 < 26; i1++) {
        for (var i2 = 0; i2 < 4; i2++) {
            lambda[i0][i1][i2].in <== batch_normalization.out[i0][i1][i2];
}}}
for (var i0 = 0; i0 < 26; i0++) {
    for (var i1 = 0; i1 < 26; i1++) {
        for (var i2 = 0; i2 < 4; i2++) {
            average_pooling2d.in[i0][i1][i2] <== lambda[i0][i1][i2].out;
}}}
for (var i0 = 0; i0 < 13; i0++) {
    for (var i1 = 0; i1 < 13; i1++) {
        for (var i2 = 0; i2 < 4; i2++) {
            conv2d_1.in[i0][i1][i2] <== average_pooling2d.out[i0][i1][i2];
}}}
for (var i0 = 0; i0 < 3; i0++) {
    for (var i1 = 0; i1 < 3; i1++) {
        for (var i2 = 0; i2 < 4; i2++) {
            for (var i3 = 0; i3 < 16; i3++) {
                conv2d_1.weights[i0][i1][i2][i3] <== conv2d_1_weights[i0][i1][i2][i3];
}}}}
for (var i0 = 0; i0 < 16; i0++) {
    conv2d_1.bias[i0] <== conv2d_1_bias[i0];
}
for (var i0 = 0; i0 < 11; i0++) {
    for (var i1 = 0; i1 < 11; i1++) {
        for (var i2 = 0; i2 < 16; i2++) {
            batch_normalization_1.in[i0][i1][i2] <== conv2d_1.out[i0][i1][i2];
}}}
for (var i0 = 0; i0 < 16; i0++) {
    batch_normalization_1.a[i0] <== batch_normalization_1_a[i0];
}
for (var i0 = 0; i0 < 16; i0++) {
    batch_normalization_1.b[i0] <== batch_normalization_1_b[i0];
}
for (var i0 = 0; i0 < 11; i0++) {
    for (var i1 = 0; i1 < 11; i1++) {
        for (var i2 = 0; i2 < 16; i2++) {
            lambda_1[i0][i1][i2].in <== batch_normalization_1.out[i0][i1][i2];
}}}
for (var i0 = 0; i0 < 11; i0++) {
    for (var i1 = 0; i1 < 11; i1++) {
        for (var i2 = 0; i2 < 16; i2++) {
            average_pooling2d_1.in[i0][i1][i2] <== lambda_1[i0][i1][i2].out;
}}}
for (var i0 = 0; i0 < 5; i0++) {
    for (var i1 = 0; i1 < 5; i1++) {
        for (var i2 = 0; i2 < 16; i2++) {
            global_average_pooling2d.in[i0][i1][i2] <== average_pooling2d_1.out[i0][i1][i2];
}}}
for (var i0 = 0; i0 < 16; i0++) {
    dense.in[i0] <== global_average_pooling2d.out[i0];
}
for (var i0 = 0; i0 < 16; i0++) {
    for (var i1 = 0; i1 < 10; i1++) {
        dense.weights[i0][i1] <== dense_weights[i0][i1];
}}
for (var i0 = 0; i0 < 10; i0++) {
    dense.bias[i0] <== dense_bias[i0];
}
for (var i0 = 0; i0 < 10; i0++) {
    softmax.in[i0] <== dense.out[i0];
}
out[0] <== softmax.out;

}