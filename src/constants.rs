pub const BLOCK_SIZE: f64 = 10.0;
pub const COLS: usize = 60;
pub const ROWS: usize = 60;

#[derive(Copy, Clone)]
pub enum DIRECTION {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
