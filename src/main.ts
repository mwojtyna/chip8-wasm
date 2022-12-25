import "./style.css";
import * as emulator from "chip8-emulator";

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

document.onkeydown = e => emulator.on_key_down(e.code);
document.onkeyup = () => emulator.on_key_up();

emulator.start(compatibility.value);
