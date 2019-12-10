use std::fs;
mod intcode;
use intcode::intcode::IntCodeMemory;
use intcode::intcode::IntCodeCompute;

fn main() {
    let filename = "data/input5.txt";

    let _file_contents = fs::read_to_string(filename).unwrap();
    let memory_values : Vec<i32> = _file_contents
                                    .split(",")
                                    .map(|s| s.trim().parse().unwrap())
                                    .collect();

    let inputs : Vec<i32> = [1].to_vec();

    let mut int_code1 = IntCodeMemory::new_with_input(&memory_values, &inputs);
    int_code1.run_program();
}