use std::time::Duration;

use crate::frame::{Drawable, Frame};
use crate::invader::Invaders;
use crate::shot::Shot;
use crate::{NUM_COLS, NUM_ROWS};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1, // Last playable row
            shots: Vec::new(),
        }
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) {
        let mut hit_something;

        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                hit_something = invaders.kill_invader_at(shot.x, shot.y);
                if hit_something {
                    shot.explode();
                }
            }
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < NUM_ROWS - 1 {
            self.y += 1;
        }
    }

    pub fn shoot(&mut self) {
        if self.shots.len() < 3 {
            self.shots.push(Shot::new(self.x, self.y - 1));
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        // retain is basically like filter in place
        self.shots.retain(|shot| !shot.dead());
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";

        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
