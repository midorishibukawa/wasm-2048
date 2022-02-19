import { GameBoard } from 'wasm-2048';

const gameBoard = GameBoard.new(4);

console.log(gameBoard.get_cells());