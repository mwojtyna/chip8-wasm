import { defineConfig } from "vite";

process.env.BROWSER = "firefox";
export default defineConfig({
	server: {
		open: true
	}
});
