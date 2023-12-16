#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RotationalDirection {
    Clockwise,
    Anticlockwise,
}

impl RotationalDirection {
    pub fn from_incoming_and_outgoing(incoming: &Direction, outgoing: &Direction) -> Option<Self> {
        match (*outgoing as u8 + 4 - *incoming as u8) % 4 {
            0 => None,
            1 => Some(RotationalDirection::Clockwise),
            2 => None,
            3 => Some(RotationalDirection::Anticlockwise),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

pub trait Position {
    fn y(&self) -> usize;
    fn x(&self) -> usize;

    #[must_use]
    fn with_direction(self, direction: &Direction) -> Self;
}

impl Direction {
    #[must_use]
    pub fn with_rotation(self, rotational_direction: &RotationalDirection) -> Self {
        match (self, rotational_direction) {
            (Direction::Up, RotationalDirection::Clockwise) => Direction::Right,
            (Direction::Up, RotationalDirection::Anticlockwise) => Direction::Left,
            (Direction::Down, RotationalDirection::Clockwise) => Direction::Left,
            (Direction::Down, RotationalDirection::Anticlockwise) => Direction::Right,
            (Direction::Right, RotationalDirection::Clockwise) => Direction::Down,
            (Direction::Right, RotationalDirection::Anticlockwise) => Direction::Up,
            (Direction::Left, RotationalDirection::Clockwise) => Direction::Up,
            (Direction::Left, RotationalDirection::Anticlockwise) => Direction::Down,
        }
    }
}
