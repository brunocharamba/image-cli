use strum_macros;

#[derive(strum_macros::Display, PartialEq)]
pub enum Direction {
  #[strum(serialize = "left")]
  Left,
  #[strum(serialize = "right")]
  Right,
}
