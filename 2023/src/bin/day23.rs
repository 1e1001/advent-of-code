use std::collections::VecDeque;

use ahash::{AHashMap, AHashSet};
use aoc2023::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
	Air,
	Wall,
	Up,
	Down,
	Left,
	Right,
}

fn main() {
	part1(|input| {
		let grid = input
			.lines()
			.map(|v| {
				v.chars()
					.map(|v| match v {
						'.' => Tile::Air,
						'#' => Tile::Wall,
						'^' => Tile::Up,
						'v' => Tile::Down,
						'<' => Tile::Left,
						'>' => Tile::Right,
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let mut finished = AHashSet::new();
		let mut queue = VecDeque::new();
		queue.push_back((1, 0, 0, AHashSet::new()));
		while let Some((x, y, d, mut found)) = queue.pop_front() {
			println!("{}", queue.len());
			let tile = grid[y][x];
			if tile == Tile::Wall || found.contains(&(x, y)) {
				continue;
			}
			let next = if y == 0 {
				vec![(x, y + 1)]
			} else if y == grid.len() - 1 {
				finished.insert(d);
				vec![]
			} else {
				match tile {
					Tile::Air => vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)],
					Tile::Wall => unreachable!(),
					Tile::Up => vec![(x, y - 1)],
					Tile::Down => vec![(x, y + 1)],
					Tile::Left => vec![(x - 1, y)],
					Tile::Right => vec![(x + 1, y)],
				}
			};
			found.insert((x, y));
			for next in next {
				queue.push_back((next.0, next.1, d + 1, found.clone()));
			}
		}
		finished.into_iter().max()
	});
	part2(|input| {
		let grid = input
			.lines()
			.map(|v| {
				v.chars()
					.map(|v| match v {
						'.' => Tile::Air,
						'#' => Tile::Wall,
						'^' => Tile::Up,
						'v' => Tile::Down,
						'<' => Tile::Left,
						'>' => Tile::Right,
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let end_y = grid.len() - 1;
		let mut near = AHashMap::new();
		near.insert((1, 0), vec![(1, 1, 1)]);
		for x in 1..grid[0].len() - 1 {
			for y in 1..end_y {
				if grid[y][x] != Tile::Wall {
					let mut out = Vec::new();
					for (xx, yy) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
						if grid[yy][xx] != Tile::Wall {
							out.push((xx, yy, 1));
						}
					}
					near.insert((x, y), out);
				}
			}
		}
		near.insert(
			(grid[0].len() - 2, end_y),
			vec![(grid[0].len() - 2, end_y - 1, 1)],
		);
		let keys = near.keys().copied().collect::<Vec<_>>();
		for pos in keys {
			let next = near.get(&pos).unwrap().clone();
			if next.len() == 2 {
				near.remove(&pos);
				let new_d = next[0].2 + next[1].2;
				for (a, b) in [(0, 1), (1, 0)] {
					for v in near.get_mut(&(next[a].0, next[a].1)).unwrap() {
						if v.0 == pos.0 && v.1 == pos.1 {
							v.0 = next[b].0;
							v.1 = next[b].1;
							v.2 = new_d;
						}
					}
				}
			}
		}
		println!("graph what {{");
		for ((x, y), c) in near.iter() {
			for (xx, yy, _) in c {
				println!("x{x}y{y} -- x{xx}y{yy}");
			}
		}
		println!("}}");
		let mut best = 0;
		let mut queue = VecDeque::new();
		queue.push_back((1, 0, 0, AHashSet::new()));
		let mut i = 0;
		while let Some((x, y, d, mut found)) = queue.pop_back() {
			i += 1;
			if i % 100000 == 0 {
				println!("{i} {}", queue.len());
			}
			if y == end_y {
				if d > best {
					println!("NEW BEST! {d}");
					best = d;
				}
				continue;
			} else {
				found.insert((x, y));
				for (nex, ney, ned) in near.get(&(x, y)).unwrap() {
					if !found.contains(&(*nex, *ney)) {
						queue.push_back((*nex, *ney, d + *ned, found.clone()));
					}
				}
			}
		}
		best
	});
}
