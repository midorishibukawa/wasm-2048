extern crate web_sys;

mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use rand::*;
use std::collections::BTreeSet;

macro_rules! log {
    ( $( $t:tt)* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Vertical,
    Horizontal,
}

#[wasm_bindgen]
pub struct GameBoard {
    size: usize,
    cells: Vec<u32>,
    rng: rngs::ThreadRng,
}

#[wasm_bindgen]
impl GameBoard {

    #[wasm_bindgen(constructor)]
    pub fn new(s: usize) -> GameBoard {
        let size: usize = s;
        let mut cells: Vec<u32> = vec![0; size * size];
        let mut rng: rngs::ThreadRng = thread_rng();

        GameBoard {
            size,
            cells,
            rng,
        }
    }

    #[wasm_bindgen(method, getter)]
    pub fn cells(&self) -> *const u32 {
        self.cells.as_ptr()
    }

    fn empty_cells(&self) -> BTreeSet<usize> {
        let mut empty: BTreeSet<usize> = BTreeSet::new();

        for i in 0..self.cells.len() {
            if self.cells[i] == 0 { empty.insert(i); }
        }

        return empty;
    }

    pub fn move_cells(&mut self, dir: Direction) {
        let mut next: Vec<u32> = vec![0; self.size * self.size];

        let lines: Vec<Vec<u32>> = self.lines(
                if dir == Direction::Up || dir == Direction::Down {
                    Axis::Vertical
                } else {
                    Axis::Horizontal
                });
        
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                next[self.move_index(i, j, dir)] = lines[i][j];
            }
        }
        if self.cells != next {
            self.cells = next;
            self.generate();
        }
    }

    pub fn generate(&mut self) {
        let empty_idx: usize = self.rng.gen_range(0..self.empty_cells().len());
        let empty_vec: Vec<usize> = self.empty_cells().into_iter().collect();
        let idx = empty_vec[empty_idx];
        self.cells[idx] = if self.rng.gen_range(0..64) == 0 { 4 } else { 2 };
        self.empty_cells().remove(&idx);

        if self.empty_cells().len() + 1 == self.cells.len() {
            self.generate();
        }
    }
    
    fn move_index(&self, i: usize, j: usize, dir: Direction) -> usize {
        match dir {
            Direction::Up       => { self.index(i,                  j                   ) }
            Direction::Down     => { self.index(i,                  self.size - j - 1   ) }
            Direction::Left     => { self.index(j,                  i                   ) }
            Direction::Right    => { self.index(self.size - j - 1,  i                   ) }
        }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        return row + col * self.size;
    }

    fn lines(&self, axis: Axis) -> Vec<Vec<u32>> {
        let mut lines: Vec<Vec<u32>> = vec![vec![]; self.size];

        for i in 0..self.cells.len() {
            if self.cells[i] > 0 {
                lines[if axis == Axis::Vertical {
                    i % self.size
                } else {
                    i / self.size
                }].push(self.cells[i]);
            }
        }
        
        return lines;
    }
}