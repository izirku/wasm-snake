#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn opposite(&self, other: Direction) -> bool {
    match (self, other) {
      (Direction::Up, Direction::Down) => true,
      (Direction::Down, Direction::Up) => true,
      (Direction::Left, Direction::Right) => true,
      (Direction::Right, Direction::Left) => true,
      _ => false,
    }
  }
}
