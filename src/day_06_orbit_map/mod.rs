use std::collections::HashMap;
use std::fs;

struct SpaceObject {
  name: String,
  orbiting: Option<&SpaceObject>,
  distance: u32,
}

fn parse_uom(input: String) -> HashMap<String, SpaceObject> {
  let mut objects_in_space = HashMap::new();
  objects_in_space.insert(
    "COM".to_string(),
    SpaceObject {
      name: "COM".to_string(),
      orbiting: None,
      distance: 0,
    },
  );

  input
    .lines()
    .map(|orbit| orbit.split(')').collect())
    .for_each(|[center_identifier, orbiting]| {
      let center = objects_in_space.get(center_identifier);
      let distance = match center {
        Some(object) => object.distance + 1,
        None => 0,
      };
      objects_in_space.insert(
        orbiting,
        SpaceObject {
          name: orbiting,
          orbiting: center,
          distance: distance,
        },
      );
    });

  return objects_in_space;
}

fn load_uom() -> HashMap<String, SpaceObject> {
  let input = fs::read_to_string("./src/day_06_orbit_map/input.txt")
    .expect("Something went wrong reading the input file from the day-folder:");

  return parse_uom(input);
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
"
    .to_string();
    assert_eq!(count_orbits(parse_uom(input)), 42);
  }
}
