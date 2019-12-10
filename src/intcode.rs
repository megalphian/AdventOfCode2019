pub mod intcode {

    pub trait IntCodeCompute {
        fn run_opcode_instructions(&self, noun : i32, verb : i32) -> i32;
    }

    pub struct IntCodeMemory {
        _command_list : Vec<i32>
    }

    impl IntCodeMemory {
        pub fn new(_command_list : &Vec<i32>) -> IntCodeMemory {
            IntCodeMemory {
                _command_list: _command_list.to_vec()
            }
        }
    }

    impl IntCodeCompute for IntCodeMemory {

        fn run_opcode_instructions(&self, noun : i32, verb : i32) -> i32 {
    
            let mut cached_command_list : Vec<i32> = self._command_list.clone();
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
    }
}