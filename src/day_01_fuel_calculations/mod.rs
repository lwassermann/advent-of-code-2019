use std::fs;

fn read_input() -> String {
  return fs::read_to_string("./src/day_01_fuel_calculations/input.txt")
    .expect("Something went wrong reading the input file from the day-folder:");
}

pub fn calculate() {
  let contents = read_input();
  println!("{}", contents);
}
