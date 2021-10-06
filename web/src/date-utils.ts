export function toISODate(d: Date) {
	const s = d.toISOString();
	return s.substring(0, s.indexOf('T'));
}

export function yearBlockStarts(date = new Date()) {
	const blocks = [];

	let year =
		date.getMonth() >= JULY ? date.getFullYear() : date.getFullYear() - 1;

	let d = new Date(year, JULY, 1);
	blocks.push(new Date(d));
	d.setDate(d.getDate() + 28);

	// Set next block start to Monday, first block is <= 4 weeks
	d.setDate(d.getDate() - d.getDay() + 1);
	blocks.push(new Date(d));

	for (let i = 0; i < 11; i++) {
		d.setDate(d.getDate() + 28);
		blocks.push(new Date(d));
	}

	return blocks;
}

const JULY = 6;

export type Datelike = Date | string;

export function getDate(date: Datelike): Date | undefined {
	if (date instanceof Date) return date;

	return date.includes(' ') ? parseDateTime(date) : parseDate(date);
}

export function parseDate(dateString: string): Date | undefined {
	try {
		const [year, month, day] = dateString.split('-').map(Number);
		return new Date(year, month - 1, day);
	} catch (err) {
		console.error(err);
	}

	return undefined;
}

export function parseDateTime(dateString: string): Date | undefined {
	try {
		const [date, time] = dateString.split(' ');
		const [year, month, day] = date.split('-').map(Number);
		const [hour, minute] = time.split(':').map(Number);

		return new Date(year, month - 1, day, hour, minute);
	} catch (err) {
		console.error(err);
	}

	return undefined;
}

export function parseBackendDate(dateString: string): Date | undefined {
	try {
		const [date, time] = dateString
			.substring(0, dateString.indexOf('+'))
			.split('T');
		const [year, month, day] = date.split('-').map(Number);
		const [hour, minute, second] = time.split(':').map(Number);
		return new Date(Date.UTC(year, month - 1, day, hour, minute, second));
	} catch (err) {
		console.error(err);
	}

	return undefined;
}

export function getDay(d: Datelike): Date {
	const day = getDate(d);
	day.setHours(0, 0, 0);

	return day;
}

export function getAcademicYear(d: Date): [Date, Date] {
	const start = new Date(d);
	const end = new Date(d);
	start.setHours(0, 0, 0);
	end.setHours(0, 0, 0);

	if (start.getMonth() < 6) {
		start.setFullYear(start.getFullYear() - 1);
	}
	start.setMonth(6, 1);

	if (end.getMonth() >= 6) {
		end.setFullYear(end.getFullYear() + 1);
	}
	end.setMonth(5, 30);

	return [start, end];
}
