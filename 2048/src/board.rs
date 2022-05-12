use number_renderer::NumberRenderer;
use opengl_graphics::GlGraphics;
use piston_window::*;
use rand::random;
use settings::Settings;
use std::collections::HashSet;
use tile::{Tile, TileState};

fn rgb2rgba(c: [f32; 3]) -> [f32; 4] {
    [c[0], c[1], c[2], 1.0]
}

pub struct Board<'a> {
    tiles: Vec<Tile<'a>>,
}
