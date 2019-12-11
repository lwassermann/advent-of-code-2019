use std::fs;

fn read_input() -> String {
  fs::read_to_string("./src/day_01_fuel_calculations/input.txt")
    .expect("Something went wrong reading the input file from the day-folder:")
}

fn get_module_fuel(weight: u32) -> u32 {
  weight / 3 - 2
}

fn get_total_module_fuel(weight: u32) -> u32 {
  let third = weight / 3;
  if third <= 2 { 0 }
  else {
    let fuel_weight = get_module_fuel(weight);
    fuel_weight + get_total_module_fuel(fuel_weight)
  }
}

pub fn calculate() {
  let input = read_input();
  let module_weights = input
    .lines()
    .map(|line| line.parse::<u32>().unwrap());

  let module_fuel: u32 = module_weights.clone().map(get_module_fuel).sum();
  println!("Required fuel for modules: {}", module_fuel);

  let total_fuel: u32 = module_weights.map(get_total_module_fuel).sum();
  println!("Required total fuel: {}", total_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moderate_module() {
        assert_eq!(get_total_module_fuel(1969), 966);
    }

    #[test]
    fn test_big_module() {
        assert_eq!(get_total_module_fuel(100756), 50346);
    }
}
