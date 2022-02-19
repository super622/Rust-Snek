use std::collections::VecDeque;
use ggez::{Context, GameResult, graphics};
use ggez::event::KeyCode;
use ggez::graphics::{Color, Drawable, DrawMode, DrawParam};
use ggez::mint::Point2;
use crate::direction::Direction;
use crate::hexagon::HexCoordinates;
use crate::HEX_SIDE;

#[derive(PartialEq)]
struct BodyPart{
    coordinates : HexCoordinates,
    dir_from : Direction,
    dir_to : Direction
}


impl BodyPart{
    pub fn new(coordinates : HexCoordinates, dir_from : Direction, dir_to : Direction) -> BodyPart{
        BodyPart{coordinates, dir_from, dir_to}
    }
}


pub struct Snake{
    tail : VecDeque<BodyPart>,
    pub falling: bool
}

impl Snake{
    pub fn new() -> Snake{
        let head = BodyPart::new(HexCoordinates::new(0, 0, 0), Direction::East, Direction::West);
        let mut tail = VecDeque::new();
        tail.push_back(head);
        tail.push_back(BodyPart::new(HexCoordinates::new(1 , -1, 0), Direction::East, Direction::West));
        tail.push_back(BodyPart::new(HexCoordinates::new(2 , -2, 0), Direction::East,Direction::West));
        Snake{tail, falling: false}
    }

    pub fn move_(&mut self){
        self.tail.pop_back();
            match self.tail.front(){
                None => return,
                Some(old_head) => {
                    let new_head = BodyPart::new(old_head.coordinates.move_in_dir(old_head.dir_to),old_head.dir_to.opposite_direction(),old_head.dir_to);
                    self.tail.push_front(new_head);
                }
            }
        if self.falling && !self.tail.is_empty(){
            self.tail.pop_front();
        }
        else{
            self.falling = false;
        }
    }

    pub fn rotate_head(&mut self, key : KeyCode){
        let new_dir = Direction::change_dir(self.tail.front().unwrap().dir_to, key);
        match new_dir {
            None => {}
            Some(dir) => {
                let old_head = self.tail.pop_front().unwrap();
                let new_head = BodyPart::new(old_head.coordinates,old_head.dir_from, dir);
                self.tail.push_front(new_head);
            }
        }
    }

    pub fn grow(&mut self, coord: HexCoordinates){
        let end_part = self.tail.back().unwrap();
        let new_part = BodyPart::new(coord, end_part.dir_from, end_part.dir_to);
        self.tail.push_back(new_part);
    }

    pub fn has_eaten_itself(&self) -> bool{
        let head = self.tail.front().unwrap();
        for part in &self.tail{
            if head != part && part.coordinates == head.coordinates{
                return true;
            }
        }
        false
    }

    pub fn get_head(&self) -> &HexCoordinates{
        let head = self.tail.front().unwrap();
        &head.coordinates
    }

    pub fn get_end(&self) -> &HexCoordinates{
        let end = self.tail.back().unwrap();
        &end.coordinates
    }

    pub fn check_collision(&self, coord: &HexCoordinates) -> bool{
        for body_part in &self.tail{
            if &body_part.coordinates == coord {
                return true;
            }
        }
        false
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>{
        //draw head
        if !self.tail.is_empty() {
            let mut start_range = 0;
            if !self.falling {
                start_range = 1;
                let head = self.tail.front().unwrap();
                let (head_center_x, head_center_y) = head.coordinates.get_eucl_center(HEX_SIDE);
                let (point_from_1, point_from_2) = head.dir_from.get_vertices(HEX_SIDE);
                let (point_to_1, point_to_2) = head.dir_to.get_vertices(HEX_SIDE);

                let head_mesh = graphics::Mesh::new_polygon(ctx, DrawMode::fill(), &[
                    point_from_1,
                    Point2::from([(point_to_1.x + point_to_2.x) / 2_f32, (point_to_1.y + point_to_2.y) / 2_f32]),
                    point_from_2
                ], Color::from_rgb(13, 133, 31))?;


                head_mesh.draw(ctx, DrawParam::default().dest(Point2::from([head_center_x, head_center_y])))?;
            }


            //draw tail
            for tail_part in self.tail.range(start_range..) {
                let (center_x, center_y) = tail_part.coordinates.get_eucl_center(HEX_SIDE);
                let (point_from_1, point_from_2) = tail_part.dir_from.get_vertices(HEX_SIDE);
                let (point_to_1, point_to_2) = tail_part.dir_to.get_vertices(HEX_SIDE);
                let tail_part_mesh = graphics::Mesh::new_polygon(ctx, DrawMode::fill(), &[
                    point_from_1,
                    point_from_2,
                    point_to_1,
                    point_to_2
                ], Color::from_rgb(13, 133, 31))?;
                tail_part_mesh.draw(ctx, DrawParam::default().dest(Point2::from([center_x, center_y])))?;
            }
        }

        Ok(())
    }



}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_move(){
        let snek1 = Snake::new();
        let mut snek2 = Snake::new();
        snek2.move_();

        assert_eq!(snek1.tail.len(), snek2.tail.len());
        assert!(!snek2.tail.contains(snek1.tail.back().unwrap()));

        let head2 = snek2.tail.front_mut().unwrap();
        assert_eq!(head2.coordinates, HexCoordinates::new(-1, 1, 0));
    }

    #[test]
    fn test_rotate_head(){
        let mut snek = Snake::new();

        snek.rotate_head(KeyCode::Right);
        assert_eq!(snek.tail.front().unwrap().dir_to, Direction::NorthWest);

        snek.rotate_head(KeyCode::A);
        assert_eq!(snek.tail.front().unwrap().dir_to, Direction::West);

        snek.rotate_head(KeyCode::Left);
        assert_eq!(snek.tail.front().unwrap().dir_to, Direction::SouthWest);

        snek.rotate_head(KeyCode::L);
        assert_eq!(snek.tail.front().unwrap().dir_to, Direction::SouthWest);

        snek.rotate_head(KeyCode::Right);
        assert_eq!(snek.tail.front().unwrap().dir_to, Direction::West);

    }

    #[test]
    fn test_grow(){
        let mut snek = Snake::new();
        snek.grow(HexCoordinates::new(3, -3, 0));
        assert_eq!(snek.tail.len(), 4);
        let snek_end = snek.tail.back().unwrap();
        assert_eq!(snek_end.coordinates, HexCoordinates::new(3, -3, 0));
        assert_eq!(snek_end.dir_to, Direction::West);
        assert_eq!(snek_end.dir_from, Direction::East);
    }

    #[test]
    fn test_has_eaten_itself(){
        let mut snek1 = Snake::new();
        assert!(!snek1.has_eaten_itself());

        snek1.tail.push_back(BodyPart::new(HexCoordinates::new(2,-1, -1 ), Direction::SouthWest, Direction::NorthWest));
        snek1.tail.push_back(BodyPart::new(HexCoordinates::new(1,0, -1 ), Direction::East, Direction::West));
        snek1.tail.push_back(BodyPart::new(HexCoordinates::new(0,0, 0 ), Direction::NorthWest, Direction::NorthEast));
        assert!(snek1.has_eaten_itself())



    }

    #[test]
    fn test_collision(){
        let snek = Snake::new();
        assert!(snek.check_collision(&HexCoordinates::new(1, -1, 0)));
        assert!(!snek.check_collision(&HexCoordinates::new(-1, 1, 0)))
    }
}