use std::{cmp::max, time::Duration, usize};

use rusty_time::prelude::Timer;

use crate::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_ROWS,
};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    pub move_timer: Timer,
    pub direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 1..NUM_COLS - 2 {
            for y in 1..(NUM_ROWS / 2) {
                if x % 2 == 0 && y % 2 == 0 {
                    army.push(Invader { x, y })
                }
            }
        }

        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        let max_y = self.army.iter().map(|i| i.y).max().unwrap_or(0);
        return max_y == NUM_ROWS - 1;
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self.army.iter().position(|i| i.x == x && i.y == y) {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.move_timer.update(delta);

        if self.move_timer.ready {
            self.move_timer.reset();

            let mut downwards = false;

            if self.direction < 0 {
                let min_x = self.army.iter().map(|i| i.x).min().unwrap_or(0);

                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|i| i.x).max().unwrap_or(0);

                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);

                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = (invader.x as i32 + self.direction) as usize;
                }
            }
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            let time_ratio =
                self.move_timer.time_left.as_secs_f32() / self.move_timer.duration.as_secs_f32();
            frame[invader.x][invader.y] = if time_ratio > 0.5 { "x" } else { "+" };
        }
    }
}
