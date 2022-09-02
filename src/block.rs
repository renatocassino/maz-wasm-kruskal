use crate::constants::{BLOCK_SIZE, DIRECTION};
use wasm_bindgen::JsValue;

pub struct Block {
    pub x: usize,
    pub y: usize,
    pub visited: bool,
    pub walls: [bool; 4], // up, right, bottom, left
    pub index: usize,
}

impl Block {
    pub fn new(x: usize, y: usize, index: usize) -> Block {
        Block {
            x,
            y,
            visited: false,
            walls: [true, true, true, true],
            index,
        }
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.begin_path();

        let top_left_x = self.x as f64 * BLOCK_SIZE + 10.0;
        let top_left_y = self.y as f64 * BLOCK_SIZE + 10.0;

        if self.visited {
            context.set_fill_style(&JsValue::from("red"));
            context.fill_rect(top_left_x, top_left_y, BLOCK_SIZE.into(), BLOCK_SIZE.into());

            context.set_fill_style(&JsValue::from("#000"));
        }

        if self.walls[DIRECTION::UP as usize] {
            context.move_to(top_left_x, top_left_y);
            context.line_to(top_left_x + BLOCK_SIZE, top_left_y);
        }

        if self.walls[DIRECTION::RIGHT as usize] {
            context.move_to(top_left_x + BLOCK_SIZE, top_left_y);
            context.line_to(top_left_x + BLOCK_SIZE, top_left_y + BLOCK_SIZE);
        }

        if self.walls[DIRECTION::DOWN as usize] {
            context.move_to(top_left_x, top_left_y + BLOCK_SIZE);
            context.line_to(top_left_x + BLOCK_SIZE, top_left_y + BLOCK_SIZE);
        }

        if self.walls[DIRECTION::LEFT as usize] {
            context.move_to(top_left_x, top_left_y);
            context.line_to(top_left_x, top_left_y + BLOCK_SIZE);
        }

        context.stroke();
        context.close_path();
    }
}
