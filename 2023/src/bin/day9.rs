#![feature(iter_map_windows)]
use aoc2023::*;

fn extrapolate(num: &[isize]) -> isize {
	if num.iter().all(|v| *v == 0) {
		0
	} else {
		let sub = num
			.iter()
			.map_windows(|[x, y]| **y - **x)
			.collect::<Vec<_>>();
		num.last().unwrap() + extrapolate(&sub)
	}
}
fn extrapolate_r(num: &[isize]) -> isize {
	if num.iter().all(|v| *v == 0) {
		0
	} else {
		let sub = num
			.iter()
			.map_windows(|[x, y]| **y - **x)
			.collect::<Vec<_>>();
		num.first().unwrap() - extrapolate_r(&sub)
	}
}

fn main() {
	part1(|input| {
		input
			.lines()
			.map(|i| {
				let values = i
					.split_whitespace()
					.map(|v| v.parse().unwrap())
					.collect::<Vec<isize>>();
				extrapolate(&values)
			})
			.sum::<isize>()
	});
	part2(|input| {
		input
			.lines()
			.map(|i| {
				let values = i
					.split_whitespace()
					.map(|v| v.parse().unwrap())
					.collect::<Vec<isize>>();
				extrapolate_r(&values)
			})
			.sum::<isize>()
	});
}
