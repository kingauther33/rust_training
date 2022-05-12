use number_renderer::NumberRenderer;
use opengl_graphics::GlGraphics;
use piston_window::*;
use settings::Settings;

#[derive(Clone, PartialEq)]
pub enum TileState {
  TileStatic,
  /// (t, x, y, origin_x, origin_y)
  TileMoving(f64, f64, f64, i32, i32),
  /// (t, size)
  TileNew(f64, f64),
  /// (t, size)
  TileCombine(f64, f64)
}


#[derive(Clone)]