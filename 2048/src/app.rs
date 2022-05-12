use std::path::{Path, PathBuf};

use board::Board;
use number_renderer::NumberRenderer;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::*;
use settings::Settings;

pub struct App<'a> {
    board: Board<'a>,
}
