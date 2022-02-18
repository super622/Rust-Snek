use ggez::graphics::mint::Point2;
use crate::direction::Direction;


#[derive(Debug, Hash, Eq, PartialEq)]
// r - NorthWest,  g - NorthEast, b - North
pub struct HexCoordinates {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl HexCoordinates {
    pub fn new(r: i32, g: i32, b: i32) -> HexCoordinates {
        HexCoordinates { r, g, b }
    }

    pub fn get_eucl_center(&self, side: f32) -> (f32, f32) {
        let center_y = -(self.b as f32) * 1.5 * side;
        let center_x = (self.b as f32 / 2.0 + self.r as f32) * 3_f32.sqrt() * side;
        (center_x, center_y)
    }

    pub fn move_in_dir(&self, dir: Direction) -> HexCoordinates {
        match dir {
            Direction::West => HexCoordinates::new(self.r - 1, self.g + 1, self.b),
            Direction::NorthWest => HexCoordinates::new(self.r - 1, self.g, self.b + 1),
            Direction::NorthEast => HexCoordinates::new(self.r, self.g - 1, self.b + 1),
            Direction::East => HexCoordinates::new(self.r + 1, self.g - 1, self.b),
            Direction::SouthEast => HexCoordinates::new(self.r + 1, self.g, self.b - 1),
            Direction::SouthWest => HexCoordinates::new(self.r, self.g + 1, self.b - 1)
        }
    }
}

impl Clone for HexCoordinates {
    fn clone(&self) -> HexCoordinates {
        HexCoordinates::new(self.r, self.g, self.b)
    }
}

pub struct Hexagon {
    pub center: Point2<f32>,
    side: f32,
    pub vertices: [Point2<f32>; 6],
}

impl Hexagon {
    pub fn new(coord: HexCoordinates, side: f32) -> Hexagon {
        let (center_x, center_y) = coord.get_eucl_center(side);

        let center = Point2::from([center_x, center_y]);
        let sin = 3_f32.sqrt() / 2_f32;
        let vertices = [
            Point2::from([center_x, center_y + side]),
            Point2::from([center_x + sin * side, center_y + side / 2_f32]),
            Point2::from([center_x + sin * side, center_y - side / 2_f32]),
            Point2::from([center_x, center_y - side]),
            Point2::from([center_x - sin * side, center_y - side / 2_f32]),
            Point2::from([center_x - sin * side, center_y + side / 2_f32])
        ];
        Hexagon { center, side, vertices }
    }
}
