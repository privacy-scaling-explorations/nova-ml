use std::env;

// mod merkle;
mod recursive;
mod conv2d;
mod conv2d_nova;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an example name");
        return;
    }

    let example = &args[1];

    if example == "recursive" {
        recursive::main();
    } else if example == "merkle" {
        // merkle::main();
    } else if example == "conv2d" {
        conv2d::main();
    } else if example == "conv2d_nova" {
        conv2d_nova::main();
    } else {
        println!("Please provide a valid example name");
    }
}
