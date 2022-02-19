mod utils;

use wasm_bindgen::prelude::*;
use rand::Rng;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[wasm_bindgen]
pub struct GameBoard {
    side: usize,
    cells: Vec<u32>
}

#[wasm_bindgen]
impl GameBoard {
    #[wasm_bindgen(constructor)]
    pub fn new(s: usize) -> GameBoard {
        let side = s;

        let mut cells = vec![0; side * side];

        let mut rng = rand::thread_rng();

        let (first, second) = (rng.gen_range(0..cells.len()), rng.gen_range(0..(cells.len() - 1)));
        cells[first] = 2;
        cells[if second >= first { second + 1 } else { second }] = 2;

        GameBoard {
            side,
            cells,
        }
    }

    #[wasm_bindgen(method, getter)]
    pub fn cells(&self) -> *const u32 {
        self.cells.as_ptr()
    }
}
