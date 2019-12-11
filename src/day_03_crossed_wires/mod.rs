use std::fs;

fn contains(x: i32, a: i32, b: i32) -> bool {
  if a <= b {
    x >= a && x <= b
  } else {
    contains(x, b, a)
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Edge {
  from: Point,
  to: Point,
}

impl Edge {
  fn intersect(&self, other: &Edge) -> Option<Point> {
    if self.horizontal() == other.horizontal() {
      return None;
    }

    if self.horizontal() {
      if contains(self.from.y, other.from.y, other.to.y)
        && contains(other.from.x, self.from.x, self.to.x)
      {
        Some(Point {
          x: other.from.x,
          y: self.from.y,
        })
      } else {
        None
      }
    } else {
      other.intersect(&self)
    }
  }

  fn horizontal(&self) -> bool {
    self.from.y == self.to.y
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn distance(&self) -> i32 {
    self.x.abs() + self.y.abs()
  }
}

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

type Circuit = (Vec<Edge>, Vec<Edge>);

fn parse_dir(indicator: char) -> Direction {
  match indicator {
    'R' => Direction::Right,
    'L' => Direction::Left,
    'U' => Direction::Up,
    'D' => Direction::Down,
    _ => panic!("This edge has an unknown direction: {}", indicator),
  }
}

fn translate(point: &Point, x: i32, y: i32) -> Point {
  Point {
    x: point.x + x,
    y: point.y + y,
  }
}

fn layout_edge(from: Point, movement: &str) -> Edge {
  let direction = parse_dir(movement.chars().next().unwrap());
  let distance = movement[1..]
    .parse::<i32>()
    .expect("Can not parse distance");

  match direction {
    Direction::Right => Edge {
      from,
      to: translate(&from, distance, 0),
    },
    Direction::Left => Edge {
      from,
      to: translate(&from, -distance, 0),
    },
    Direction::Up => Edge {
      from,
      to: translate(&from, 0, distance),
    },
    Direction::Down => Edge {
      from,
      to: translate(&from, 0, -distance),
    },
  }
}

fn layout_wire<I: Iterator>(directions: I) -> Vec<Edge>
where
  I::Item: AsRef<str>,
{
  directions
    .scan(Point { x: 0, y: 0 }, |wire_end, direction| {
      let edge = layout_edge(*wire_end, direction.as_ref());
      *wire_end = edge.to;
      Some(edge)
    })
    .collect()
}

fn layout_circuit(wires: (&str, &str)) -> Circuit {
  (
    layout_wire(wires.0.split(",")),
    layout_wire(wires.1.split(",")),
  )
}

fn crossings(circuit: &Circuit) -> Vec<Point> {
  let (a, b) = circuit;
  b.into_iter()
    .flat_map(|edge_b| {
      a.into_iter()
        .filter_map(move |edge_a| edge_b.intersect(&edge_a))
    })
    .skip(1) // All wires start in 0,0
    .collect()
}

fn get_closest_crossing(circuit: &Circuit) -> i32 {
  crossings(circuit)
    .into_iter()
    .fold(None, |maybe_distance, crossing| match maybe_distance {
      None => Some(crossing.distance()),
      Some(distance) => Some(distance.min(crossing.distance())),
    })
    .unwrap()
}

pub fn calculate() {
  println!("Hello World!")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_layout_edge() {
    assert_eq!(
      layout_edge(Point { x: 0, y: 0 }, "R75"),
      Edge {
        from: Point { x: 0, y: 0 },
        to: Point { x: 75, y: 0 },
      }
    );
  }

  #[test]
  fn test_intersection() {
    let edge = Edge {
      from: Point { x: 0, y: 0 },
      to: Point { x: 75, y: 0 },
    };
    assert_eq!(
      edge.intersect(&Edge {
        from: Point { x: 0, y: 2 },
        to: Point { x: 75, y: 2 },
      }),
      None
    );

    assert_eq!(
      edge.intersect(&Edge {
        from: Point { x: 0, y: 0 },
        to: Point { x: 0, y: 75 },
      }),
      Some(Point { x: 0, y: 0 })
    );

    assert_eq!(
      edge.intersect(&Edge {
        from: Point { x: 20, y: 0 },
        to: Point { x: 20, y: 75 },
      }),
      Some(Point { x: 20, y: 0 })
    );

    assert_eq!(
      edge.intersect(&Edge {
        from: Point { x: 76, y: 75 },
        to: Point { x: 76, y: 0 },
      }),
      None
    );
  }

  #[test]
  fn test_simple() {
    let wires: (&str, &str) = (
      "R75,D30,R83,U83,L12,D49,R71,U7,L72",
      "U62,R66,U55,R34,D71,R55,D58,R83",
    );
    let circuit = layout_circuit(wires);
    println!("{:?}", crossings(&circuit));
    assert_eq!(get_closest_crossing(&circuit), 159);
  }

  #[test]
  fn test_moderate() {
    let wires: (&str, &str) = (
      "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
      "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    );
    let circuit = layout_circuit(wires);
    println!("{:?}", crossings(&circuit));
    assert_eq!(get_closest_crossing(&circuit), 135);
  }
}
