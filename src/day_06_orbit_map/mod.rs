use std::collections::HashMap;
use std::fs;

type SolarSystem = HashMap<String, SpaceObject>;

struct SpaceObject<'a> {
  name: String,
  orbiting: Option<&'a SpaceObject<'a>>,
  distance: u32,
}

fn parse_orbit(line: &str) -> (&str, &str) {
  let parts = line.split(')');
  let a = parts.next().unwrap();
  let b = parts.next().unwrap();
  assert!(parts.next().is_none());
  return (a, b);
}

fn parse_uom<'a>(input: String) -> SolarSystem<'a> {
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
    .map(|orbit| parse_orbit(orbit))
    .for_each(|(center_identifier, orbiting)| {
      let center = objects_in_space.get(&center_identifier.to_string());
      let distance = match center {
        Some(object) => object.distance + 1,
        None => 0,
      };
      objects_in_space.insert(
        orbiting.to_string(),
        SpaceObject {
          name: orbiting.to_string(),
          orbiting: center,
          distance: distance,
        },
      );
    });

  return objects_in_space;
}

fn load_uom<'a>() -> SolarSystem<'a> {
  let input = fs::read_to_string("./src/day_06_orbit_map/input.txt")
    .expect("Something went wrong reading the input file from the day-folder:");

  return parse_uom(input);
}

fn count_orbits(universe: &SolarSystem) -> u32 {
  return 1;
}

pub fn calculate() {
  let universe = load_uom();
  println!("Orbits: {}", count_orbits(&universe));
}

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
    assert_eq!(count_orbits(&parse_uom(input)), 42);
  }
}
