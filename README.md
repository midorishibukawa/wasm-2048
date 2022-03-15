# wasm-2048

  <strong>An implementation of Gabriele Cirulli's 2048 on Rust-WASM and HTML Canvas</strong>

  This project was kickstarted using <code><a href="https://github.com/rustwasm/wasm-pack-template">wasm-pack-template</a></code>

  The wasm bundle has been published on npm, which can be found <a href="https://www.npmjs.com/package/wasm-2048">here</a>.

## Implementation

### Direction

  An enum that represents the player input movement direction.
  
  It has four possible values: <code>Up</code>, <code>Down</code>, <code>Left</code>, and <code>Right</code>.


### GameBoard

  This struct implements the game logic and gameboard structure.
  
#### Fields

  ##### <code>size: u8</code>

  The game board size.

  ##### <code>cells: Vec<u8></code>

  A vector that holds the game cells value as powers of two.
  
  It's initialized as an <code>u8</code> vector of size <code>size * size</code> filled with zeroes.
  
  ##### <code>rng: ThreadRng</code>
  
  A single RNG thread to be used whenever a new cell is generated.

  ##### <code>game_over: bool</code>
  
  A boolean that is used to check if the game is in a "game over" state.
  
  It is updated using the <code>merge_prediction</code> generated after every new cell generation.
  
  If true, the <code>move_cells</code> method aborts before executing any logic.
  
  ##### <code>merge_prediction: HashMap<Direction, Vec<u8>></code>
  
  A HashMap that holds the possible game states depending on the user input.
  
  Its keys are the four possible <code>Direction</code> enum values, while its values are of type <code>Vec<u8></code>, and hold the game state predictions (see <code>Gameboard cells</code>).
  
  Whenever the player moves, the GameBoard's <code>cells</code> value is updated to reflect the prediction according to the input <code>Direction</code>.
  
  Whenever a new cell is generated, the HashMap is updated to represent the new possible game states.
  
#### Public methods
  
  ##### <code>new(s: usize) -> GameBoard</code>

  The GameBoard constructor method.

  It receives an integer <code>s</code>, initializing a GameBoard with size <code>s</code> filled with zeroes.
  
  ##### <code>cells -> *const u8</code>
  
  A getter that returns a pointer to the GameBoard <code>cells</code> Vector.
  
  It can be converted to an Uint8Array in JavaScript.
  
  ##### <code>is_game_win -> bool</code>
  
  A getter that returns <code>true</code> if there's a cell in the GameBoard with a value of <code>11</code> (2^11 = 2048).
  
  ##### <code>is_game_over -> bool</code>
  
  A getter that returns the GameBoard's <code>game_over</code> value.
  
  ##### <code>generate</code>
  
  A method that generates a new cell in an empty space.
  
  It has a 63/64th chance of generating a new cell of power 1, and a 1/64th chance of generating a new cell of power 2.
  
  If the board is empty, it generates two new cells instead.
  
  ##### <code>move_cells(dir: Direction)</code>
  
  A method that receives a <code>Direction</code> and updates the GameBoard's state accordingly.
  
  It uses the GameBoard's <code>merge_prediction</code> value to update the GameBoard's <code>cells</code> value, calls the <code>generate</code> method to generate a new cell, generates a new <code>merge_prediction</code> HashMap, and then checks if the game is over.
