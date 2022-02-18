use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Color};
use crate::hexagon::{HexCoordinates, Hexagon};

pub struct Tile {
    hex: Hexagon,
    is_wall: bool,
}

impl Tile {
    pub fn new(r: i32, b: i32, g: i32, is_wall: bool) -> Tile {
        let hex = Hexagon::new(HexCoordinates::new(r, g, b), crate::HEX_SIDE);

        Tile { hex, is_wall }
    }

    pub fn is_wall(&self) -> bool {
        return self.is_wall;
    }

    pub fn set_as_wall(&mut self) {
        self.is_wall = true;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let color = if self.is_wall { Color::BLACK } else { Color::from_rgb(199, 154, 18) };
        let hex_mesh_fill = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &self.hex.vertices,
            color,
        )?;

        let hex_mesh_stroke = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::stroke(2.0),
            &self.hex.vertices,
            Color::BLACK,
        )?;


        graphics::draw(ctx, &hex_mesh_stroke, (glam::Vec2::new(0.0, 0.0), ))?;
        graphics::draw(ctx, &hex_mesh_fill, (glam::Vec2::new(0.0, 0.0), ))
    }
}


