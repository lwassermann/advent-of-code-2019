fn has_double_digit(n: u32) -> bool {
  let str = format!("{}", n);
  let mut digits = str.chars().peekable();
  while let Some(digit) = digits.next() {
    if let Some(next_digit) = digits.peek() {
      if digit == *next_digit {
        return true;
      }
    }
  }
  false
}

fn is_monotonous(n: u32) -> bool {
  let str = format!("{}", n);
  let mut digits = str.chars().peekable();
  while let Some(digit) = digits.next() {
    if let Some(next_digit) = digits.peek() {
      if digit > *next_digit {
        return false;
      }
    }
  }
  true
}

pub fn calculate() {
  let mut candidates = 0;
  for n in 382345..843168 {
    if is_monotonous(n) && has_double_digit(n) {
      candidates += 1;
    }
  }
  println!("382345-843167 has {} candidates for passwords", candidates);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_double_digit() {
    assert!(has_double_digit(11));
    assert!(has_double_digit(1335));
    assert!(has_double_digit(462443));

    assert!(!has_double_digit(12));
    assert!(!has_double_digit(123456));
    assert!(!has_double_digit(12456));
    assert!(!has_double_digit(12456));
  }

  #[test]
  fn test_monotonousness() {
    assert!(is_monotonous(11));
    assert!(is_monotonous(1335));
    assert!(is_monotonous(12));
    assert!(is_monotonous(123456));
    assert!(is_monotonous(12456));
    assert!(is_monotonous(12456));

    assert!(!is_monotonous(462443));
    assert!(!is_monotonous(12343456));
  }
}
