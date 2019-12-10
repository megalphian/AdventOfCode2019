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

    
    let inputs1 : Vec<i32> = [1].to_vec();
    println!("Part1:");
    let mut int_code1 = IntCodeMemory::_new_with_input(&memory_values, &inputs1);
    int_code1.run_program();

    let inputs2 : Vec<i32> = [5].to_vec();
    println!("Part2:");
    let mut int_code2 = IntCodeMemory::_new_with_input(&memory_values, &inputs2);
    int_code2.run_program();
}