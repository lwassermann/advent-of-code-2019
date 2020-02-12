use std::fs;

fn load_uom() -> Vec<i64> {
  let input = fs::read_to_string("./src/day_06_orbit_map/input.txt")
    .expect("Something went wrong reading the input file from the day-folder:");
  input.lines().map(|orbit| orbit.split(')')).collect()
}

pub fn calculate() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_orbit_counts() {
    let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";
    assert_eq!(count_orbits(input), 42);
  }
}
