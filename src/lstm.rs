use std::{
    collections::HashMap,
    env::current_dir,
    fs::File,
    io::{BufReader},
    time::Instant,
};

use ff::PrimeField;
use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F1,
    G2, S1, S2
};
use nova_snark::{traits::Group, CompressedSNARK};
use serde_json::{from_reader, Value};

pub fn main() {
    let iteration_count = 12;
    let root = current_dir().unwrap();

    let circuit_file = root.join("src/lstm/lstm.r1cs");
    let r1cs = load_r1cs(&FileLocation::PathBuf(circuit_file));
    let witness_generator_wasm = root.join("src/lstm/lstm_js/lstm.wasm");

    let params_filename = root.join("src/lstm/params.json");
    let params_file = File::open(params_filename).unwrap();
    let params_reader = BufReader::new(params_file);
    let params: HashMap<String, Value> = from_reader(params_reader).unwrap();

    let out_filename = root.join("src/lstm/out.json");
    let out_file = File::open(out_filename).unwrap();
    let out_reader = BufReader::new(out_file);
    let out_json: Vec<Value> = from_reader(out_reader).unwrap();

    let remainder_filename = root.join("src/lstm/remainder.json");
    let remainder_file = File::open(remainder_filename).unwrap();
    let remainder_reader = BufReader::new(remainder_file);
    let remainder_json: Vec<Value> = from_reader(remainder_reader).unwrap();

    let in_filename = root.join("src/lstm/in.json");
    let in_file = File::open(in_filename).unwrap();
    let in_reader = BufReader::new(in_file);
    let in_json: Vec<Value> = from_reader(in_reader).unwrap();

    let mut private_inputs = Vec::new();
    for i in 0..iteration_count {
        let mut json = params.clone();
        json.insert("in".to_string(), in_json[i].clone());
        json.insert("out".to_string(), out_json[i].clone());
        json.insert("remainder".to_string(), remainder_json[i].clone());
        private_inputs.push(json.clone());
    }

    let start_public_input = vec![F1::zero(), F1::zero(), F1::zero(), F1::zero(), F1::zero(), F1::zero()];
    // println!("start_public_input: {:?}", start_public_input);

    // println!("json: {:?}", json);

    let start = Instant::now();
    let pp = create_public_params(r1cs.clone());
    println!("PublicParams creation took {:?}", start.elapsed());

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
    // println!("z0_primary: {:?}", start_public_input);
    // println!("z0_secondary: {:?}", z0_secondary);
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

    let result  = res.unwrap().0;

    // println!("{:?}", out_json);

    for i in 0..6 {
        let out;
        if out_json[11][i].as_str().unwrap().starts_with("-") {
            let out_str = out_json[11][i].as_str().unwrap()[1..].to_string();
            out = F1::zero() - F1::from_str_vartime(&out_str).unwrap();
        } else {
            let out_str = out_json[11][i].as_str().unwrap();
            out = F1::from_str_vartime(&out_str).unwrap();
        }
        let diff = out.sub(&result[i]).to_repr();
        for j in 0..diff.len() {
            assert_eq!(diff[j], 0);
        }
    }

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
    // println!("z0_primary: {:?}", start_public_input);
    // println!("z0_secondary: {:?}", z0_secondary);
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
