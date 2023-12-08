use aoc2023::*;

fn quad(a: f64, b: f64, c: f64) -> (f64, f64) {
	let i = (b * b - 4.0 * a * c).sqrt();
	let l = 2.0 * a;
	((-b + i) / l, (-b - i) / l)
}

fn main() {
	part1(|input| {
		let mut input = input.lines();
		let times = input
			.next()
			.unwrap()
			.split_once(':')
			.unwrap()
			.1
			.split(' ')
			.filter(|v| !v.is_empty())
			.map(|v| v.parse().unwrap());
		let distances = input
			.next()
			.unwrap()
			.split_once(':')
			.unwrap()
			.1
			.split(' ')
			.filter(|v| !v.is_empty())
			.map(|v| v.parse().unwrap());
		let mut prod = 1;
		for (time, dist) in times.zip(distances) {
			let dist: f64 = dist;
			let (min, max) = quad(-1.0, time, -dist);
			let min = min.ceil() as usize;
			let max = max.floor() as usize;
			if max >= min {
				let diff = 1 + max - min;
				prod *= diff;
			}
		}
		prod
	});
	part2(|input| {
		let mut input = input.lines();
		let time = input
			.next()
			.unwrap()
			.split_once(':')
			.unwrap()
			.1
			.chars()
			.filter(|v| *v != ' ')
			.collect::<String>()
			.parse()
			.unwrap();
		let dist = input
			.next()
			.unwrap()
			.split_once(':')
			.unwrap()
			.1
			.chars()
			.filter(|v| *v != ' ')
			.collect::<String>()
			.parse()
			.unwrap();
		let mut prod = 1;
		let dist: f64 = dist;
		let (min, max) = quad(-1.0, time, -dist);
		let min = min.ceil() as usize;
		let max = max.floor() as usize;
		if max >= min {
			let diff = 1 + max - min;
			prod *= diff;
		}
		prod
	});
}
