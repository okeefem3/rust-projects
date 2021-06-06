use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>; // Vector of vectors of borrowed static string slices

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);

    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);

        for _ in 0..NUM_ROWS {
            col.push(" "); // add a blank frame
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
