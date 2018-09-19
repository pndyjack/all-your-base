#[derive(Debug, PartialEq)]
pub enum Error {
  InvalidInputBase,
  InvalidOutputBase,
  InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
  if from_base == 1 || from_base == 0 {
    return Err(Error::InvalidInputBase);
  }
  if to_base == 1 || to_base == 0 {
    return Err(Error::InvalidOutputBase);
  }
  if from_base == to_base {
    return Ok(number.to_vec());
  }
  if number.to_vec().is_empty() {
    return Ok(vec![]);
  }
  if from_base == 10 {
    match convert_helper(number, from_base, to_base) {
      Ok(value) => return Ok(value),
      Err(err) => return Err(err),
    }
  } else if to_base == 10 {
    match convert_helper(number, from_base, to_base) {
      Ok(value) => return Ok(value),
      Err(err) => return Err(err),
    }
  } else {
    match convert_helper(number, from_base, 10) {
      Ok(value) => match convert_helper(&value, 10, to_base) {
        Ok(value) => return Ok(value),
        Err(err) => return Err(err),
      },
      Err(err) => return Err(err),
    }
  }
}

fn convert_helper(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
  let mut error = (false, 0);
  let mut new_value: u32 = number
    .iter()
    .enumerate()
    .map(|(index, &value)| {
      if value >= from_base {
        error = (true, value);
      }
      from_base.pow((number.len() - 1 - index) as u32) * value
    })
    .sum();
  if error.0 {
    return Err(Error::InvalidDigit(error.1));
  }
  let mut new_value_vec: Vec<u32> = Vec::new();
  while new_value / to_base != 0 {
    new_value_vec.push(new_value % to_base);
    new_value = new_value / to_base;
  }
  if new_value < to_base {
    new_value_vec.push(new_value);
  }
  new_value_vec.reverse();
  new_value_vec = new_value_vec
    .into_iter()
    .skip_while(|&digit| digit == 0)
    .collect();
  return Ok(new_value_vec);
}
