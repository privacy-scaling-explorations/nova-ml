use std::{collections::HashMap, env::current_dir, time::Instant, fs::File, io::BufReader};

use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F1,
    G2, S1, S2,
};
use nova_snark::{traits::Group, CompressedSNARK};
use serde_json::{Value, from_reader};
use ff::PrimeField;

pub fn main() {
    let iteration_count = 1;
    let root = current_dir().unwrap();

    let mut start_public_input = Vec::new();
    for _i in 0..(3*3*2) {
        start_public_input.push(F1::from_str_vartime("0").unwrap());
    }
    // println!("start_public_input: {:?}", start_public_input);

    let circuit_file = root.join("src/conv2d/conv2d.r1cs");
    let r1cs = load_r1cs(&FileLocation::PathBuf(circuit_file));
    let witness_generator_wasm = root.join("src/conv2d/conv2d_js/conv2d.wasm");

    let json_filename = root.join("src/conv2d/input.json");
    let json_file = File::open(json_filename).unwrap();
    let json_reader = BufReader::new(json_file);
    let json: HashMap<String, Value> = from_reader(json_reader).unwrap();

    // println!("json: {:?}", json);

    let mut private_inputs = Vec::new();
    for _i in 0..iteration_count {
        private_inputs.push(json.clone());
    }

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

    let result = res.unwrap().0;
    // println!("result: {:?}", result);

    // check output
    let out_filename = root.join("src/conv2d/output.json");
    let out_file = File::open(out_filename).unwrap();
    let out_reader = BufReader::new(out_file);
    let out_json: Vec<Value> = from_reader(out_reader).unwrap();
    // println!("out_json: {:?}", out_json);

    for _i in 0..(3*3*2) {
        // println!("out_json: {:?}", out_json[_i]);
        let out_num = out_json[_i].as_i64().unwrap();
        if out_num < 0 {
            let out_str = (-out_num).to_string();
            let zero = F1::from(0);
            let out = zero.sub(&F1::from_str_vartime(&out_str).unwrap());
            // println!("out: {:?}", out);
            // println!("result: {:?}", result[_i]);
            // println!("{:?}", out.sub(&result[_i]));
            let diff = out.sub(&result[_i]).to_repr();
            // println!("diff: {:?}", diff);
            for _i in 0..diff.len() {
                if _i > 1 {
                    assert_eq!(diff[_i], 0);
                }
            }
        } else {
            let out_str = out_num.to_string();
            let out = F1::from_str_vartime(&out_str).unwrap();
            // println!("out: {:?}", out);
            // println!("result: {:?}", result[_i]);
            // println!("{:?}", out.sub(&result[_i]));
            let diff = out.sub(&result[_i]).to_repr();
            // println!("diff: {:?}", diff);
            for _i in 0..diff.len() {
                if _i > 1 {
                    assert_eq!(diff[_i], 0);
                }
            }
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