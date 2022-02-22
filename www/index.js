import { GameBoard, Direction } from './node_modules/wasm-2048';
import { memory } from './node_modules/wasm-2048/wasm_2048_bg.wasm';

const movementKeys = [
    {
        keys: ['ArrowUp',     'KeyW', 'KeyK'],
        dir: Direction.Up,
    }, {
        keys: ['ArrowDown',   'KeyS', 'KeyJ'],
        dir: Direction.Down,
    }, {
        keys: ['ArrowLeft',   'KeyA', 'KeyH'],
        dir : Direction.Left,
    }, {
        keys:  ['ArrowRight',  'KeyD', 'KeyL'],
        dir: Direction.Right
    }
];
const margin = 128;
const fontName = 'Open Sans';
const canvas = document.querySelector('#c2048');
let isGameWin = false;
let gameWinAnimationStart;
let prevTimestamp;

let boardSize, gameBoard, canvasSize, ctx, gap, fontSize, squareSize, borderRadius;

const setVariables = () => {
    canvasSize = Math.min(window.innerHeight, window.innerWidth) - margin;
    ctx = canvas.getContext('2d');
    gap = canvasSize / 64;
    fontSize = (canvasSize / boardSize - gap) / 2.5;
    squareSize = (canvasSize - gap * (boardSize + 1)) / boardSize;
    borderRadius = canvasSize / 200;
    
    canvas.height = canvasSize;
    canvas.width = canvasSize;
    ctx.textBaseline = 'middle';
    ctx.textAlign = 'center';
    window.requestAnimationFrame(render);
}

const init = (size) => {
    boardSize = size;
    gameBoard = new GameBoard(boardSize);
    setVariables();
    gameBoard.generate();
    window.requestAnimationFrame(render);
}

const move = (key) => {
    let dir = movementKeys.filter(e => e.keys.includes(key.code)).map(e => e.dir)[0];
    gameBoard.move_cells(dir);
    window.requestAnimationFrame(render);
    if (!isGameWin && gameBoard.is_game_win()) window.requestAnimationFrame(gameWin);
}

const gameWin = (timestamp) => {
    if (!gameWinAnimationStart) gameWinAnimationStart = timestamp;
    console.log(timestamp);
    const elapsed = timestamp - gameWinAnimationStart;

    isGameWin = true;
    ctx.fillStyle = '#ddc01103';
    ctx.fillRect(0, 0, canvasSize, canvasSize);
    
    ctx.fillStyle = '#fff';
    ctx.fillText('YOU WIN!', canvasSize / 2, canvasSize / 2);
    
    if (elapsed < 1000) window.requestAnimationFrame(gameWin);
}

const render = () => {
    ctx.clearRect(0, 0, canvasSize, canvasSize);
    ctx.restore();
    const cells = new Uint32Array(memory.buffer, gameBoard.cells, boardSize * boardSize);
    renderCells(cells);
    ctx.save();
}

const renderCells = (cells) => {
    for (let i = 0; i < boardSize; i++) {
        for (let j = 0; j < boardSize; j++) {
            const pow = cells[i * boardSize + j];
            const cellColor = color(pow);
            const [x, y] = [j, i].map(e => e * squareSize + (e + 1) * gap);

            ctx.strokeStyle = cellColor;
            ctx.fillStyle = cellColor;

            roundRect(ctx, x, y, squareSize, squareSize, borderRadius);
            if (pow === 0) continue;
            ctx.fillStyle = pow <= 2 ? '#776e65' : '#f9f6f2';
            const fontS = fontSize / (pow < 7 ? 1 : pow < 10 ? 1.1 : 1.2);
            ctx.font = `bold ${fontS}px ${fontName}`;
            ctx.fillText(Math.pow(2, pow), x + squareSize / 2, y + squareSize / 2);
        }
    }
}

const color = (pow) => {
    switch (pow) {
        case 0: return '#cdc1b4';
        case 1: return '#eee4da';
        case 2: return '#eee1c9';
        case 3: return '#f3b27a';
        case 4: return '#f69664';
        case 5: return '#f77c5f';
        case 6: return '#f75f3b';
        case 7: return '#edd073';
        case 8: return '#edcc62';
        case 9: return '#edc950';
        case 10: return '#edc53f';
        default: return '#eec22e';
    }
}

const roundRect = (ctx, x, y, width, height, radius) => {
    if (typeof radius === 'undefined') radius = 4;
    
    ctx.beginPath();
    ctx.moveTo(x + radius, y);
    
    ctx.lineTo(x + width - radius, y);
    ctx.quadraticCurveTo(x + width, y, x + width, y + radius);
    
    ctx.lineTo(x + width, y + height - radius);
    ctx.quadraticCurveTo(x + width, y + height, x + width - radius, y + height);

    ctx.lineTo(x + radius, y + height);
    ctx.quadraticCurveTo(x, y + height, x, y + height - radius);
    
    ctx.lineTo(x, y + radius);
    ctx.quadraticCurveTo(x, y, x + radius, y);
    
    ctx.closePath();
    ctx.fill();
    ctx.stroke();
}

const swipe = {
    x: null,
    y: null,
};

const handleTouchStart = (e) => {
    const firstTouch = e.touches[0];                                      
    [swipe.x, swipe.y] = [firstTouch.clientX, firstTouch.clientY];
};                                                
                                                                         
const handleTouchMove = (e) => {
    if (!swipe.x || !swipe.y) return;
    
    const delta = {
        x: swipe.x - e.touches[0].clientX,
        y: swipe.y - e.touches[0].clientY,
    };
    
    gameBoard.move_cells(getDir(delta));
    
    render();
    
    [swipe.x, swipe.y] = [null, null];
};

const getDir = (delta) => {
    if (Math.abs(delta.x) > Math.abs(delta.y)) {
        return delta.x > 0 ? Direction.Left : Direction.Right;
    } else {
        return delta.y > 0 ? Direction.Up : Direction.Down;
    }
}

window.addEventListener('keydown', move);
window.addEventListener('resize', setVariables);

document.addEventListener('touchstart', handleTouchStart, false);        
document.addEventListener('touchmove', handleTouchMove, false);

init(4);