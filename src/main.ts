import "./style.css";

const canvas = document.querySelector("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d");

if (ctx) {
	for (let x = 0; x < canvas.width; x++) {
		for (let y = 0; y < canvas.height; y++) {
			const r = Math.floor(Math.random() * 255);
			const g = Math.floor(Math.random() * 255);
			const b = Math.floor(Math.random() * 255);
			ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
			ctx.fillRect(x, y, 1, 1);
		}
	}
} else {
	console.error("Could not get canvas context");
}
