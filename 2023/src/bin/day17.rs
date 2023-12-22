use ahash::AHashMap;
use aoc2023::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Path {
	x: isize,
	y: isize,
	run: u8,
	dir: Dir,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
	N,
	E,
	S,
	W,
}
impl Dir {
	fn offset(self) -> (isize, isize) {
		match self {
			Dir::N => (0, -1),
			Dir::E => (1, 0),
			Dir::S => (0, 1),
			Dir::W => (-1, 0),
		}
	}
	fn left(self) -> Self {
		match self {
			Dir::N => Dir::W,
			Dir::E => Dir::N,
			Dir::S => Dir::E,
			Dir::W => Dir::S,
		}
	}
	fn right(self) -> Self {
		match self {
			Dir::N => Dir::E,
			Dir::E => Dir::S,
			Dir::S => Dir::W,
			Dir::W => Dir::N,
		}
	}
}

fn main() {
	part1(|input| {
		let grid = input
			.lines()
			.map(|v| {
				v.chars()
					.map(|v| format!("{v}").parse().unwrap())
					.collect::<Vec<usize>>()
			})
			.collect::<Vec<_>>();
		let mut paths = AHashMap::new();
		paths.insert(
			Path {
				x: 0,
				y: 0,
				run: 1,
				dir: Dir::E,
			},
			0,
		);
		paths.insert(
			Path {
				x: 0,
				y: 0,
				run: 1,
				dir: Dir::S,
			},
			0,
		);
		let mut real_min = usize::MAX;
		let mut i = 0;
		while !paths.is_empty() {
			let mut new_paths = AHashMap::new();
			for (path, heat) in paths {
				for (dir, run) in [
					(path.dir.left(), 1),
					(path.dir.right(), 1),
					(path.dir, path.run + 1),
				] {
					if run > 3 || heat > real_min {
						continue;
					}
					let offset = dir.offset();
					let new_pos = (path.x + offset.0, path.y + offset.1);
					if new_pos.0 < 0
						|| new_pos.1 < 0 || new_pos.1 as usize >= grid.len()
						|| new_pos.0 as usize >= grid[0].len()
					{
						continue;
					}
					let new_heat = new_paths
						.entry(Path {
							x: new_pos.0,
							y: new_pos.1,
							run,
							dir,
						})
						.or_insert(usize::MAX);
					*new_heat =
						(*new_heat).min(heat + grid[new_pos.1 as usize][new_pos.0 as usize]);
				}
			}
			paths = new_paths;
			real_min = paths
				.iter()
				.filter_map(|(k, v)| {
					(k.x as usize == grid[0].len() - 1 && k.y as usize == grid.len() - 1)
						.then_some(v)
				})
				.min()
				.copied()
				.unwrap_or(usize::MAX)
				.min(real_min);
			println!("{i} {real_min} {}", paths.len());
			i += 1;
		}
		real_min
	});
	part2(|input| {
		let grid = input
			.lines()
			.map(|v| {
				v.chars()
					.map(|v| format!("{v}").parse().unwrap())
					.collect::<Vec<usize>>()
			})
			.collect::<Vec<_>>();
		let mut paths = AHashMap::new();
		paths.insert(
			Path {
				x: 0,
				y: 0,
				run: 1,
				dir: Dir::E,
			},
			0,
		);
		paths.insert(
			Path {
				x: 0,
				y: 0,
				run: 1,
				dir: Dir::S,
			},
			0,
		);
		let mut real_min = usize::MAX;
		let mut i = 0;
		while !paths.is_empty() {
			let mut new_paths = AHashMap::new();
			for (path, heat) in paths {
				let vals = [
					(path.dir, path.run + 1),
					(path.dir.left(), 1),
					(path.dir.right(), 1),
				];
				for (dir, run) in if path.run < 4 { &vals[..1] } else { &vals } {
					if *run > 10 || heat > real_min {
						continue;
					}
					let offset = dir.offset();
					let new_pos = (path.x + offset.0, path.y + offset.1);
					if new_pos.0 < 0
						|| new_pos.1 < 0 || new_pos.1 as usize >= grid.len()
						|| new_pos.0 as usize >= grid[0].len()
					{
						continue;
					}
					let new_heat = new_paths
						.entry(Path {
							x: new_pos.0,
							y: new_pos.1,
							run: *run,
							dir: *dir,
						})
						.or_insert(usize::MAX);
					*new_heat =
						(*new_heat).min(heat + grid[new_pos.1 as usize][new_pos.0 as usize]);
				}
			}
			paths = new_paths;
			real_min = paths
				.iter()
				.filter_map(|(k, v)| {
					(k.x as usize == grid[0].len() - 1
						&& k.y as usize == grid.len() - 1
						&& k.run >= 4)
						.then_some(v)
				})
				.min()
				.copied()
				.unwrap_or(usize::MAX)
				.min(real_min);
			println!("{i} {real_min} {}", paths.len());
			i += 1;
		}
		real_min
	});
}
