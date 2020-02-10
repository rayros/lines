use crate::store::state::View;
use crate::canvas::Canvas;

#[derive(Clone)]
pub enum Action {
  None,
  FontLoaded,
  NewCanvas { canvas: Canvas, width: i32, height: i32 },
  Draw,
  WindowResize,
  CanvasResize { width: i32, height: i32 },
  Click { x: i32, y: i32 },
  ChangeView { view: View }
  // Move { counter: u8 }
}

impl PartialEq for Action  {
  fn eq(&self, other: &Self) -> bool {
      self == other
  }
}