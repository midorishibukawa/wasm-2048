import { GameBoard, Direction } from 'wasm-2048';
import { memory } from 'wasm-2048/wasm_2048_bg';

const boardSize = 4;
const gameBoard = new GameBoard(boardSize);
const canvas = document.querySelector('#c2048');
const margin = 128;
const canvasSize = Math.min(window.innerHeight, window.innerWidth) - margin;
const ctx = canvas.getContext('2d');
const fontName = 'monospace';
const gap = canvasSize / 64;
const fontSize = (canvasSize / boardSize - gap) / 4;
const squareSize = (canvasSize - gap * (boardSize + 1)) / boardSize;

canvas.height = canvasSize;
canvas.width = canvasSize;
ctx.font = `${fontSize}px ${fontName}`;

const init = () => {
    gameBoard.generate();
    render();
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
    render();
}

const render = () => {
    const cells = new Uint32Array(memory.buffer, gameBoard.cells, boardSize * boardSize);
    
    for (let i = 0; i < boardSize; i++) {
        for (let j = 0; j < boardSize; j++) {
            ctx.strokeStyle = '#eee';
            ctx.fillStyle = '#eee';
            let x = j * squareSize + (j + 1) * gap;
            let y = i * squareSize + (i + 1) * gap;
            roundRect(ctx, x, y, squareSize, 4);
        }
    }

    for (let i = 0; i < boardSize; i++) {
        for (let j = 0; j < boardSize; j++) {
            let value = cells[i * boardSize + j];
            if (value == 0) continue;
            let x = j * squareSize + (j + 1) * gap;
            let y = i * squareSize + (i + 1) * gap;
            ctx.strokeStyle = '#ddd';
            ctx.fillStyle = '#ddd';
            roundRect(ctx, x, y, squareSize, 4);
            ctx.fillStyle = '#000';
            ctx.fillText(value, x + squareSize / 2 - fontSize / 3, y + squareSize / 2 + fontSize / 3);
        }
    }
}

const roundRect = (ctx, x, y, size, radius) => {
    if (typeof radius === 'undefined') radius = 4;
    
    ctx.beginPath();
    ctx.moveTo(x + radius, y);
    
    ctx.lineTo(x + size - radius, y);
    ctx.quadraticCurveTo(x + size, y, x + size, y + radius);
    ctx.lineTo(x + size, y + size - radius);
    ctx.quadraticCurveTo(x + size, y + size, x + size - radius, y + size);
    ctx.lineTo(x + radius, y + size);
    ctx.quadraticCurveTo(x, y + size, x, y + size - radius);
    ctx.lineTo(x, y + radius);
    ctx.quadraticCurveTo(x, y, x + radius, y);
    
    ctx.closePath();
    ctx.fill();
    ctx.stroke();
}

init();

window.addEventListener('keydown', move);