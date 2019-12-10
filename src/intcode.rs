pub mod intcode {

    pub trait IntCodeCompute {
        fn run_opcode_instructions(&mut self, noun : i32, verb : i32) -> i32;
        fn run_program(&mut self) -> i32;
        fn get_arguments(&self, _index : usize) -> Vec<i32>;
    }

    pub struct IntCodeMemory {
        command_list : Vec<i32>
    }

    impl IntCodeMemory {
        pub fn new(command_list : &Vec<i32>) -> IntCodeMemory {
            IntCodeMemory {
                command_list: command_list.to_vec()
            }
        }
    }

    impl IntCodeCompute for IntCodeMemory {

        fn run_opcode_instructions(&mut self, noun : i32, verb : i32) -> i32 {

            self.command_list[1] = noun;
            self.command_list[2] = verb;
        
            self.run_program()
        }

        fn run_program(&mut self) -> i32 {
            let mut x : usize = 0;
            loop{
                match self.command_list[x] % 100 {
                    99 => break,
                    01 => {
                        let out_pos = self.command_list[x+3];
                        let in_args = self.get_arguments(x);
                        self.command_list[out_pos as usize] = in_args[0] + in_args[1];
                        x += 4;
                    }
                    02 => {
                        let out_pos = self.command_list[x+3];
                        let in_args = self.get_arguments(x);
                        self.command_list[out_pos as usize] = in_args[0] * in_args[1];
                        x += 4;
                    }
                    03 => {
    
                        x += 2;
                    }
                    _ => panic!("Unrecognized opcode {} {}", self.command_list[x], x)
                }
            }

            self.command_list[0]
        }

        fn get_arguments(&self, _index : usize) -> Vec<i32> {
            
            let op_code = self.command_list[_index] as usize;
            let mut return_value : Vec<i32> = [0,0].to_vec();
            let temp1 = self.command_list[_index + 1];
            let temp2 = self.command_list[_index + 2];
            if ((op_code % 1000) / 100) == 1 {
                return_value[0] = temp1;
            }
            else
            {
                return_value[0] = self.command_list[temp1 as usize];
            }


            if (op_code) / 1000 == 1 {
                return_value[1] = temp2;
            }
            else {
                return_value[1] = self.command_list[temp2 as usize];
            }

            return_value
        }
    }
}