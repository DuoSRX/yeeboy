// Import the Console class from the WebAssembly module
import { Console } from "yeeboy";

const pc = document.getElementById("console-pc");

const canvas = document.getElementById("console-canvas");
canvas.width = 160;
canvas.height = 144;

const ctx = canvas.getContext('2d');

document.addEventListener("keydown", event => {
  if (gameboy.key_down(event.key)) {
    event.preventDefault();
    return false;
  }
});

document.addEventListener("keyup", event => {
  if (gameboy.key_up(event.key)) {
    event.preventDefault();
    return false;
  }
});

const gameboy = Console.new();

const renderLoop = () => {
  while (!gameboy.new_frame()) {
    gameboy.step();
  }

  gameboy.end_frame();
  pc.innerHTML = gameboy.regs().toString(16);
  
  const frameData = gameboy.get_frame_data();
  const imageData = new ImageData(new Uint8ClampedArray(frameData), 160, 144);

  ctx.putImageData(imageData, 0, 0);
  requestAnimationFrame(renderLoop);
}

renderLoop();
