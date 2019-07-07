import { Game, Cell, GameStatus } from 'tic-tac-toe';
import { memory } from "tic-tac-toe/tic_tac_toe_bg";

const CELL_SIZE = 50; // px
const CELL_PADDING = 5;
const GRID_COLOR = "#CCCCCC";

// Construct the universe, and get its width and height.
const game = Game.new();
const width = game.width();
const height = game.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("tic-tac-toe-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    drawGrid();
    drawCells();
    checkGameStatus()
    requestAnimationFrame(renderLoop);
}

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCross = (row, column) => {
  const cellMargin = 2;
  ctx.beginPath();
  ctx.moveTo(column * CELL_SIZE + CELL_PADDING, row * CELL_SIZE + CELL_PADDING);
  ctx.lineTo((column + 1) * CELL_SIZE - CELL_PADDING, (row + 1) * CELL_SIZE - CELL_PADDING);
  ctx.moveTo(column * CELL_SIZE + CELL_PADDING, (row + 1) * CELL_SIZE - CELL_PADDING);
  ctx.lineTo((column + 1) * CELL_SIZE - CELL_PADDING, row * CELL_SIZE + CELL_PADDING); 
  ctx.stroke();
}

const drawNought = (row, column) => {
  const noughtX = column * CELL_SIZE + CELL_SIZE / 2;
  const noughtY = row * CELL_SIZE + CELL_SIZE / 2;
  ctx.beginPath();
  ctx.arc(noughtX, noughtY, CELL_SIZE / 2 - CELL_PADDING, 0, 360);
  ctx.stroke();
}
  
const drawCells = () => {
  const cellsPtr = game.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      if(cells[idx] === Cell.Cross) {
        drawCross(row, col);
      }

      if(cells[idx] === Cell.Nought) {
        drawNought(row, col);
      }
    }

  };
}

const checkGameStatus = () => {
  const resultField = document.getElementById('result');
  // const status = new Uint8Array(memory.buffer, game.status(), 3);
  // console.log(status);
  if (game.status() == GameStatus.NoughtsWin) {
    resultField.innerHTML = "Noughts won";
  }

  if(game.status() == GameStatus.CrossesWin) {
    resultField.innerHTML = "Crosses won";
  }
}


canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  game.make_move(row, col);

  drawGrid();
  drawCells();
  checkGameStatus();
});

drawGrid();
drawCells();