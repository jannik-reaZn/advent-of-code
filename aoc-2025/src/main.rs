const DIAL_MAX_NUMBER_OF_POSITIONS: i32 = 99;

fn main() {
  let rotations = [("L", 20)];
  find_out_password(&rotations);
}

/// Calulates password for the dial.
/// This is done by counting how often the dial points to 0.
fn find_out_password(rotations: &[(&str, i32)]) -> u32 {
  let mut password: u32 = 0;
  // The current position always starts at 50
  let mut current_position: i32 = 50;

  for rotation in rotations {
    current_position = apply_rotation_to_dial(current_position, &rotation);
    if current_position == 0 {
      password += 1
    }
  }

  password
}

fn apply_rotation_to_dial(current_position: i32, rotation: &(&str, i32)) -> i32 {
  let mut rotation_number: i32 = rotation.1;
  if rotation.0 == "L" {
    rotation_number = rotation.1 * -1;
  }

  let applied_rotation: i32 = current_position + rotation_number;
  let mut _final_position: i32 = applied_rotation;

  // NOTE: The assumption is that the number of rotations is between 0-99
  if applied_rotation < 0 {
    _final_position = DIAL_MAX_NUMBER_OF_POSITIONS + 1 + applied_rotation;
  } else if applied_rotation >= DIAL_MAX_NUMBER_OF_POSITIONS {
    _final_position = applied_rotation - DIAL_MAX_NUMBER_OF_POSITIONS - 1;
  } 

  return _final_position;
}

#[cfg(test)]
mod tests;