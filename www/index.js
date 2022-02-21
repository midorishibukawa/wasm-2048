import { GameBoard, Direction } from './node_modules/wasm-2048';
import { memory } from './node_modules/wasm-2048/wasm_2048_bg.wasm';

const boardSize = 4;
const gameBoard = new GameBoard(boardSize);
const canvas = document.querySelector('#c2048');
const margin = 128;
const fontName = 'Open Sans';
const canvas = document.querySelector('#c2048');

let boardSize, gameBoard, canvasSize, ctx, gap, fontSize, squareSize, borderRadius;

const init = (size) => {
    boardSize = 4;
    gameBoard = new GameBoard(boardSize);
    canvasSize = Math.min(window.innerHeight, window.innerWidth) - margin;
    ctx = canvas.getContext('2d');
    gap = canvasSize / 64;
    fontSize = (canvasSize / boardSize - gap) / 2.5;
    squareSize = (canvasSize - gap * (boardSize + 1)) / boardSize;
    borderRadius = canvasSize / 200;

    console.log(`boardSize = ${boardSize}\ncanvasSize = ${canvasSize}\ngap = ${gap}\nfontSize = ${fontSize}\nsquareSize = ${squareSize}\nborderRadius = ${borderRadius}`);
    
    canvas.height = canvasSize;
    canvas.width = canvasSize;
    ctx.textBaseline = 'middle';
    ctx.textAlign = 'center';
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
    renderBoard();
    renderCells(cells);
}

const renderBoard = () => {
    for (let i = 0; i < boardSize; i++) {
        for (let j = 0; j < boardSize; j++) {
            ctx.strokeStyle = '#cdc1b4';
            ctx.fillStyle = '#cdc1b4';
            let x = j * squareSize + (j + 1) * gap;
            let y = i * squareSize + (i + 1) * gap;
            roundRect(ctx, x, y, squareSize, borderRadius);
        }
    }
}

const renderCells = (cells) => {
    for (let i = 0; i < boardSize; i++) {
        for (let j = 0; j < boardSize; j++) {
            let value = cells[i * boardSize + j];
            if (value == 0) continue;
            let cellColor = color(value);
            let x = j * squareSize + (j + 1) * gap;
            let y = i * squareSize + (i + 1) * gap;
            ctx.strokeStyle = cellColor;
            ctx.fillStyle = cellColor;
            roundRect(ctx, x, y, squareSize, borderRadius);
            ctx.fillStyle = value <= 4 ? '#776e65' : '#f9f6f2';
            const fontS = value < 100 ? fontSize : value < 1000 ? fontSize / 1.1 : fontSize / 1.2;
            ctx.font = `${fontS}px ${fontName}`;
            ctx.fillText(value, x + squareSize / 2, y + squareSize / 2);
        }
    }
}

const color = (value) => {
    switch (value) {
        case 2: return '#eee4da';
        case 4: return '#eee1c9';
        case 8: return '#f3b27a';
        case 16: return '#f69664';
        case 32: return '#f77c5f';
        case 64: return '#f75f3b';
        case 128: return '#edd073';
        case 256: return '#edcc62';
        case 512: return '#edc950';
        case 1024: return '#edc53f';
        default: return '#eec22e';
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