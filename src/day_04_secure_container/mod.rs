fn get_digits(n: u32) -> [u8; 6] {
  [
    (n / 100000 % 10) as u8,
    (n / 10000 % 10) as u8,
    (n / 1000 % 10) as u8,
    (n / 100 % 10) as u8,
    (n / 10 % 10) as u8,
    (n / 1 % 10) as u8,
  ]
}

fn has_double_digit(n: u32) -> bool {
  get_digits(n).windows(2).any(|w| w[0] == w[1])
}

fn has_pair(n: u32) -> bool {
  let digits = get_digits(n);
  let extended_digits: [u8; 8] = [
    // 🤔
    0, digits[0], digits[1], digits[2], digits[3], digits[4], digits[5], 0,
  ];
  extended_digits
    .windows(4)
    .any(|w| w[0] != w[1] && w[1] == w[2] && w[2] != w[3])
}

fn is_monotonous(n: u32) -> bool {
  get_digits(n).windows(2).all(|w| w[0] <= w[1])
}

pub fn calculate() {
  let mut candidates = 0;
  let mut improved = 0;
  for n in 382345..843168 {
    if is_monotonous(n) && has_double_digit(n) {
      candidates += 1;
      if has_pair(n) {
        improved += 1;
      }
    }
  }
  println!(
    "382345-843167 has {} candidates for passwords, with the additional requirements, it's just {}",
    candidates, improved
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_double_digit() {
    assert!(has_double_digit(11));
    assert!(has_double_digit(1335));
    assert!(has_double_digit(462443));

    assert!(!has_double_digit(24578));
    assert!(!has_double_digit(123456));
    assert!(!has_double_digit(12456));
  }

  #[test]
  fn test_has_pair() {
    assert!(has_pair(1123));
    assert!(has_pair(13358));
    assert!(has_pair(462443));
    assert!(has_pair(111223));

    assert!(!has_pair(555679));
    assert!(!has_pair(244457));
    assert!(!has_pair(124666));
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
