mod intcode;
use std::fs;

fn load_memory() -> Vec<i64> {
  let input = fs::read_to_string("./src/day_05_intcode_io/input.txt")
    .expect("Something went wrong reading the input file from the day-folder:");
  input
    .trim()
    .split(",")
    .map(|cell| cell.parse::<i64>().unwrap())
    .collect()
}

pub fn calculate() {
  let mut memory = load_memory();
  intcode::interpret(&mut memory, &[1]);
}
