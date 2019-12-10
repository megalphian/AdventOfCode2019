use std::fs;

fn calculate_fuel( mass : i32) -> i32 {
    (mass/3 - 2)
}

fn main() {
  let filename = "data/input1.txt";
  
  let mut _part_1_total_fuel : i32 = 0;
  let mut _part_2_total_fuel : i32 = 0;
  
  let _file_contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
  let module_masses : Vec<&str> = _file_contents.split("\n").collect();

  for i  in 0..(module_masses.len()-1) {
    
    let _module : i32 = module_masses[i].parse::<i32>().unwrap();
    let mut _temp_fuel : i32 = calculate_fuel(_module);

    _part_1_total_fuel += _temp_fuel;

    while _temp_fuel > 0
    {
        _part_2_total_fuel += _temp_fuel;
        _temp_fuel = calculate_fuel(_temp_fuel);
    }

  }

  println!("Part 1 Answer : {}", _part_1_total_fuel);
  println!("Part 2 Answer : {}", _part_2_total_fuel);
}