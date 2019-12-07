use std::fs;

fn main() {
    let filename = "input.txt";

    let _file_contents = fs::read_to_string(filename).unwrap();
    let computer_commands : Vec<u16> = _file_contents
                                    .split(",")
                                    .map(|s| s.trim().parse().unwrap())
                                    .collect();

    for x in (0..computer_commands.len() - 1).step_by(4){
        match computer_commands[x] {
            _ => panic!("Test exception")
        }
    }
}