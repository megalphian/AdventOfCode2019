use std::fs;
mod intcode;
use intcode::intcode::IntCodeMemory;
use intcode::intcode::IntCodeCompute;

fn search_protocol(memory_values : &Vec<i32>, final_val: i32) -> i32
{
    let mut out_protocol : i32 = 0;

    for i in 0..99
    {
        for j in 0..99 {
            let mut int_code = IntCodeMemory::new(memory_values);
            let result = int_code.run_opcode_instructions(i, j);
            if final_val == result {
                out_protocol = 100 * i + j;
                break; 
            }
        }
    }
    
    out_protocol
}

fn main() {
    let filename = "data/input2.txt";

    let _file_contents = fs::read_to_string(filename).unwrap();
    let memory_values : Vec<i32> = _file_contents
                                    .split(",")
                                    .map(|s| s.trim().parse().unwrap())
                                    .collect();

    let mut int_code1 = IntCodeMemory::new(&memory_values);
    let part1_result = int_code1.run_opcode_instructions(12,2);
    let part2_result = search_protocol(&memory_values, 19690720);

    println!("1202 result = {}", part1_result);
    println!("19690720 result = {}", part2_result);

}