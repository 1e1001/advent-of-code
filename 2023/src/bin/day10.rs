use std::collections::VecDeque;

use ahash::AHashMap;
use aoc2023::*;

#[derive(Debug, Clone, Copy)]
enum Pipe {
	Ns,
	Ew,
	Ne,
	Nw,
	Se,
	Sw,
	None,
	Start,
}
impl Pipe {
	fn char(self) -> char {
		match self {
			Pipe::Ns => '|',
			Pipe::Ew => '-',
			Pipe::Ne => 'L',
			Pipe::Nw => 'J',
			Pipe::Se => 'F',
			Pipe::Sw => '7',
			Pipe::None => '.',
			Pipe::Start => 'S',
		}
	}
	fn ym(self, (x, y): (usize, usize), at: impl Fn((usize, usize)) -> Pipe) -> Option<bool> {
		let filter = self
			.near((x, y), at)
			.filter(|v| v.1 != y)
			.collect::<Vec<_>>();

		(filter.len() == 1).then(|| filter[0].1 == y + 1)
	}
	fn near_inner(self, (x, y): (usize, usize)) -> std::vec::IntoIter<(usize, usize)> {
		match self {
			Pipe::Ns => vec![(x, y.wrapping_sub(1)), (x, y + 1)],
			Pipe::Ew => vec![(x.wrapping_sub(1), y), (x + 1, y)],
			Pipe::Ne => vec![(x, y.wrapping_sub(1)), (x + 1, y)],
			Pipe::Nw => vec![(x, y.wrapping_sub(1)), (x.wrapping_sub(1), y)],
			Pipe::Se => vec![(x, y + 1), (x + 1, y)],
			Pipe::Sw => vec![(x, y + 1), (x.wrapping_sub(1), y)],
			Pipe::None => vec![],
			Pipe::Start => vec![
				(x, y.wrapping_sub(1)),
				(x, y + 1),
				(x.wrapping_sub(1), y),
				(x + 1, y),
			],
		}
		.into_iter()
	}
	fn near(
		self,
		(x, y): (usize, usize),
		at: impl Fn((usize, usize)) -> Pipe,
	) -> impl Iterator<Item = (usize, usize)> {
		self.near_inner((x, y))
			.filter(|v| at(*v).near_inner(*v).any(|v| v == (x, y)))
			.collect::<Vec<_>>()
			.into_iter()
	}
}

fn main() {
	part1(|input| {
		let mut start = None;
		let grid = input
			.lines()
			.enumerate()
			.map(|(y, v)| {
				v.chars()
					.enumerate()
					.map(|(x, v)| match v {
						'|' => Pipe::Ns,
						'-' => Pipe::Ew,
						'L' => Pipe::Ne,
						'J' => Pipe::Nw,
						'F' => Pipe::Se,
						'7' => Pipe::Sw,
						'.' => Pipe::None,
						'S' => {
							start = Some((x, y));
							Pipe::Start
						}
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let at = |(x, y): (usize, usize)| {
			grid.get(y)
				.and_then(|v| v.get(x))
				.copied()
				.unwrap_or(Pipe::None)
		};
		let mut map: AHashMap<_, usize> = AHashMap::new();
		let mut queue = VecDeque::new();
		queue.push_back((start.unwrap(), 0));
		while let Some((pos, dist)) = queue.pop_front() {
			let tile = at(pos);
			println!("{tile:?} {pos:?} {dist}");
			if !map.contains_key(&pos) {
				for next in tile.near(pos, &at) {
					queue.push_back((next, dist + 1));
				}
			}
			map.insert(pos, (*map.get(&pos).unwrap_or(&dist)).min(dist));
		}
		for (y, r) in grid.iter().enumerate() {
			for (x, p) in r.iter().enumerate() {
				print!("{}", p.char());
				if let Some(d) = map.get(&(x, y)) {
					print!("{d: >4}");
				} else {
					print!("    ");
				}
			}
			println!();
		}
		map.values().copied().max()
	});
	part2(|input| {
		let mut start = None;
		let grid = input
			.lines()
			.enumerate()
			.map(|(y, v)| {
				v.chars()
					.enumerate()
					.map(|(x, v)| match v {
						'|' => Pipe::Ns,
						'-' => Pipe::Ew,
						'L' => Pipe::Ne,
						'J' => Pipe::Nw,
						'F' => Pipe::Se,
						'7' => Pipe::Sw,
						'.' => Pipe::None,
						'S' => {
							start = Some((x, y));
							Pipe::Start
						}
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let at = |(x, y): (usize, usize)| {
			grid.get(y)
				.and_then(|v| v.get(x))
				.copied()
				.unwrap_or(Pipe::None)
		};
		let mut map: AHashMap<_, usize> = AHashMap::new();
		let mut queue = VecDeque::new();
		queue.push_back((start.unwrap(), 0));
		while let Some((pos, dist)) = queue.pop_front() {
			let tile = at(pos);
			if !map.contains_key(&pos) {
				for next in tile.near(pos, &at) {
					queue.push_back((next, dist + 1));
				}
			}
			map.insert(pos, (*map.get(&pos).unwrap_or(&dist)).min(dist));
		}
		let mut area = 0usize;
		for (y, r) in grid.iter().enumerate() {
			for (x, _) in r.iter().enumerate() {
				if map.contains_key(&(x, y)) {
					continue;
				}
				let mut toggle = false;
				let mut i = x;
				while i < r.len() {
					if map.contains_key(&(i, y)) {
						let start_i = i;
						let start = at((i, y));
						while at((i, y)).near((i, y), at).any(|v| v == (i + 1, y)) {
							i += 1;
						}
						let end = at((i, y));
						if start
							.ym((start_i, y), at)
							.zip(end.ym((i, y), at))
							.map(|(a, b)| a != b)
							.unwrap_or(true)
						{
							toggle = !toggle;
						}
					}
					i += 1;
				}
				if toggle {
					area += 1;
				}
			}
		}
		area
	});
}
