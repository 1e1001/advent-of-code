use ahash::AHashMap;
use aoc2023::*;

fn main() {
	fn find_near(
		numbers: &mut AHashMap<(isize, isize), Option<(String, bool)>>,
		(mut x, y): (isize, isize),
	) -> Option<&mut (String, bool)> {
		loop {
			match numbers.get(&(x, y)) {
				None => return None,
				Some(None) => x += 1,
				Some(Some(_)) => return numbers.get_mut(&(x, y)).unwrap().as_mut(),
			}
		}
	}
	part1(|input| {
		let mut symbols = AHashMap::new();
		let mut numbers = AHashMap::<_, Option<(String, bool)>>::new();
		for (y, line) in input.lines().enumerate() {
			for (x, col) in line.chars().enumerate() {
				let (x, y) = (x as isize, y as isize);
				match col {
					ch @ '0'..='9' => {
						if let Some(num) = numbers.get_mut(&(x - 1, y)) {
							let mut v = num.take().unwrap();
							v.0.push(ch);
							numbers.insert((x, y), Some(v));
						} else {
							numbers.insert((x, y), Some((format!("{ch}"), false)));
						}
					}
					'.' => {}
					ch => {
						symbols.insert((x, y), ch);
					}
				}
			}
		}
		let mut sum = 0;
		for ((x, y), _) in &symbols {
			let near = [
				(x - 1, *y),
				(x - 1, y - 1),
				(*x, y - 1),
				(x + 1, y - 1),
				(x + 1, *y),
				(x + 1, y + 1),
				(*x, y + 1),
				(x - 1, y + 1),
			];
			for (x, y) in near {
				if let Some((num, used)) = find_near(&mut numbers, (x, y)) {
					if !*used {
						*used = true;
						let parse: usize = num.parse().unwrap();
						sum += parse;
					}
				}
			}
		}
		sum
	});
	part2(|input| {
		let mut symbols = AHashMap::new();
		let mut numbers = AHashMap::<_, Option<(String, bool)>>::new();
		for (y, line) in input.lines().enumerate() {
			for (x, col) in line.chars().enumerate() {
				let (x, y) = (x as isize, y as isize);
				match col {
					ch @ '0'..='9' => {
						if let Some(num) = numbers.get_mut(&(x - 1, y)) {
							let mut v = num.take().unwrap();
							v.0.push(ch);
							numbers.insert((x, y), Some(v));
						} else {
							numbers.insert((x, y), Some((format!("{ch}"), false)));
						}
					}
					'*' => {
						symbols.insert((x, y), '*');
					}
					_ => {}
				}
			}
		}
		let mut sum = 0;
		for ((x, y), _) in &symbols {
			for (_, v) in &mut numbers {
				if let Some(v) = v {
					v.1 = false;
				}
			}
			let near = [
				(x - 1, *y),
				(x - 1, y - 1),
				(*x, y - 1),
				(x + 1, y - 1),
				(x + 1, *y),
				(x + 1, y + 1),
				(*x, y + 1),
				(x - 1, y + 1),
			]
			.into_iter()
			.filter_map(|(x, y)| {
				if let Some((num, used)) = find_near(&mut numbers, (x, y)) {
					if !*used {
						*used = true;
						Some(num.parse().unwrap())
					} else {
						None
					}
				} else {
					None
				}
			})
			.collect::<Vec<usize>>();
			if near.len() == 2 {
				sum += near[0] * near[1];
			}
		}
		sum
	});
}
