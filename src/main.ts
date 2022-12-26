import "./style.css";
import * as wasm from "chip8-emulator";
import { Emulator } from "chip8-emulator";
import "./fasterInterval.js";

const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 10;

const canvas = document.querySelector("canvas")!;
canvas.style.width = WIDTH * SCALE + "px";
canvas.style.height = HEIGHT * SCALE + "px";

wasm.init();
document.onkeydown = e => wasm.on_key_down(e.code);
document.onkeyup = () => wasm.on_key_up();

const emulator = Emulator.init(1);
let intervalId: NodeJS.Timer;
let animationId: number;

const selectedRom = document.getElementById("rom")! as HTMLSelectElement;
selectedRom.onchange = async () => {
	await startGame();
};

document.getElementById("reload")!.onclick = async () => await loadRom();

await startGame();

async function startGame() {
	clearInterval(intervalId);
	cancelAnimationFrame(animationId);
	await loadRom();
	intervalId = setInterval(cycle, 2);
	animationId = draw();
}
async function loadRom() {
	if (selectedRom.value === "") return;

	const response = await fetch(`roms/${selectedRom.value}.ch8`);
	const data = await response.arrayBuffer();
	emulator.load_rom(new Uint8Array(data));
}

function cycle() {
	if (selectedRom.value === "") return;
	emulator.cycle();
}
function draw() {
	emulator.draw();
	return requestAnimationFrame(draw);
}
