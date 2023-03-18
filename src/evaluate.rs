pub fn evaluate(password: &str) -> u32 {
  let mut has_upper: bool = false;
  let mut has_number: bool = false;
  let mut has_symbol: bool = false;
  let mut score: u32 = 0;
  
  for char in password.chars() {
    if char.is_uppercase() & !has_upper { score += 1; has_upper = true; }
    else if char.is_numeric() & !has_number { score += 1; has_number = true; }
    else if !char.is_alphanumeric() & !has_symbol { score += 1; has_symbol = true; }
  }

  if password.len() >= 8 { score += 1 };
  
  return score;
}