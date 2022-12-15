import tailwindcss from "prettier-plugin-tailwindcss";

module.exports = {
	printWidth: 100,
	tabWidth: 4,
	useTabs: true,
	endOfLine: "lf",
	arrowParens: "avoid",
	trailingComma: "none",
	plugins: [tailwindcss()]
};
