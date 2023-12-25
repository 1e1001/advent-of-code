use std::fmt::Debug;
use std::time::Instant;

fn part_common<T: Debug, F: FnOnce(&str) -> T>(f: F, part2: bool) {
	let mut iter = std::env::args();
	iter.next();
	let day = iter.next().unwrap();
	if iter
		.next()
		.map(|v| v != if part2 { "1" } else { "2" })
		.unwrap()
	{
		let text = &*std::fs::read_to_string(format!("inputs/day{day}.txt")).unwrap();
		let start = Instant::now();
		let ans = f(text);
		let len = start.elapsed();
		eprintln!(
			"PART {} (in {:?}): {:?}",
			if part2 { "2" } else { "1" },
			len,
			ans
		);
	}
}

pub fn part1<T: Debug, F: FnOnce(&str) -> T>(f: F) {
	part_common(f, false);
}

pub fn part2<T: Debug, F: FnOnce(&str) -> T>(f: F) {
	part_common(f, true);
}
