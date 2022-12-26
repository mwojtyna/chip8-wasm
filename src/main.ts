import "./style.css";
import * as wasm from "chip8-emulator";
import { Emulator } from "chip8-emulator";

// Set up the emulator
const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 10;

const canvas = document.querySelector("canvas")!;
const ctx = canvas.getContext("2d")!;
canvas.style.width = WIDTH * SCALE + "px";
canvas.style.height = HEIGHT * SCALE + "px";

const compatibility = document.getElementById("compatibility")! as HTMLSelectElement;
compatibility.onchange = () => {
	location.reload();
};

wasm.init();
document.onkeydown = e => wasm.on_key_down(e.code);
document.onkeyup = () => wasm.on_key_up();

const emulator = Emulator.init(compatibility.selectedIndex);
const response = await fetch("roms/games/space-invaders.ch8");
const rom = await response.arrayBuffer();
emulator.load_rom(new Uint8Array(rom));

setInterval(draw, 1);

function draw() {
	const gfx = emulator.cycle();

	for (let row = 0; row < HEIGHT; row++) {
		for (let col = 0; col < WIDTH; col++) {
			ctx.fillStyle = gfx[row * WIDTH + col] == 0 ? "#000" : "#fff";
			ctx.fillRect(col, row, 1, 1);
		}
	}
}
