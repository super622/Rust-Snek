use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::{Context, ContextBuilder, event, GameError, GameResult, graphics, timer};
use ggez::conf::{Conf, FullscreenType, WindowMode};
use ggez::graphics::{Color, Drawable, DrawMode, Mesh, Rect, Text};
use indexmap::map::IndexMap;
use crate::tile::Tile;
use rand::Rng;
use crate::hexagon::HexCoordinates;
use crate::snake::Snake;
use ggez::graphics::mint::Point2;
use ggez::graphics::DrawParam;
use crate::direction::Direction;

mod hexagon;
mod tile;
mod snake;
mod direction;

const HEX_SIDE: f32 = 30.0;
const GRID_RADIUS: i32 = 9;
const ADD_WALLS_NUM: i32 = 3;
const ADD_WALLS_INTERVAL: i32 = 3;
const UPDATE_SPEED : u32 = 2;

pub struct MainState {
    board: IndexMap<HexCoordinates, Tile>,
    apple: HexCoordinates,
    prev_apple: Option<HexCoordinates>,
    snake: Snake,
    score: i32,
    end_game: bool,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut map = IndexMap::new();
        for radius in 0_i32..GRID_RADIUS + 1 {
            for r in -radius..radius + 1 {
                for b in -radius..radius + 1 {
                    for g in -radius..radius + 1 {
                        if (r.abs() + b.abs() + g.abs() == 2 * radius) && (r + b + g == 0) {
                            if r.abs() == GRID_RADIUS || b.abs() == GRID_RADIUS || g.abs() == GRID_RADIUS {
                                map.insert(HexCoordinates::new(r, b, g), Tile::new(r, g, b, true));
                            } else {
                                map.insert(HexCoordinates::new(r, b, g), Tile::new(r, g, b, false));
                            }
                        }
                    }
                }
            }
        }
        let snake = Snake::new();
        let mut game = MainState {
            apple: HexCoordinates::new(0, 0, 0),
            board: map,
            snake,
            score: 0,
            end_game: false,
            prev_apple: None,
        };
        game.apple = game.get_random_tile();
        Ok(game)
    }

    fn get_random_tile(&self) -> HexCoordinates {
        let board_size = self.board.len();
        let mut rand_index = rand::thread_rng().gen_range(0..board_size);
        let mut pair = self.board.get_index(rand_index).unwrap();
        while pair.1.is_hole()  {
            rand_index = rand::thread_rng().gen_range(0..board_size);
            pair = self.board.get_index(rand_index).unwrap();
        }
        return pair.0.clone();
    }

    fn check_if_falls(&mut self) {
        let head_coord = self.snake.get_head();
        let head_tile = self.board.get(head_coord).unwrap();
        if head_tile.is_hole() {
            self.end_game = true;
            self.snake.falling = true;
        }
    }

    fn check_if_eaten_apple(&mut self) {
        if self.snake.get_head() == &self.apple {
            self.prev_apple = Some(self.apple.clone());
            self.apple = self.get_random_tile();
            self.score = self.score + 1;
            if self.score % ADD_WALLS_INTERVAL == 0 {
                self.add_holes();
            }
        }
        let end_coord = self.snake.get_end();
        if let Some(prev_apple_coord) = &self.prev_apple {
            if end_coord == prev_apple_coord {
                self.snake.grow(end_coord.clone());
                self.prev_apple = None;
            }
        }
    }

    fn add_holes(&mut self) {
        let mut wall_coord = self.get_random_tile();
        let mut counter = 0;
        while counter < ADD_WALLS_NUM {
            let mut rand_index = rand::thread_rng().gen_range(0..6);
            let rand_dir = Direction::from_value(rand_index).unwrap();
            let rand_neighbour_coord = wall_coord.move_in_dir(rand_dir);
            if let Some(neighbour) = self.board.get_mut(&rand_neighbour_coord) {
                if !neighbour.is_hole() && rand_neighbour_coord != self.apple && !self.snake.check_collision(&rand_neighbour_coord) {
                    neighbour.set_as_hole();
                    wall_coord = rand_neighbour_coord;
                    counter = counter + 1;
                }
            }
        }
    }
}

impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {

        while timer::check_update_time(ctx, UPDATE_SPEED) {

            if self.end_game {
                if self.snake.falling{
                    self.snake.move_();
                }
                return Ok(());
            }

            self.snake.move_();
            if self.snake.has_eaten_itself() {
                self.end_game = true;
                return Ok(());
            }
            self.check_if_falls();
            self.check_if_eaten_apple();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        for (_, tile) in &self.board {
            tile.draw(ctx)?;
        }


        let (apple_x, apple_y) = self.apple.get_eucl_center(HEX_SIDE);
        let apple = Mesh::new_circle(ctx, DrawMode::fill(), Point2::from([apple_x, apple_y]), 10.0, 0.2, Color::RED)?;
        graphics::draw(ctx, &apple, (glam::Vec2::new(0.0, 0.0), ))?;


        self.snake.draw(ctx)?;


        let screen = ((GRID_RADIUS as f32) * 2.0 + 1.0) * 3_f32.sqrt() / 2_f32 * HEX_SIDE;
        let mut score_text = Text::new(self.score.to_string());
        score_text.set_font(graphics::Font::default(), graphics::PxScale { x: 50.0, y: 50.0 });
        score_text.draw(ctx, DrawParam::from((glam::Vec2::new(screen - 100.0, -screen + 100.0), )))?;

        if self.end_game {
            let mut end_text = Text::new("GAME OVER");

            end_text.set_font(graphics::Font::default(), graphics::PxScale { x: 100.0, y: 100.0 });
            end_text.draw(ctx, DrawParam::from((glam::Vec2::new(-250.0, -100.0), Color::RED, )))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self,
                      _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.snake.rotate_head(keycode);
    }
}

fn main() {
    let screen = ((GRID_RADIUS as f32) * 2.0 + 1.0) * 3_f32.sqrt() / 2_f32 * HEX_SIDE;
    let conf = Conf::new().window_mode(WindowMode {
        width: screen * 2.0,
        height: screen * 2.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        min_height: 0.0,
        max_width: 0.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        resize_on_scale_factor_change: false,
    });


    let (mut ctx, event_loop) = ContextBuilder::new("snake", "edi")
        .default_conf(conf.clone())
        .build().
        unwrap();

    graphics::set_screen_coordinates(&mut ctx, Rect::new(-screen, -screen, screen * 2.0, screen * 2.0));

    let main_state = MainState::new().unwrap();

    event::run(ctx, event_loop, main_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_tile() {
        let game = MainState::new().unwrap();
        let random_tile_coord = game.get_random_tile();
        let random_tile = game.board.get(&random_tile_coord).unwrap();
        assert!(!random_tile.is_hole())
    }

    #[test]
    fn test_add_walls(){
        let mut game = MainState::new().unwrap();
        game.add_holes();
        let mut counter = 0;
        for (coord, tile) in game.board{
            if tile.is_hole() &&  !(coord.r.abs() == GRID_RADIUS || coord.b.abs() == GRID_RADIUS || coord.g.abs() == GRID_RADIUS){
                counter = counter + 1;
            }
        }
        assert_eq!(counter, ADD_WALLS_NUM);
    }
}
