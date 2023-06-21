use std::{collections::HashMap, env::current_dir, fs::File, io::BufReader, time::Instant};

use ff::PrimeField;
use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F1,
    G2, S1, S2,
};
use nova_snark::{traits::Group, CompressedSNARK};
use serde_json::{from_reader, json, Value};

pub fn main() {
    let n_rows = 32;
    let n_cols = 32;
    let n_channels = 3;
    let n_filters = 2;
    let kernel_size = 3;

    let x = n_rows - kernel_size + 1;
    let y = n_cols - kernel_size + 1;

    let iteration_count = x * y * n_filters;
    let root = current_dir().unwrap();

    let mut start_public_input = Vec::new();
    for _i in 0..iteration_count {
        start_public_input.push(F1::from_str_vartime("0").unwrap());
    }
    // println!("start_public_input: {:?}", start_public_input);

    let circuit_file = root.join("src/conv2d_nova/conv2d_nova.r1cs");
    let r1cs = load_r1cs(&FileLocation::PathBuf(circuit_file));
    let witness_generator_wasm = root.join("src/conv2d_nova/conv2d_nova_js/conv2d_nova.wasm");

    let json_filename = root.join("src/conv2d/input.json");
    let json_file = File::open(json_filename).unwrap();
    let json_reader = BufReader::new(json_file);
    let json: HashMap<String, Value> = from_reader(json_reader).unwrap();

    // println!("json: {:?}", json);

    let in_vector = json.get("in").unwrap().as_array().unwrap();
    let mut idx: usize = 0;
    let mut in_array = Vec::new();
    for _i in 0..n_rows {
        let mut row = Vec::new();
        for _j in 0..n_cols {
            let mut col = Vec::new();
            for _k in 0..n_channels {
                col.push(in_vector[idx].clone());
                idx += 1;
            }
            row.push(col);
        }
        in_array.push(row);
    }
    // println!("in_array: {:?}", in_array);

    let weights_vector = json.get("weights").unwrap().as_array().unwrap();
    let mut idx: usize = 0;
    let mut weights_array = Vec::new();
    for _a in 0..kernel_size {
        let mut row = Vec::new();
        for _b in 0..kernel_size {
            let mut col = Vec::new();
            for _c in 0..n_channels {
                let mut channel = Vec::new();
                for _d in 0..n_filters {
                    channel.push(weights_vector[idx].clone());
                    idx += 1;
                }
                col.push(channel);
            }
            row.push(col);
        }
        weights_array.push(row);
    }
    // println!("weights_array: {:?}", weights_array);

    let mut private_inputs = Vec::new();
    for i in 0..x {
        for j in 0..y {
            for k in 0..n_filters {
                let mut private_json = HashMap::new();
                // in and weight
                let mut input = Vec::new();
                let mut weights = Vec::new();

                for a in 0..kernel_size {
                    for b in 0..kernel_size {
                        for c in 0..n_channels {
                            input.push(in_array[i + a][j + b][c].clone());
                            weights.push(weights_array[a][b][c][k].clone());
                        }
                    }
                }
                private_json.insert("in".to_string(), json!(input));
                private_json.insert("weights".to_string(), json!(weights));
                // bias
                let bias = json.get("bias").unwrap().as_array().unwrap()[k].clone();
                private_json.insert("bias".to_string(), json!(bias));
                // sel
                let mut sel = Vec::new();
                for a in 0..x {
                    for b in 0..y {
                        for c in 0..n_filters {
                            if a == i && b == j && c == k {
                                sel.push(1);
                            } else {
                                sel.push(0);
                            }
                        }
                    }
                }
                private_json.insert("sel".to_string(), json!(sel));
                private_inputs.push(private_json.clone());
            }
        }
    }

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
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());

    let result = res.unwrap().0;
    // println!("result: {:?}", result);

    // TODO: check output
    let out_filename = root.join("src/conv2d/result.json");
    let out_file = File::open(out_filename).unwrap();
    let out_reader = BufReader::new(out_file);
    let out_json: Vec<Value> = from_reader(out_reader).unwrap();
    // println!("out_json: {:?}", out_json);

    let result_json: Vec<Value> =
        serde_json::from_str(&serde_json::to_string(&result).unwrap()).unwrap();
    // println!("result_json: {:?}", result_json);

    for i in 0..iteration_count {
        assert_eq!(result_json[i], out_json[i]);
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
