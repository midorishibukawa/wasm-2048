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
let gameOverAnimationStart;

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
}

const move = (key) => {
    if (key.code.startsWith('Arrow')) key.preventDefault();
    let dir = movementKeys.filter(e => e.keys.includes(key.code)).map(e => e.dir);
    if (dir.length == 0) return
    gameBoard.move_cells(dir[0], true);
    window.requestAnimationFrame(render);
    if (!isGameWin && gameBoard.is_game_win) window.requestAnimationFrame(gameWin);
    if (gameBoard.is_game_over) window.requestAnimationFrame(gameOver);
}

const gameOver = (timestamp) => {
    if (!gameOverAnimationStart) gameOverAnimationStart = timestamp;

    window.removeEventListener('keydown', move);
    const elapsed = timestamp - gameOverAnimationStart;

    isGameWin = true;
    ctx.fillStyle = '#00000008';
    ctx.fillRect(0, 0, canvasSize, canvasSize);
    
    ctx.fillStyle = '#ffffff80';
    ctx.fillText('GAME OVER!', canvasSize / 2, canvasSize / 2);
    
    if (elapsed < 1000) window.requestAnimationFrame(gameOver);
}

const gameWin = (timestamp) => {
    if (!gameWinAnimationStart) gameWinAnimationStart = timestamp;
    
    window.removeEventListener('keydown', move);
    const elapsed = timestamp - gameWinAnimationStart;

    isGameWin = true;
    ctx.fillStyle = '#ffffff08';
    ctx.fillRect(0, 0, canvasSize, canvasSize);
    
    ctx.fillStyle = '#00000010';
    ctx.fillText('YOU WIN!', canvasSize / 2, canvasSize / 2);
    ctx.fillStyle = '#00000010';
    
    if (elapsed < 1000) window.requestAnimationFrame(gameWin);
    else {
        window.addEventListener('keydown', move);
        ctx.fillStyle = '#00000080';
        ctx.font = `bold ${fontSize / 2}px ${fontName}`;
        ctx.fillText('press any key', canvasSize / 2, canvasSize / 2 + fontSize);
        ctx.fillText('to continue', canvasSize / 2, canvasSize / 2 + 1.5 * fontSize + gap / 4);
    }
}

const render = () => {
    ctx.clearRect(0, 0, canvasSize, canvasSize);
    ctx.restore();
    const cells = Array.from(new Uint8Array(memory.buffer, gameBoard.cells, boardSize * boardSize));
    document.querySelector('strong').innerText = cells.filter(cell => cell > 0).map(cell => Math.pow(2, cell)).reduce((cur, acc) => acc + cur);
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

document.querySelectorAll('.size-btn')
    .forEach((btn, i) => btn.addEventListener('click', () => init(i + 3)));

init(4);