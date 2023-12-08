use aoc2023::*;

fn main() {
	part1(|input| {
		let mut sum = 0;
		for line in input.lines() {
			let mut filtered = line.chars().filter(|v| v.is_ascii_digit());
			let first = filtered.next().unwrap();
			let last = filtered.next_back().unwrap_or(first);
			let concat: u32 = format!("{first}{last}").parse().unwrap();
			sum += concat;
		}
		sum
	});
	part2(|input| {
		let mut sum = 0;
		const DIGITS: &[(&str, u32)] = &[
			("0", 0),
			("1", 1),
			("2", 2),
			("3", 3),
			("4", 4),
			("5", 5),
			("6", 6),
			("7", 7),
			("8", 8),
			("9", 9),
			("zero", 0),
			("one", 1),
			("two", 2),
			("three", 3),
			("four", 4),
			("five", 5),
			("six", 6),
			("seven", 7),
			("eight", 8),
			("nine", 9),
		];
		for line in input.lines() {
			let text = line
				.char_indices()
				.filter_map(|(idx, _)| {
					let start = &line[idx..];
					for (text, num) in DIGITS {
						if start.starts_with(text) {
							return Some(*num);
						}
					}
					None
				})
				.collect::<Vec<_>>();
			let concat = text.first().unwrap() * 10 + text.last().unwrap();
			sum += concat;
		}
		sum
	});
}
