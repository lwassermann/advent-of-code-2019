fn load_memory(into: &mut Vec<u8>) {
  into.push(99);
}

fn interpret(memory: &mut [u8]) {
  let position: usize = 0;
  loop {
    let opcode = memory[position];
    if opcode == 99 { return }
    panic!("Unknown opcode {}", opcode)
  }
}

pub fn calculate() {
  let mut memory = Vec::new();
  load_memory(&mut memory);

  interpret(&mut memory);
  println!("Value at position 0: {}", memory[0])
}
