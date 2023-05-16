pragma circom 2.0.3;

include "./merkleProof.circom";
include "./merkleTree.circom";

// inserts a leaf into a merkle tree
// checks that leaf previously is zero in the same position
template MerkleTreeUpdater(levels) {

    signal input oldRoot;
    signal input newRoot;
    signal input leaf;
    signal input pathIndices;
    signal input pathElements[levels];

    component treeBefore = MerkleProof(levels);
    for(var i = 0; i < levels; i++) {
        treeBefore.pathElements[i] <== pathElements[i];
    }
    treeBefore.pathIndices <== pathIndices;
    treeBefore.leaf <== 0;
    treeBefore.root === oldRoot;

    component treeAfter = MerkleProof(levels);
    for(var i = 0; i < levels; i++) {
        treeAfter.pathElements[i] <== pathElements[i];
    }
    treeAfter.pathIndices <== pathIndices;
    treeAfter.leaf <== leaf;
    treeAfter.root === newRoot;
}

// component main = MerkleTreeUpdater(2);

/* INPUT = {
    "oldRoot": "7423237065226347324353380772367382631490014989348495481811164164159255474657",
    "newRoot": "8609834780266691486032689962977132690188858761733934246118117325806381580487",
    "leaf": "1",
    "pathIndices": "0x00",
    "pathElements": ["0","14744269619966411208579211824598458697587494354926760081771325075741142829156"]
} */