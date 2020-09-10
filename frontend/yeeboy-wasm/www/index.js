import { memory } from "yeeboy/yeeboy_wasm_bg";
import { Console } from "yeeboy";

const pc = document.getElementById("console-pc");

const canvas = document.getElementById("console-canvas");
canvas.width = 160;
canvas.height = 144;

const ctx = canvas.getContext('2d');

const gameboy = Console.new();

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

const frame = new Uint8Array(memory.buffer, gameboy.frame(), 160 * 144 * 4);

const renderLoop = () => {
  while (!gameboy.new_frame()) {
    gameboy.step();
  }

  gameboy.end_frame();
  pc.innerHTML = gameboy.regs().toString(16);
  const imageData = new ImageData(new Uint8ClampedArray(frame), 160, 144);
  ctx.putImageData(imageData, 0, 0);
  requestAnimationFrame(renderLoop);
}

renderLoop();
