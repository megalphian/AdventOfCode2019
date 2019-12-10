use std::fs;

fn run_opcode_instructions(_command_list : &Vec<i32>, noun : i32, verb : i32) -> i32 {

    let mut cached_command_list : Vec<i32> = _command_list.to_vec();
    cached_command_list[1] = noun;
    cached_command_list[2] = verb;

    for x in (0..cached_command_list.len() - 1).step_by(4){
        match cached_command_list[x] {
            99 => break,
            1 => {
                let out_pos = cached_command_list[x+3];
                let in_pos_1 = cached_command_list[x+1];
                let in_pos_2 = cached_command_list[x+2];
                cached_command_list[out_pos as usize] = cached_command_list[in_pos_1 as usize] + cached_command_list[in_pos_2 as usize]
            }
            2 => {
                let out_pos = cached_command_list[x+3];
                let in_pos_1 = cached_command_list[x+1];
                let in_pos_2 = cached_command_list[x+2];
                cached_command_list[out_pos as usize] = cached_command_list[in_pos_1 as usize] * cached_command_list[in_pos_2 as usize]
            }
            _ => panic!("Unrecognized opcode")
        }
    }

    cached_command_list[0]
}

fn search_protocol(_command_list : &Vec<i32>, final_val: i32) -> i32
{
    let mut out_protocol : i32 = 0;

    for i in 0..99
    {
        for j in 0..99 {
            let result = run_opcode_instructions(_command_list, i, j);
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

    let part1_result = run_opcode_instructions(&memory_values, 12, 2);
    let part2_result = search_protocol(&memory_values, 19690720);

    println!("1202 result = {}", part1_result);
    println!("19690720 result = {}", part2_result);

}