fn lookup(memory: &[i64], address: usize) -> i64 {
  let lvalue = memory[address] as usize;
  memory[lvalue]
}

fn set(memory: &mut [i64], address: usize, value: i64) {
  let lvalue = memory[address] as usize;
  memory[lvalue] = value;
}

pub fn interpret(memory: &mut [i64], answers: &[i64]) {
  let mut instruction_pointer: usize = 0;
  let mut input = answers.into_iter();
  loop {
    let opcode = memory[instruction_pointer];
    match opcode {
      1 => {
        set(
          memory,
          instruction_pointer + 3,
          lookup(memory, instruction_pointer + 1) + lookup(memory, instruction_pointer + 2),
        );
        instruction_pointer += 4;
      }
      2 => {
        set(
          memory,
          instruction_pointer + 3,
          lookup(memory, instruction_pointer + 1) * lookup(memory, instruction_pointer + 2),
        );
        instruction_pointer += 4;
      }
      3 => {
        let value = *input.next().expect("Not enough input provided");
        set(memory, instruction_pointer + 1, value);
        instruction_pointer += 2;
      }
      4 => {
        println!("{}", lookup(memory, instruction_pointer + 1));
        instruction_pointer += 2;
      }
      99 => {
        return;
        // instruction_pointer += 1;
      }
      _ => panic!(
        "Unknown opcode {} at address {}",
        opcode, instruction_pointer
      ),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_exit() {
    let mut memory: &mut [i64] = &mut [99];
    interpret(&mut memory, &[1]);
    assert_eq!(memory, [99]);
  }

  #[test]
  fn test_addition() {
    let mut memory: &mut [i64] = &mut [1, 0, 0, 0, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [2, 0, 0, 0, 99]);
  }

  #[test]
  fn test_multiplication() {
    let mut memory: &mut [i64] = &mut [2, 3, 0, 3, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [2, 3, 0, 6, 99]);
  }

  #[test]
  fn test_combination() {
    let mut memory: &mut [i64] = &mut [1, 1, 1, 4, 99, 5, 6, 0, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
  }

  #[test]
  fn test_read_input() {
    let mut memory: &mut [i64] = &mut [3, 0, 99];
    interpret(&mut memory, &[99]);
    assert_eq!(memory, [99, 0, 99]);
  }

  #[test]
  fn test_print_output() {
    let mut memory: &mut [i64] = &mut [4, 2, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [4, 2, 99]);
  }
}
