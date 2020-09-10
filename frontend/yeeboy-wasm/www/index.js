import { memory } from "yeeboy/yeeboy_wasm_bg";
import { Console } from "yeeboy";

// const app = new Vue({
//   el: '#app',
//   data: {
//     ctx: null,
//     gameboy: Console.new(),
//     counter: 0,
//   },
//   mounted() {
//     var canvas = document.getElementById('console-screen');
//     canvas.width = 160;
//     canvas.height = 144;
//     this.ctx = canvas.getContext('2d');
//     this.frame = new Uint8Array(memory.buffer, this.gameboy.frame(), 160 * 144 * 4);
//     // setInterval(function() {
//     //   this.counter += 1;
//     // }, 1000);
//     // this.gameboy.run();
//   },
//   // render() {
//     // gameboy.step();
//   // }
// });

const gameboy = Console.new();

const app = new Vue({
  el: '#app',
  methods: {
    pause() {
      if (this.gameboy.paused) {
        this.gameboy.paused = false;
        this.paused = false;
        renderLoop();
      } else {
        this.gameboy.paused = true;
        this.paused = true;
      }
    },
    pauseText() {
      return this.paused ? "Start" : "Pause";
    }
  },
  data: {
    gameboy: gameboy,
    paused: gameboy.paused,
  },
  mounted() {
    window.addEventListener("keydown", event => {
      if (gameboy.key_down(event.key)) {
        return event.preventDefault();
      }
    });
    window.addEventListener("keyup", event => {
      if (gameboy.key_up(event.key)) {
        return event.preventDefault();
      }
    });
  }
});

const canvas = document.getElementById("console-canvas");
const ctx = canvas.getContext('2d');
const frame = new Uint8Array(memory.buffer, gameboy.frame(), 160 * 144 * 4);

const renderLoop = () => {
  while (!gameboy.paused && !gameboy.new_frame()) {
    gameboy.step();
  }

  gameboy.end_frame();
  const imageData = new ImageData(new Uint8ClampedArray(frame), 160, 144);
  ctx.putImageData(imageData, 0, 0);

  if (!gameboy.paused) {
    requestAnimationFrame(renderLoop);
  }
}

renderLoop();
