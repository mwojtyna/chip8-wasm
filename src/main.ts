import "./style.css";

const canvas = document.getElementById("emulator") as HTMLCanvasElement;
const ctx = canvas.getContext("2d");

if (ctx) {
	console.log("Canvas loaded");
	ctx.fillStyle = "green";
	ctx.fillRect(0, 0, canvas.width, canvas.height);
} else {
	console.error("Could not get canvas context");
}
