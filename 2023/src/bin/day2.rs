use aoc2023::*;

fn main() {
	part1(|input| {
		let mut sum = 0;
		for i in input.lines() {
			let idx = i[5..].find(':').unwrap();
			let id: u32 = i[5..idx + 5].parse().unwrap();
			let split = i[idx + 7..].split("; ");
			let mut valid = true;
			'first: for s in split {
				let split = s.split(", ");
				for s in split {
					let (num, col) = s.split_once(' ').unwrap();
					let num: u32 = num.parse().unwrap();
					match (num, col) {
						(13.., "red") | (14.., "green") | (15.., "blue") => {
							valid = false;
							break 'first;
						}
						_ => {}
					}
				}
			}
			if valid {
				sum += id;
			}
		}
		sum
	});
	part2(|input| {
		let mut sum = 0;
		for i in input.lines() {
			let idx = i[5..].find(':').unwrap();
			let split = i[idx + 7..].split("; ");
			let mut power_r = 0;
			let mut power_g = 0;
			let mut power_b = 0;
			for s in split {
				let split = s.split(", ");
				for s in split {
					let (num, col) = s.split_once(' ').unwrap();
					let num: u32 = num.parse().unwrap();
					match (num, col) {
						(n, "red") => {
							power_r = power_r.max(n);
						}
						(n, "green") => {
							power_g = power_g.max(n);
						}
						(n, "blue") => {
							power_b = power_b.max(n);
						}
						_ => unreachable!(),
					}
				}
			}
			sum += power_r * power_g * power_b;
		}
		sum
	});
}
