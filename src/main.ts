import "./style.css";
import * as wasm from "chip8-emulator";
import { Emulator } from "chip8-emulator";
import "./fasterInterval.js";

// Set up the emulator
const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 10;

const canvas = document.querySelector("canvas")!;
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
const response = await fetch("roms/games/cavern.ch8");
const rom = await response.arrayBuffer();
emulator.load_rom(new Uint8Array(rom));

setInterval(cycle, 3);
draw();

function cycle() {
	emulator.cycle();
}
function draw() {
	emulator.draw();
	requestAnimationFrame(draw);
}
