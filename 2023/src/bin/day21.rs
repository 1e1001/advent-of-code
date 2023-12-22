use ahash::{AHashMap, AHashSet};
use aoc2023::*;

fn main() {
	part1(|input| {
		let mut start = (0, 0);
		let grid = input
			.lines()
			.enumerate()
			.map(|(y, v)| {
				v.chars()
					.enumerate()
					.map(|(x, v)| {
						if v == 'S' {
							start = (x as isize, y as isize);
						}
						v != '#'
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let mut paths = AHashSet::new();
		paths.insert(start);
		for _ in 0..64 {
			let mut new_paths = AHashSet::new();
			for (x, y) in paths {
				for near in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
					if near.0 >= 0
						&& near.1 >= 0 && *grid
						.get(near.1 as usize)
						.and_then(|v| v.get(near.0 as usize))
						.unwrap_or(&false)
					{
						new_paths.insert(near);
					}
				}
			}
			paths = new_paths;
		}
		paths.len()
	});
	part2(|input| {
		// solved externally
		// https://www.desmos.com/calculator/atj2tlfy58
		let mut start = (0, 0);
		let grid = input
			.lines()
			.enumerate()
			.map(|(y, v)| {
				v.chars()
					.enumerate()
					.map(|(x, v)| {
						if v == 'S' {
							start = (x as isize, y as isize);
						}
						v != '#'
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let mut paths = AHashMap::new();
		paths.insert(
			start,
			[(0isize, 0isize)].into_iter().collect::<AHashSet<_>>(),
		);
		let (w, h) = (grid[0].len() as isize, grid.len() as isize);
		for i in 0..5000
		/* 26501365 */
		{
			println!("{i},{}", paths.values().map(|v| v.len()).sum::<usize>());
			let mut new_paths = AHashMap::new();
			for ((x, y), paths) in paths {
				for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
					let iax = if x < 0 {
						-1
					} else if x >= w {
						1
					} else {
						0
					};
					let iay = if y < 0 {
						-1
					} else if y >= h {
						1
					} else {
						0
					};
					let in_rect = (x - w * iax, y - h * iay);
					let in_area = (iax, iay);
					if grid[in_rect.1 as usize][in_rect.0 as usize] {
						let entry: &mut AHashSet<(isize, isize)> =
							new_paths.entry(in_rect).or_default();
						entry.extend(paths.iter().map(|v| (v.0 + in_area.0, v.1 + in_area.1)));
					}
				}
			}
			paths = new_paths;
		}
		paths.values().map(|v| v.len()).sum::<usize>()
	});
}
