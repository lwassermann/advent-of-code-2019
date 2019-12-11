use std::fs;
use std::iter::FromIterator;

fn load_memory(into: &mut Vec<u64>) {
  let input = fs::read_to_string("./src/day_02_intcode/input.txt")
    .expect("Something went wrong reading the input file from the day-folder:");
  let program = Vec::from_iter(
    input
      .trim()
      .split(",")
      .map(|cell| cell.parse::<u64>().unwrap()),
  );
  into.extend_from_slice(&program);
}

fn lookup(memory: &[u64], address: usize) -> u64 {
  let lvalue = memory[address] as usize;
  memory[lvalue]
}

fn set(memory: &mut [u64], address: usize, value: u64) {
  let lvalue = memory[address] as usize;
  memory[lvalue] = value;
}

fn interpret(memory: &mut [u64]) {
  let mut instruction_pointer: usize = 0;
  loop {
    let opcode = memory[instruction_pointer];
    if opcode == 99 {
      return;
    } else if opcode == 1 {
      set(
        memory,
        instruction_pointer + 3,
        lookup(memory, instruction_pointer + 1) + lookup(memory, instruction_pointer + 2),
      );
      instruction_pointer += 4;
    } else if opcode == 2 {
      set(
        memory,
        instruction_pointer + 3,
        lookup(memory, instruction_pointer + 1) * lookup(memory, instruction_pointer + 2),
      );
      instruction_pointer += 4;
    } else {
      panic!("Unknown opcode {}", opcode)
    }
  }
}

fn find_trajectory() {
  let mut memory = Vec::new();
  load_memory(&mut memory);

  memory[1] = 12;
  memory[2] = 2;

  interpret(&mut memory);
  println!("Value at address 0: {}", memory[0])
}

fn find_error_input() {
  let mut memory = Vec::new();
  load_memory(&mut memory);

  for noun in 0..99 {
    for verb in 0..99 {
      let mut attempt = memory.clone();
      attempt[1] = noun;
      attempt[2] = verb;
      interpret(&mut attempt);
      if attempt[0] == 19690720 {
        println!("Found parameters: {} {}", noun, verb);
        return;
      }
    }
  }
}

pub fn calculate() {
  find_trajectory();
  find_error_input();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_exit() {
    let mut memory: &mut [u64] = &mut [99];
    interpret(&mut memory);
    assert_eq!(memory, [99]);
  }

  #[test]
  fn test_addition() {
    let mut memory: &mut [u64] = &mut [1, 0, 0, 0, 99];
    interpret(&mut memory);
    assert_eq!(memory, [2, 0, 0, 0, 99]);
  }

  #[test]
  fn test_multiplication() {
    let mut memory: &mut [u64] = &mut [2, 3, 0, 3, 99];
    interpret(&mut memory);
    assert_eq!(memory, [2, 3, 0, 6, 99]);
  }

  #[test]
  fn test_combination() {
    let mut memory: &mut [u64] = &mut [1, 1, 1, 4, 99, 5, 6, 0, 99];
    interpret(&mut memory);
    assert_eq!(memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
  }
}
