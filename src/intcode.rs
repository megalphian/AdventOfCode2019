pub mod intcode {

    pub trait ProgramNounVerb {
        fn run_opcode_instructions(&self, noun : i32, verb : i32) -> i32;
    }

    pub struct IntCodeMemory {
        _command_list : Vec<i32>
    }

    fn run_program(mut cmds: Vec<i32>) -> i32 {
        for x in (0..cmds.len() - 1).step_by(4){
            match cmds[x] {
                99 => break,
                1 => {
                    let out_pos = cmds[x+3];
                    let in_pos_1 = cmds[x+1];
                    let in_pos_2 = cmds[x+2];
                    cmds[out_pos as usize] = cmds[in_pos_1 as usize] + cmds[in_pos_2 as usize]
                }
                2 => {
                    let out_pos = cmds[x+3];
                    let in_pos_1 = cmds[x+1];
                    let in_pos_2 = cmds[x+2];
                    cmds[out_pos as usize] = cmds[in_pos_1 as usize] * cmds[in_pos_2 as usize]
                }
                _ => panic!("Unrecognized opcode")
            }
        }

        cmds[0]
    }

    impl IntCodeMemory {
        pub fn new(_command_list : &Vec<i32>) -> IntCodeMemory {
            IntCodeMemory {
                _command_list: _command_list.to_vec()
            }
        }
    }

    impl ProgramNounVerb for IntCodeMemory {

        fn run_opcode_instructions(&self, noun : i32, verb : i32) -> i32 {
    
            let mut cached_command_list : Vec<i32> = self._command_list.clone();
            cached_command_list[1] = noun;
            cached_command_list[2] = verb;
        
            run_program(cached_command_list)
        
        }
    }
}