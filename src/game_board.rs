use wasm_bindgen::prelude::*;
use rand::*;
use std::collections::{BTreeSet, HashMap};

macro_rules! log {
    ( $( $t:tt)* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    Vertical,
    Horizontal,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct GameBoard {
    size: usize,
    cells: Vec<u8>,
    rng: rngs::ThreadRng,
    game_over: bool,
    merge_prediction: HashMap<Direction, Vec<u8>>
}

#[wasm_bindgen]
impl GameBoard {

    #[wasm_bindgen(constructor)]
    pub fn new(s: usize) -> GameBoard {
        let size: usize = s;
        let cells: Vec<u8> = vec![0; size * size];
        let rng: rngs::ThreadRng = thread_rng();
        let _game_over: bool = false;
        let merge_prediction: HashMap<Direction, Vec<u8>> = HashMap::new();

        GameBoard {
            size,
            cells,
            rng,
            _game_over,
            merge_prediction,
        }
    }

    #[wasm_bindgen(method, getter)]
    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }

    #[wasm_bindgen(method, getter)]
    pub fn is_game_win(&self) -> bool {
        self.cells.contains(&11)
    }
    
    #[wasm_bindgen(method, getter)]
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn generate(&mut self) {
        let empty_idx: usize = self.rng.gen_range(0..self.empty_cells().len());
        let empty_vec: Vec<usize> = self.empty_cells().into_iter().collect();
        let idx = empty_vec[empty_idx];
        self.cells[idx] = if self.rng.gen_range(0..64) == 0 { 2 } else { 1 };
        
        if self.empty_cells().len() == self.cells.len() - 1 {
            self.generate();
        }

        self.predict_merge();
    }

    pub fn move_cells(&mut self, dir: Direction) {
        if self.game_over { return }

        let next: Vec<u8> = self.merge_prediction.get(&dir).unwrap().to_vec();
        
        if self.cells != next {
            self.cells = next;
            self.generate();
        }

        self.check_if_game_over();
    }

    fn predict_merge(&mut self) {
        let mut lines_axis: HashMap<Axis, Vec<Vec<u8>>> = HashMap::new();

        for axis in vec![Axis::Vertical, Axis::Horizontal] {
            lines_axis.insert(axis, self.lines(axis));
        }

        let mut merge_prediction: HashMap<Direction, Vec<u8>> = HashMap::new();

        for dir in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            merge_prediction.insert(dir, self.lines_to_vec(&self.merge(lines_axis.get(&GameBoard::dir_to_axis(dir)).unwrap(), dir), dir));
        }

        self.merge_prediction = merge_prediction;
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
    
    fn check_if_game_over(&mut self) -> bool {
        let merge: Vec<&Vec<u8>> = self.merge_prediction.values().collect();
        let game_over = false;
        for i in 0..merge[0].len() {
            if merge[0][i] != merge[1][i] ||
                merge[0][i] != merge[2][i] ||
                merge[0][i] != merge[3][i] {
                    return false;     
            }
        }
        self.game_over = true;
        true
    }

    
    fn lines_to_vec(&self, lines: &Vec<Vec<u8>>, dir: Direction) -> Vec<u8> {
        let mut next: Vec<u8> = vec![0; self.size * self.size];
        lines.into_iter()
        .enumerate()
        .for_each(|(i, line)| {
            match dir {
                Direction::Up | Direction::Left => line.into_iter().collect::<Vec<&u8>>(),
                Direction::Down | Direction::Right => line.into_iter().rev().collect::<Vec<&u8>>(),
            }.iter()
            .enumerate()
            .for_each(|(j, cell)|
            next[self.move_index(i, j, dir)] = **cell
            );
        });
        next
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
    
    fn merge(&self, lines: &Vec<Vec<u8>>, dir: Direction) -> Vec<Vec<u8>>{
        let mut merge: Vec<Vec<u8>> = vec![vec![]; self.size];
        
        let is_dir_rev: bool = GameBoard::is_dir_rev(dir);

        lines.into_iter()
            .enumerate()
            .for_each(|(i, l)| {
                let line_vec = match is_dir_rev {
                    true    => l.iter().rev().collect::<Vec<&u8>>(),
                    false   => l.iter().collect::<Vec<&u8>>(),
                };
                let mut line = line_vec.into_iter().peekable();
                let mut line_merge = Vec::new();

                while let Some(cell) = line.next() {
                    if Some(&cell) == line.peek() {
                        line_merge.push(*cell + 1);
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

    fn lines(&self, axis: Axis) -> Vec<Vec<u8>> {
        let mut lines: Vec<Vec<u8>> = vec![vec![]; self.size];
        
        self.cells
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, cell)|
                if cell > 0 {
                    lines[match axis {
                        Axis::Vertical      => i % self.size,
                        Axis::Horizontal    => i / self.size,
                    }].push(cell)}
            );
        
        lines
    }
}