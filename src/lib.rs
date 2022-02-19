extern crate web_sys;

mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use rand::Rng;

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
    cells: Vec<u32>
}

#[wasm_bindgen]
impl GameBoard {

    #[wasm_bindgen(constructor)]
    pub fn new(s: usize) -> GameBoard {
        let size = s;
        let mut cells = vec![0; size * size];
        let mut rng = rand::thread_rng();
        let (first, second) = (rng.gen_range(0..cells.len()), rng.gen_range(0..(cells.len() - 1)));

        cells[first] = 2;
        cells[if second >= first { second + 1 } else { second }] = 2;

        for i in 0..cells.len() {
            if cells[i] != 0 { log!("cell[{}][{}] = {}", i % size, i / size, cells[i]); }
        }

        GameBoard {
            size,
            cells,
        }
    }

    #[wasm_bindgen(method, getter)]
    pub fn cells(&self) -> *const u32 {
        self.cells.as_ptr()
    }

    pub fn move_cells(&mut self, dir: Direction) {
        let mut next: Vec<u32> = vec![0; self.size * self.size];

        let lines: Vec<Vec<u32>> = self.lines(if dir == Direction::Up || dir == Direction::Down { Axis::Vertical } else { Axis::Horizontal });
        
        for i in 0..lines.len() {
            for mut j in 0..lines[i].len() {
                next[self.move_index(i, j, dir)] = lines[i][j];
            }
        }
        
        self.cells = next;
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
                lines[if axis == Axis::Vertical { i % self.size } else { i / self.size }].push(self.cells[i]);
            }
        }
        
        return lines;
    }
}