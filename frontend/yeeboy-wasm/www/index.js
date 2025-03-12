import { Console } from "yeeboy";
import { memory } from "../pkg/yeeboy_wasm_bg.wasm";

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

let lastFrameTime = 0;
const targetFrameRate = 60;
const frameInterval = 1000 / targetFrameRate; // ~16.67ms per frame at 60 FPS

const frame = new Uint8Array(memory.buffer, gameboy.frame(), 160 * 144 * 4);

const renderLoop = (currentTime) => {
  const elapsed = currentTime - lastFrameTime;

  if (elapsed >= frameInterval || lastFrameTime === 0) {
    lastFrameTime = currentTime - (elapsed % frameInterval);
 
    while (!gameboy.new_frame()) {
      gameboy.step();
    }

    gameboy.end_frame();
    pc.innerHTML = gameboy.regs().toString(16);

    const imageData = new ImageData(new Uint8ClampedArray(frame), 160, 144);
    ctx.putImageData(imageData, 0, 0);
  }

  requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
