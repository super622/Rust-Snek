use ggez::event::KeyCode;
use ggez::mint::Point2;
use ggez::winit::event::VirtualKeyCode;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    West,
    NorthWest,
    NorthEast,
    East,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub fn value(&self) -> i32 {
        match self {
            Direction::West => 0,
            Direction::NorthWest => 1,
            Direction::NorthEast => 2,
            Direction::East => 3,
            Direction::SouthEast => 4,
            Direction::SouthWest => 5,
        }
    }

    pub fn from_value(val: i32) -> Option<Direction> {
        match val {
            0 => Some(Direction::West),
            1 => Some(Direction::NorthWest),
            2 => Some(Direction::NorthEast),
            3 => Some(Direction::East),
            4 => Some(Direction::SouthEast),
            5 => Some(Direction::SouthWest),
            _ => None
        }
    }

    pub fn change_dir(from_dir: Direction, with_key: KeyCode) -> Option<Direction> {
        let change_value = match with_key {
            VirtualKeyCode::A => -1,
            VirtualKeyCode::D => 1,
            VirtualKeyCode::Left => -1,
            VirtualKeyCode::Right => 1,
            _ => return None
        };
        let new_value = (from_dir.value() + change_value).rem_euclid(6);
        Direction::from_value(new_value)
    }

    pub fn get_vertices(&self, side: f32) -> (Point2<f32>, Point2<f32>) {
        let sin = 3_f32.sqrt() / 2_f32;
        let vertices = [
            Point2::from([-sin * side, side / 2_f32]),
            Point2::from([-sin * side, -side / 2_f32]),
            Point2::from([0.0, -side]),
            Point2::from([sin * side, -side / 2_f32]),
            Point2::from([sin * side, side / 2_f32]),
            Point2::from([0.0, side])
        ];
        let val = self.value() as usize;
        (vertices[val], vertices[(val + 1).rem_euclid(6)])
    }

    pub fn opposite_direction(&self) -> Direction {
        match self {
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_dir() {
        if let Some(new_dir) = Direction::change_dir(Direction::East, KeyCode::A) {
            assert_eq!(new_dir, Direction::NorthEast)
        }
        if let Some(new_dir) = Direction::change_dir(Direction::East, KeyCode::Right) {
            assert_eq!(new_dir, Direction::SouthEast)
        }

        if let Some(new_dir) = Direction::change_dir(Direction::West, KeyCode::Left) {
            assert_eq!(new_dir, Direction::SouthWest)
        }
    }
}