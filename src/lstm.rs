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

    let f_out_filename = root.join("src/lstm/f_out.json");
    let f_out_file = File::open(f_out_filename).unwrap();
    let f_out_reader = BufReader::new(f_out_file);
    let f_out_json: Vec<Value> = from_reader(f_out_reader).unwrap();

    let f_remainder_filename = root.join("src/lstm/f_remainder.json");
    let f_remainder_file = File::open(f_remainder_filename).unwrap();
    let f_remainder_reader = BufReader::new(f_remainder_file);
    let f_remainder_json: Vec<Value> = from_reader(f_remainder_reader).unwrap();

    let i_out_filename = root.join("src/lstm/i_out.json");
    let i_out_file = File::open(i_out_filename).unwrap();
    let i_out_reader = BufReader::new(i_out_file);
    let i_out_json: Vec<Value> = from_reader(i_out_reader).unwrap();

    let i_remainder_filename = root.join("src/lstm/i_remainder.json");
    let i_remainder_file = File::open(i_remainder_filename).unwrap();
    let i_remainder_reader = BufReader::new(i_remainder_file);
    let i_remainder_json: Vec<Value> = from_reader(i_remainder_reader).unwrap();

    let candidate_out_filename = root.join("src/lstm/candidate_out.json");
    let candidate_out_file = File::open(candidate_out_filename).unwrap();
    let candidate_out_reader = BufReader::new(candidate_out_file);
    let candidate_out_json: Vec<Value> = from_reader(candidate_out_reader).unwrap();

    let candidate_remainder_filename = root.join("src/lstm/candidate_remainder.json");
    let candidate_remainder_file = File::open(candidate_remainder_filename).unwrap();
    let candidate_remainder_reader = BufReader::new(candidate_remainder_file);
    let candidate_remainder_json: Vec<Value> = from_reader(candidate_remainder_reader).unwrap();

    let o_out_filename = root.join("src/lstm/o_out.json");
    let o_out_file = File::open(o_out_filename).unwrap();
    let o_out_reader = BufReader::new(o_out_file);
    let o_out_json: Vec<Value> = from_reader(o_out_reader).unwrap();

    let o_remainder_filename = root.join("src/lstm/o_remainder.json");
    let o_remainder_file = File::open(o_remainder_filename).unwrap();
    let o_remainder_reader = BufReader::new(o_remainder_file);
    let o_remainder_json: Vec<Value> = from_reader(o_remainder_reader).unwrap();

    let c_out_filename = root.join("src/lstm/c_out.json");
    let c_out_file = File::open(c_out_filename).unwrap();
    let c_out_reader = BufReader::new(c_out_file);
    let c_out_json: Vec<Value> = from_reader(c_out_reader).unwrap();

    let c_remainder_filename = root.join("src/lstm/c_remainder.json");
    let c_remainder_file = File::open(c_remainder_filename).unwrap();
    let c_remainder_reader = BufReader::new(c_remainder_file);
    let c_remainder_json: Vec<Value> = from_reader(c_remainder_reader).unwrap();

    let h_out_filename = root.join("src/lstm/h_out.json");
    let h_out_file = File::open(h_out_filename).unwrap();
    let h_out_reader = BufReader::new(h_out_file);
    let h_out_json: Vec<Value> = from_reader(h_out_reader).unwrap();

    let h_remainder_filename = root.join("src/lstm/h_remainder.json");
    let h_remainder_file = File::open(h_remainder_filename).unwrap();
    let h_remainder_reader = BufReader::new(h_remainder_file);
    let h_remainder_json: Vec<Value> = from_reader(h_remainder_reader).unwrap();

    let in_filename = root.join("src/lstm/in.json");
    let in_file = File::open(in_filename).unwrap();
    let in_reader = BufReader::new(in_file);
    let in_json: Vec<Value> = from_reader(in_reader).unwrap();

    let mut private_inputs = Vec::new();
    for i in 0..iteration_count {
        let mut json = params.clone();
        json.insert("in".to_string(), in_json[i].clone());
        json.insert("f_out".to_string(), f_out_json[i].clone());
        json.insert("f_remainder".to_string(), f_remainder_json[i].clone());
        json.insert("i_out".to_string(), i_out_json[i].clone());
        json.insert("i_remainder".to_string(), i_remainder_json[i].clone());
        json.insert("candidate_out".to_string(), candidate_out_json[i].clone());
        json.insert("candidate_remainder".to_string(), candidate_remainder_json[i].clone());
        json.insert("o_out".to_string(), o_out_json[i].clone());
        json.insert("o_remainder".to_string(), o_remainder_json[i].clone());
        json.insert("c_out".to_string(), c_out_json[i].clone());
        json.insert("c_remainder".to_string(), c_remainder_json[i].clone());
        json.insert("h_out".to_string(), h_out_json[i].clone());
        json.insert("h_remainder".to_string(), h_remainder_json[i].clone());
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

    for i in 0..3 {
        let out;
        if h_out_json[11][i].as_str().unwrap().starts_with("-") {
            let out_str = h_out_json[11][i].as_str().unwrap()[1..].to_string();
            out = F1::zero() - F1::from_str_vartime(&out_str).unwrap();
        } else {
            let out_str = h_out_json[11][i].as_str().unwrap();
            out = F1::from_str_vartime(&out_str).unwrap();
        }
        let diff = out.sub(&result[i]).to_repr();
        for j in 0..diff.len() {
            assert_eq!(diff[j], 0);
        }
    }

    for i in 0..3 {
        let out;
        if c_out_json[11][i].as_str().unwrap().starts_with("-") {
            let out_str = c_out_json[11][i].as_str().unwrap()[1..].to_string();
            out = F1::zero() - F1::from_str_vartime(&out_str).unwrap();
        } else {
            let out_str = c_out_json[11][i].as_str().unwrap();
            out = F1::from_str_vartime(&out_str).unwrap();
        }
        let diff = out.sub(&result[i+3]).to_repr();
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
