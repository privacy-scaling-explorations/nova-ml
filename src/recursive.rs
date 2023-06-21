use std::{collections::HashMap, env::current_dir, fs::File, io::BufReader, time::Instant};

use ff::PrimeField;
use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F1,
    G2, S1, S2,
};
use nova_snark::{traits::Group, CompressedSNARK};
use serde_json::{from_reader, Value};

pub fn main() {
    let iteration_count = 100;
    let root = current_dir().unwrap();

    let circuit_file = root.join("src/recursive/recursive.r1cs");
    let r1cs = load_r1cs(&FileLocation::PathBuf(circuit_file));
    let witness_generator_wasm = root.join("src/recursive/recursive_js/recursive.wasm");

    let json_filename = root.join("src/recursive/input.json");
    let json_file = File::open(json_filename).unwrap();
    let json_reader = BufReader::new(json_file);
    let json: HashMap<String, Value> = from_reader(json_reader).unwrap();

    // println!("json: {:?}", json);

    let mut private_inputs = Vec::new();
    for i in 0..iteration_count {
        let in_filename = root.join("assets/mnist_".to_owned() + &i.to_string() + ".json");
        let in_file = File::open(in_filename).unwrap();
        let in_reader = BufReader::new(in_file);
        let mut in_json: HashMap<String, Value> = from_reader(in_reader).unwrap();
        in_json.extend(json.clone());
        private_inputs.push(in_json.clone());
        // println!("in_json: {:?}", in_json);
    }

    let start_public_input = vec![
        F1::from_str_vartime(
            "9046494323512618473441251792600205790522500136671742160573113950083932095915",
        )
        .unwrap(),
        F1::from_str_vartime("0").unwrap(),
        F1::from_str_vartime("0").unwrap(),
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
