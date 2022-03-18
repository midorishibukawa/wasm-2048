//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasm_2048;
use wasm_bindgen_test::*;
use wasm_2048::game_board::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

// tests new game board generation
// every board of size i should have a cells vector length of i * i
// and the vector should be filled with zeroes
#[wasm_bindgen_test]
pub fn gameboard_generation() {
    for i in 3..10 {
        let gameboard = GameBoard::new(i);
        let cell_qty = i * i;
        let empty_vec = vec![0; cell_qty];

        assert_eq!(gameboard.size(), i);
        assert_eq!(gameboard.vec_cells(), empty_vec);
        assert_eq!(gameboard.empty_cells_qty(), cell_qty);
    }
}

// tests new cell generation
// every new cell should have a value of 1 or 2
// when the board is empty, two new cells should be generated
#[wasm_bindgen_test]
pub fn cell_generation() {
    for i in 3..10 {
        let mut gameboard = GameBoard::new(i);

        gameboard.generate();

        assert_eq!(gameboard.empty_cells_qty(), i * i - 2);
        assert!(
            gameboard.vec_cells()
                    .iter()
                    .filter(|cell| **cell == 1 || **cell == 2)
                    .collect::<Vec<&u8>>()
                    .len() == 2
        )
    }
}

// tests the merge prediction function
#[wasm_bindgen_test]
pub fn merge_prediction() {
    for i in 3..10 {
        let mut gameboard = GameBoard::new(i);
        gameboard.generate();
        let cells = gameboard.vec_cells().clone();
        let merge_prediction = gameboard.merge_prediction().clone();

        for dir in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            gameboard.set_cells(&cells);
            gameboard.move_cells(dir, false);
            assert_eq!(merge_prediction.get(&dir).unwrap(), &gameboard.vec_cells());
        }
    }
}

#[wasm_bindgen_test]
pub fn victory() {
    for i in 3..10 {
        let mut gameboard = GameBoard::new(i);
        
        assert!(!gameboard.is_game_win());

        let mut mock_cells = gameboard.vec_cells().clone();
        mock_cells[0] = 10;
        mock_cells[1] = 10;
        for dir in vec![Direction::Left, Direction::Right] {
            gameboard.set_cells(&mock_cells);
            gameboard.move_cells(dir, true);
            assert!(gameboard.is_game_win());
        }
    }
}

#[wasm_bindgen_test]
pub fn game_over() {
    for i in 3..10 {
        let mut gameboard = GameBoard::new(i);
        let n = i as u8;
        let mock_cells: Vec<u8> = (1u8..(n * n)).collect();
        for dir in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            gameboard.set_cells(&mock_cells);
            gameboard.move_cells(dir, true);
            assert!(gameboard.is_game_over());
        }
    }
}