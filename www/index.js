import { GameBoard, Direction } from 'wasm-2048';
import { memory } from 'wasm-2048/wasm_2048_bg';

const boardSize = 4;
const gameBoard = new GameBoard(boardSize);
const pre = document.querySelector('#canvas');

const init = () => {
    let cells = new Uint32Array(memory.buffer, gameBoard.cells, boardSize * boardSize);
    render(cells);
    console.log(cells);
}

const move = (key) => {
    if (key.code == 'ArrowUp') {
        gameBoard.move_cells(Direction.Up);
    } else if (key.code == 'ArrowDown') {
        gameBoard.move_cells(Direction.Down);
    } else if (key.code == 'ArrowLeft') {
        gameBoard.move_cells(Direction.Left);
    } else if (key.code == 'ArrowRight') {
        gameBoard.move_cells(Direction.Right);
    }
    let cells = new Uint32Array(memory.buffer, gameBoard.cells, boardSize * boardSize);
    render(cells);
    console.log(cells);
}

const render = (cells) => {
    pre.textContent = '';
    for (let i = 0; i < boardSize; i++) {
        pre.textContent += `${cells.slice(i * boardSize, (i + 1) * boardSize)}\n`;
    }
}

init();

window.addEventListener('keydown', move);