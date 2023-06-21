use std::{collections::HashMap, env::current_dir, fs::File, io::BufReader, time::Instant};

use ff::PrimeField;
use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F1,
    G2, S1, S2,
};
use nova_snark::{traits::Group, CompressedSNARK};
use serde_json::{from_reader, json, Value};

pub fn main() {
    let iteration_count = 2;
    let root = current_dir().unwrap();

    let circuit_file = root.join("src/merkle/merkle.r1cs");
    let r1cs = load_r1cs(&FileLocation::PathBuf(circuit_file));
    let witness_generator_wasm = root.join("src/merkle/merkle_js/merkle.wasm");

    let json_filename = root.join("src/merkle/input.json");
    let json_file = File::open(json_filename).unwrap();
    let json_reader = BufReader::new(json_file);
    let json: HashMap<String, Value> = from_reader(json_reader).unwrap();

    // println!("json: {:?}", json);

    let new_data_roots: Vec<&str> = vec![
        "9978745174407704562480317393089160098503076291990639551137347089166356848494",
        "20006630605069097454531987701120801349324493017093922517491290840824485474630",
    ];
    let new_out_roots = vec![
        "20303444514131719247382249113208220639224156735928111019283282774530433983207",
        "21066590153337281233666325381424170318020910187536186684053897380256045351510",
    ];
    let path_indices = vec!["0x00", "0x01"];
    let data_path_elements = vec![
        vec![
            "0",
            "13369283962880989185402749078631251414623069379107970236676130977028910264128",
        ],
        vec![
            "293522823212032739177258903802228976166",
            "13369283962880989185402749078631251414623069379107970236676130977028910264128",
        ],
    ];
    let out_path_elements = vec![
        vec![
            "0",
            "13369283962880989185402749078631251414623069379107970236676130977028910264128",
        ],
        vec![
            "7",
            "13369283962880989185402749078631251414623069379107970236676130977028910264128",
        ],
    ];

    let mut private_inputs = Vec::new();
    for i in 0..iteration_count {
        let in_filename = root.join("assets/mnist_".to_owned() + &i.to_string() + ".json");
        let in_file = File::open(in_filename).unwrap();
        let in_reader = BufReader::new(in_file);
        let mut in_json: HashMap<String, Value> = from_reader(in_reader).unwrap();
        in_json.extend(json.clone());
        in_json.insert("newDataRoot".to_string(), json!(&new_data_roots[i]));
        in_json.insert("newOutRoot".to_string(), json!(&new_out_roots[i]));
        in_json.insert("pathIndices".to_string(), json!(&path_indices[i]));
        in_json.insert("dataPathElements".to_string(), json!(data_path_elements[i]));
        in_json.insert("outPathElements".to_string(), json!(out_path_elements[i]));
        private_inputs.push(in_json);
    }
    // println!("private_inputs: {:?}", private_inputs);

    let start_public_input = vec![
        F1::from_str_vartime(
            "9046494323512618473441251792600205790522500136671742160573113950083932095915",
        )
        .unwrap(),
        F1::from_str_vartime(
            "17902710477904492439403341920153385152309361002685772052322415234613963971884",
        )
        .unwrap(),
        F1::from_str_vartime(
            "17902710477904492439403341920153385152309361002685772052322415234613963971884",
        )
        .unwrap(),
    ];

    let pp = create_public_params(r1cs.clone());

    println!(
        "Number of constraints per step (primary circuit): {}",
        pp.num_constraints().0
    );
    println!(
        "Number of constraints per step (secondary circuit): {}",
        pp.num_constraints().1
    );

    println!(
        "Number of variables per step (primary circuit): {}",
        pp.num_variables().0
    );
    println!(
        "Number of variables per step (secondary circuit): {}",
        pp.num_variables().1
    );

    println!("Creating a RecursiveSNARK...");
    let start = Instant::now();
    let recursive_snark = create_recursive_circuit(
        FileLocation::PathBuf(witness_generator_wasm),
        r1cs,
        private_inputs,
        start_public_input.clone(),
        &pp,
    )
    .unwrap();
    println!("RecursiveSNARK creation took {:?}", start.elapsed());

    // TODO: empty?
    let z0_secondary = vec![<G2 as Group>::Scalar::zero()];

    // verify the recursive SNARK
    println!("Verifying a RecursiveSNARK...");
    println!("z0_primary: {:?}", start_public_input);
    println!("z0_secondary: {:?}", z0_secondary);
    let start = Instant::now();
    let res = recursive_snark.verify(
        &pp,
        iteration_count,
        start_public_input.clone(),
        z0_secondary.clone(),
    );
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    assert!(res.is_ok());

    // produce a compressed SNARK
    println!("Generating a CompressedSNARK using Spartan with IPA-PC...");
    let start = Instant::now();
    let (pk, vk) = CompressedSNARK::<_, _, _, _, S1, S2>::setup(&pp).unwrap();
    let res = CompressedSNARK::<_, _, _, _, S1, S2>::prove(&pp, &pk, &recursive_snark);
    println!(
        "CompressedSNARK::prove: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());
    let compressed_snark = res.unwrap();

    // verify the compressed SNARK
    println!("Verifying a CompressedSNARK...");
    println!("z0_primary: {:?}", start_public_input);
    println!("z0_secondary: {:?}", z0_secondary);
    let start = Instant::now();
    let res = compressed_snark.verify(
        &vk,
        iteration_count,
        start_public_input.clone(),
        z0_secondary,
    );
    println!(
        "CompressedSNARK::verify: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());
}
