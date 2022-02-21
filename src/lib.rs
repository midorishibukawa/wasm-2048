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
        let cells: Vec<u32> = vec![0; size * size];
        let rng: rngs::ThreadRng = thread_rng();

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
        self.cells
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(i, cell)| match cell {
                0 => Some(i),
                _ => None,
            })
            .collect::<BTreeSet<usize>>()
    }

    pub fn move_cells(&mut self, dir: Direction) {
        let mut next: Vec<u32> = vec![0; self.size * self.size];
        
        let lines: Vec<Vec<u32>> = self.merge(self.lines(GameBoard::dir_to_axis(dir)), dir);

        lines.into_iter()
            .enumerate()
            .for_each(|(i, line)| {
                match dir {
                    Direction::Up | Direction::Left => line.iter().collect::<Vec<&u32>>(),
                    Direction::Down | Direction::Right => line.iter().rev().collect::<Vec<&u32>>(),
                }.iter().enumerate().for_each(|(j, cell)|
                next[self.move_index(i, j, dir)] = **cell
            );
                }
            );
            
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
        
        if self.empty_cells().len() + 1 == self.cells.len() {
            self.generate();
        }
    }
    
    fn move_index(&self, i: usize, j: usize, dir: Direction) -> usize {
        match dir {
            Direction::Up       => self.index(i,                    j),
            Direction::Down     => self.index(i,                    self.size - j - 1),
            Direction::Left     => self.index(j,                    i),
            Direction::Right    => self.index(self.size - j - 1,    i),
        }
    }
    
    fn index(&self, row: usize, col: usize) -> usize {
        row + col * self.size
    }
    
    fn merge(&self, lines: Vec<Vec<u32>>, dir: Direction) -> Vec<Vec<u32>>{
        let mut merge: Vec<Vec<u32>> = vec![vec![]; self.size];
        
        let is_dir_rev: bool = GameBoard::is_dir_rev(dir);

        lines.into_iter()
            .enumerate()
            .for_each(|(i, l)| {
                let line_vec = match is_dir_rev {
                    true    => l.iter().rev().collect::<Vec<&u32>>(),
                    false   => l.iter().collect::<Vec<&u32>>(),
                };
                let mut line = line_vec.into_iter().peekable();
                let mut line_merge = Vec::new();

                while let Some(cell) = line.next() {
                    if Some(&cell) == line.peek() {
                        line_merge.push(*cell * 2);
                        line.next();
                    } else {
                        line_merge.push(*cell);
                    }
                }

                merge[i] = match is_dir_rev {
                    true    => line_merge.into_iter().rev().collect(),
                    false   => line_merge,
                }
            }
        );
        merge
    }

    fn dir_to_axis(dir: Direction) -> Axis {
        match dir {
            Direction::Up   | Direction::Down   => Axis::Vertical,
            Direction::Left | Direction::Right  => Axis::Horizontal,
        }
    }

    fn is_dir_rev(dir: Direction) -> bool {
        match dir {
            Direction::Up   | Direction::Left    => false,
            Direction::Down | Direction::Right   => true,
        }
    }

    fn lines(&self, axis: Axis) -> Vec<Vec<u32>> {
        let mut lines: Vec<Vec<u32>> = vec![vec![]; self.size];
        
        self.cells
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, cell)|
                if cell > 0 {
                    lines[ match axis {
                        Axis::Vertical      => i % self.size,
                        Axis::Horizontal    => i / self.size,
                    }].push(cell)}
            );
        
        lines
    }
}