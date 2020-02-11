use crate::store::Game;
use super::fill_rect::fill_rect;
use crate::store::State;
use stdweb::web::{CanvasRenderingContext2d, FillRule};
use std::f64::consts::PI;

struct Ball {
  color: String,
  x: f64,
  y: f64
}

// impl Ball {
//   fn new(color: String) -> Ball {
//     Ball {
//       color: color,
//       x: 0.0,
//       y: 0.0
//     }
//   }

//   fn set_cell_position(&mut self, column: u8, row: u8, width: i32) {
//     self.x = column * 
//   }
// }


trait DrawGameCtx {
  fn draw_background(&self, width: i32, height: i32);
  fn draw_ball(&self, ball: Ball, width: f64);
  fn draw_board(&self, game: Game, balls: Vec<Ball>);
}

impl DrawGameCtx for CanvasRenderingContext2d {
  fn draw_background(&self, width: i32, height: i32) {
    self.set_fill_style_color("#606368");
    fill_rect(self, 0, 0, width, height);
  }

  fn draw_ball(&self, ball: Ball, width: f64) {
    self.begin_path();
    self.arc(ball.x, ball.y, width / 2.0, 0.0, 2.0 * PI, false);
    self.set_fill_style_color(&ball.color);
    self.fill(FillRule::NonZero);
  }

  fn draw_board(&self, game: Game, balls: Vec<Ball>) {
    let Game {
      board_x,
      board_y,
      board_width,
      line_width,
      cell_width,
      ..
    } = game;
    self.set_fill_style_color("#3c4043");
    fill_rect(self, board_x, board_y, board_width, board_width);
    self.set_fill_style_color("#afb2b7");
    for i in 1..9 {
      fill_rect(self, board_x - line_width / 2 + (cell_width * i), board_y, line_width, board_width);
    }
    for i in 1..9 {
      fill_rect(self, board_x, board_y - line_width / 2 + (cell_width * i), board_width, line_width);
    }
    for ball in balls.into_iter() {
      self.draw_ball(ball, cell_width as f64);
    }
  }
}

pub fn draw_game(state: State) {
  let State {
    canvas_height,
    canvas_width,
    game,
    ..
  } = state;
  let canvas = state.canvas.unwrap();
  let ctx = canvas.ctx;
  ctx.draw_background(canvas_width, canvas_height);
  let balls = vec![];
  ctx.draw_board(game, balls);
}