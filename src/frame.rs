use crate::{NUM_COLS, NUM_ROWS};


pub type Frame = [[char; NUM_ROWS]; NUM_COLS];

// pub fn new_frame() -> Frame {
//     let mut cols = Vec::with_capacity(NUM_COLS);

//     for _ in 0..NUM_COLS {
//         let mut col = Vec::with_capacity(NUM_ROWS);
//         for _ in 0..NUM_ROWS {
//             col.push(" ");
//         }
//         cols.push(col);
//     }
//     cols
// }
pub fn new_frame() -> Frame {
	[[' '; NUM_ROWS]; NUM_COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
