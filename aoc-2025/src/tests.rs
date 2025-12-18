use super::*;

#[test]
fn test_apply_rotation_to_dial() {
  let current_position = 42;
  let rotation = ("R", 10);
  assert_eq!(apply_rotation_to_dial(current_position, &rotation), 52);
}

#[test]
fn test_apply_rotation_to_dial_zero() {
  let current_position = 42;
  let rotation = ("L", 42);
  assert_eq!(apply_rotation_to_dial(current_position, &rotation), 0);
}
#[test]
fn test_apply_rotation_to_dial_under_limit() {
  let current_position= 42;
  let rotation = ("L", 43);
  assert_eq!(apply_rotation_to_dial(current_position, &rotation), 99);
}

#[test]
fn test_apply_rotation_to_dial_over_limit() {
  let current_position = 98;
  let rotation = ("R", 5);
  assert_eq!(apply_rotation_to_dial(current_position, &rotation), 3);
}


#[test]
fn test_find_out_password() {
  let rotations = [
    ("L", 68),
    ("L", 30),
    ("R", 48),
    ("L", 5),
    ("R", 60),
    ("L", 55),
    ("L", 1),
    ("L", 99),
    ("R", 14),
    ("L", 82),
  ];
  assert_eq!(find_out_password(&rotations), 3)
}