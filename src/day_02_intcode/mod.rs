fn load_memory(into: &mut Vec<u8>) {
  into.push(99);
}

fn lookup(memory: &[u8], position: usize) -> u8 {
  let lvalue = memory[position] as usize;
  memory[lvalue]
}

fn set(memory: &mut [u8], position: usize, value: u8) {
  let lvalue = memory[position] as usize;
  memory[lvalue] = value;
}

fn interpret(memory: &mut [u8]) {
  let mut position: usize = 0;
  loop {
    let opcode = memory[position];
    if opcode == 99 {
      return;
    } else if opcode == 1 {
      set(
        memory,
        position + 3,
        lookup(memory, position + 1) + lookup(memory, position + 2),
      );
      position += 4;
    } else if opcode == 2 {
      set(
        memory,
        position + 3,
        lookup(memory, position + 1) * lookup(memory, position + 2),
      );
      position += 4;
    } else {
      panic!("Unknown opcode {}", opcode)
    }
  }
}

pub fn calculate() {
  let mut memory = Vec::new();
  load_memory(&mut memory);

  interpret(&mut memory);
  println!("Value at position 0: {}", memory[0])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_exit() {
    let mut memory: &mut [u8] = &mut [99];
    interpret(&mut memory);
    assert_eq!(memory, [99]);
  }

  #[test]
  fn test_addition() {
    let mut memory: &mut [u8] = &mut [1, 0, 0, 0, 99];
    interpret(&mut memory);
    assert_eq!(memory, [2, 0, 0, 0, 99]);
  }

  #[test]
  fn test_multiplication() {
    let mut memory: &mut [u8] = &mut [2, 3, 0, 3, 99];
    interpret(&mut memory);
    assert_eq!(memory, [2, 3, 0, 6, 99]);
  }
}
