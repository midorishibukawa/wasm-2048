import { GameBoard } from 'wasm-2048';
import { memory } from 'wasm-2048/wasm_2048_bg';

const boardSize = 4;
const gameBoard = new GameBoard(boardSize);

let cells = new Uint32Array(memory.buffer, gameBoard.cells, boardSize * boardSize);
console.log(cells);