pub mod intcode {

    pub trait IntCodeCompute {
        fn run_opcode_instructions(&mut self, noun : i32, verb : i32) -> i32;
        fn run_program(&mut self) -> i32;
        fn get_arguments(&self, _index : usize, multi_parameter: bool) -> Vec<usize>;
    }

    pub struct IntCodeMemory {
        command_list : Vec<i32>,
        inputs : Vec<i32>
    }

    impl IntCodeMemory {
        pub fn _new(command_list : &Vec<i32>) -> IntCodeMemory {
            IntCodeMemory {
                command_list: command_list.to_vec(),
                inputs: Vec::<i32>::new()
            }
        }

        pub fn _new_with_input(command_list : &Vec<i32>, inputs : &Vec<i32>) -> IntCodeMemory {
            IntCodeMemory {
                command_list: command_list.to_vec(),
                inputs: inputs.to_vec()
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
            let mut input_index : usize = 0;
            let mut outputs = Vec::<i32>::new();

            loop{
                match self.command_list[x] % 100 {
                    99 => break, //HALT
                    01 => { //ADD
                        let out_pos = self.command_list[x+3];
                        let in_args = self.get_arguments(x, true);
                        self.command_list[out_pos as usize] = self.command_list[in_args[0]] + self.command_list[in_args[1]];
                        x += 4;
                    }
                    02 => { //MULTIPLY
                        let out_pos = self.command_list[x+3];
                        let in_args = self.get_arguments(x, true);
                        self.command_list[out_pos as usize] = self.command_list[in_args[0]] * self.command_list[in_args[1]];
                        x += 4;
                    }
                    03 => { //INPUT
                        let in_args = self.get_arguments(x, false);
                        self.command_list[in_args[0]] = self.inputs[input_index];
                        input_index += 1;
                        x += 2;
                    }
                    04 => { //OUTPUT
                        let in_args = self.get_arguments(x, false);
                        outputs.push(self.command_list[in_args[0]]);
                        x += 2;
                    }   
                    05 => { //JUMP-IF-TRUE
                        let in_args = self.get_arguments(x, true);
                        if self.command_list[in_args[0]] != 0 { x = self.command_list[in_args[1]] as usize }
                        else { x += 3 }
                    }
                    06 => { //JUMP-IF-FALSE
                        let in_args = self.get_arguments(x, true);
                        if self.command_list[in_args[0]] == 0 { x = self.command_list[in_args[1]] as usize }
                        else { x += 3 }
                    }
                    07 => { //LESS-THAN
                        let out_pos = self.command_list[x+3];
                        let in_args = self.get_arguments(x, true);
                        if self.command_list[in_args[0]] < self.command_list[in_args[1]] {self.command_list[out_pos as usize] = 1}
                        else { self.command_list[out_pos as usize] = 0 }
                        x += 4
                    }
                    08 => { //EQUALS
                        let out_pos = self.command_list[x+3];
                        let in_args = self.get_arguments(x, true);
                        if self.command_list[in_args[0]] == self.command_list[in_args[1]] {self.command_list[out_pos as usize] = 1}
                        else { self.command_list[out_pos as usize] = 0 }
                        x += 4
                    }
                    _ => panic!("Unrecognized opcode {} {}", self.command_list[x], x)
                }
            }

            println!("Outputs: {:?}", outputs);

            self.command_list[0]
        }

        fn get_arguments(&self, _index : usize, multi_parameter: bool) -> Vec<usize> {
            
            let op_code = self.command_list[_index] as usize;
            let mut return_value : Vec<usize> = [0,0].to_vec();
            let temp1 = self.command_list[_index + 1];
            let temp2 = self.command_list[_index + 2];
            if ((op_code % 1000) / 100) == 1 {
                return_value[0] = _index + 1;
            }
            else
            {
                return_value[0] = temp1 as usize;
            }

            if multi_parameter {
                if (op_code) / 1000 == 1 {
                    return_value[1] = _index + 2;
                }
                else {
                    return_value[1] = temp2 as usize;
                }
            }
            

            return_value
        }
    }
}