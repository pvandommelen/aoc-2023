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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Dimensions(usize, usize);
impl Dimensions {
    pub fn height(&self) -> usize {
        self.0
    }
    pub fn width(&self) -> usize {
        self.1
    }
}

impl From<Dimensions> for (usize, usize) {
    fn from(value: Dimensions) -> Self {
        (value.height(), value.width())
    }
}

impl From<(usize, usize)> for Dimensions {
    fn from(value: (usize, usize)) -> Self {
        Dimensions(value.0, value.1)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn from_yx(y: usize, x: usize) -> Self {
        Self(y, x)
    }
    pub fn y(&self) -> usize {
        self.0
    }
    pub fn x(&self) -> usize {
        self.1
    }

    pub fn checked_moved(&self, dimensions: &Dimensions, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::Up => {
                if self.0 == 0 {
                    return None;
                }
                Some(Self(self.0 - 1, self.1))
            }
            Direction::Right => {
                if self.1 == dimensions.1 - 1 {
                    return None;
                }
                Some(Self(self.0, self.1 + 1))
            }
            Direction::Down => {
                if self.0 == dimensions.0 - 1 {
                    return None;
                }
                Some(Self(self.0 + 1, self.1))
            }
            Direction::Left => {
                if self.1 == 0 {
                    return None;
                }
                Some(Self(self.0, self.1 - 1))
            }
        }
    }
}

impl From<Position> for (usize, usize) {
    fn from(value: Position) -> Self {
        (value.y(), value.x())
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position(value.0, value.1)
    }
}
