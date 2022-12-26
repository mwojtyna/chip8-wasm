// For stopping the modified 'setInterval'
window._breakInterval = {};

// Replace 'setInterval'
window._setInterval = window.setInterval;
window.setInterval = (func, time = 0.001, ...args) => {
	// If time >= 10ms, use default 'setInterval'
	if (time >= 10) return window._setInterval(func, time, ...args);

	// To avoid zero or negative timings
	const minTime = 0.001;
	if (time <= 0) time = minTime;

	const callsPer10ms = 10 / time;
	const intervalCode = window._setInterval(() => {
		// Calls function 'callsPer10ms' times for every 10ms
		for (let i = 0; i < callsPer10ms; i++) {
			// Stops for loop when 'clearInterval' is called
			if (window._breakInterval[intervalCode]) {
				delete window._breakInterval[intervalCode];
				break;
			}
			func(...args);
		}
	}, 10);
	window._breakInterval[intervalCode] = false;
	return intervalCode;
};

// Replace 'clearInterval'
window._clearInterval = window.clearInterval;
window.clearInterval = intervalCode => {
	// Default 'clearInterval' behaviour
	if (window._breakInterval[intervalCode] === undefined)
		return window._clearInterval(intervalCode);

	window._clearInterval(intervalCode);
	window._breakInterval[intervalCode] = true;
};
