use rand::Rng;
use crate::block::Block;
use crate::constants::DIRECTION;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

pub struct Maze {
    pub cols: usize,
    pub rows: usize,
    pub blocks: Vec<Block>,
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

impl Maze {
    pub fn new(cols: usize, rows: usize) -> Maze {
        let mut maze = Maze {
            cols,
            rows,
            blocks: vec![],
        };

        let mut i = 0;
        for y in 0..rows {
            for x in 0..cols {
                maze.blocks.push(Block::new(x, y, i));
                i += 1
            }
        }

        let mut rng = rand::thread_rng();
        let index_first_block = rng.gen_range(0..maze.blocks.len());

        maze.blocks[index_first_block].visited = true;

        maze
    }

    fn get_positions_next_block(&mut self, index: usize) -> Vec<usize> {
        let mut positions_next_block = vec![];
        let block = &self.blocks[index];

        if block.y > 0 {
            let index_top_block = (block.y - 1) * self.cols + block.x;
            if !self.blocks[index_top_block].visited {
                positions_next_block.push(index_top_block);
            }
        }

        if block.x < self.cols - 1 {
            let index_right_block = block.y * self.cols + block.x + 1;
            if !self.blocks[index_right_block].visited {
                positions_next_block.push(index_right_block);
            }
        }

        if block.y < self.rows - 1 {
            let index_bottom_block = (block.y + 1) * self.cols + block.x;
            if !self.blocks[index_bottom_block].visited {
                positions_next_block.push(index_bottom_block);
            }
        }

        if block.x > 0 {
            let index_left_block = block.y * self.cols + block.x - 1;
            if !self.blocks[index_left_block].visited {
                positions_next_block.push(index_left_block);
            }
        }

        positions_next_block
    }

    pub fn draw_maze(&self, context: &web_sys::CanvasRenderingContext2d) {
        for block in &self.blocks {
            block.draw(&context);
        }
    }

    pub fn possible_directions(&self, index: usize) -> Vec<usize> {
        let mut directions: Vec<usize> = vec![];
        let block = &self.blocks[index];

        if block.y > 0 && block.walls[DIRECTION::UP as usize] {
            let up = &self.blocks[index - self.cols];
            if up.index != block.index {
                directions.push(index - self.cols);
            }
        }

        if block.x < self.cols - 1 && block.walls[DIRECTION::RIGHT as usize] {
            let right = &self.blocks[index + 1];

            if right.index != block.index {
                directions.push(index + 1);
            }
        }

        if block.y < self.rows - 1 && block.walls[DIRECTION::DOWN as usize] {
            let down = &self.blocks[index + self.cols];
            if down.index != block.index {
                directions.push(index + self.cols);
            }
        }

        if block.x > 0 && block.walls[DIRECTION::LEFT as usize] {
            let left = &self.blocks[index - 1];
            if left.index != block.index {
                directions.push(index - 1);
            }
        }

        directions
    }

    pub fn get_visited_neighborhood(&self, index: usize) -> Option<usize> {
        let possible_directions = self.possible_directions(index);
        if possible_directions.len() == 0 {
            return None;
        }

        if possible_directions.len() == 1 {
            return Some(possible_directions[0]);
        }

        let random_item = possible_directions.choose(&mut rand::thread_rng()).unwrap();

        Some(*random_item)
    }

    pub fn get_random_possible_block(&mut self) -> Option<usize> {
        let mut keys: Vec<usize> = vec![];
        for (index, block) in self.blocks.iter().enumerate() {
            if self.possible_directions(index).len() > 0 {
                keys.push(block.index.into());
            }
        }

        if keys.len() == 0 {
            return None;
        }

        return Some(*keys.choose(&mut rand::thread_rng()).unwrap());
    }

    pub fn run(&mut self, context: &web_sys::CanvasRenderingContext2d) {
        let random_way = self.get_random_possible_block();
        match random_way {
            Some(index) => {
                let next_block = self.get_visited_neighborhood(index).unwrap();
                self.break_wall(index, next_block);
                self.join_indexes(index, next_block);
                self.draw_blocks(index, next_block, context);
            }
            None => {
            }
        }
    }

    pub fn break_wall(&mut self, current_index: usize, next_index: usize) {
        self.blocks[current_index].visited = true;
        self.blocks[next_index].visited = true;

        if current_index == next_index + self.cols {
            self.blocks[current_index].walls[DIRECTION::UP as usize] = false;
            self.blocks[next_index].walls[DIRECTION::DOWN as usize] = false;
            return;
        }

        if next_index > 1 && current_index == next_index - 1 {
            self.blocks[current_index].walls[DIRECTION::RIGHT as usize] = false;
            self.blocks[next_index].walls[DIRECTION::LEFT as usize] = false;
            return;
        }

        if next_index > self.cols && current_index == next_index - self.cols {
            self.blocks[current_index].walls[DIRECTION::DOWN as usize] = false;
            self.blocks[next_index].walls[DIRECTION::UP as usize] = false;
            return;
        }

        self.blocks[current_index].walls[DIRECTION::LEFT as usize] = false;
        self.blocks[next_index].walls[DIRECTION::RIGHT as usize] = false;
        return;
    }

    pub fn join_indexes(&mut self, current_index: usize, next_index: usize) {
        let new_index = self.blocks[current_index].index;
        let old_index = self.blocks[next_index].index;

        for block in &mut self.blocks {
            if block.index == old_index {
                block.index = new_index;
            }
        }
    }

    pub fn draw_blocks(&mut self, current_index: usize, next_index: usize, context: &web_sys::CanvasRenderingContext2d) {
        self.blocks[current_index].draw(context);
        self.blocks[next_index].draw(context);
    }
}
