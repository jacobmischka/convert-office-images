export function camelCaseToWords(str) {
	let result = '';
	for (const char of str) {
		if (result === '') {
			result += char.toUpperCase();
		} else if (char === char.toUpperCase()) {
			result += ' ' + char.toLowerCase();
		} else {
			result += char;
		}
	}
	return result;
}

