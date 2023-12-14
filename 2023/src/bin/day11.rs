use ahash::AHashSet;
use aoc2023::*;

fn main() {
	part1(|input| {
		let mut grid = input
			.lines()
			.map(|v| {
				v.chars()
					.map(|v| match v {
						'.' => false,
						'#' => true,
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		println!("grid size: {}×{}", grid[0].len(), grid.len());
		let mut expand_line = Vec::new();
		for (i, line) in grid.iter().enumerate() {
			if !line.iter().any(|v| *v) {
				expand_line.push(i);
			}
		}
		for i in expand_line.into_iter().rev() {
			grid.insert(i, vec![false; grid[0].len()]);
		}
		let mut expand_column = Vec::new();
		for i in 0..grid[0].len() {
			if !grid.iter().any(|v| v[i]) {
				expand_column.push(i);
			}
		}
		for i in expand_column.into_iter().rev() {
			for line in &mut grid {
				line.insert(i, false);
			}
		}
		println!("grid size: {}×{}", grid[0].len(), grid.len());
		let mut tiles = AHashSet::new();
		for (y, line) in grid.into_iter().enumerate() {
			for (x, mode) in line.into_iter().enumerate() {
				if mode {
					tiles.insert((x, y));
				}
			}
		}
		let tiles = tiles.into_iter().collect::<Vec<_>>();
		let mut sum = 0;
		for (i, start) in tiles.iter().enumerate() {
			for end in tiles[i + 1..].iter() {
				sum += (start.0 as isize - end.0 as isize).abs()
					+ (start.1 as isize - end.1 as isize).abs();
			}
		}
		sum
	});
	part2(|input| {
		let grid = input
			.lines()
			.map(|v| {
				v.chars()
					.map(|v| match v {
						'.' => false,
						'#' => true,
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		println!("grid size: {}×{}", grid[0].len(), grid.len());
		let mut expand_line = Vec::new();
		for (i, line) in grid.iter().enumerate() {
			if !line.iter().any(|v| *v) {
				expand_line.push(i);
			}
		}
		let mut expand_column = Vec::new();
		for i in 0..grid[0].len() {
			if !grid.iter().any(|v| v[i]) {
				expand_column.push(i);
			}
		}
		println!(
			"grid size: {}×{}",
			grid[0].len() + expand_column.len() * 1000000,
			grid.len() + expand_line.len() * 1000000
		);
		let mut tiles = AHashSet::new();
		for (y, line) in grid.into_iter().enumerate() {
			for (x, mode) in line.into_iter().enumerate() {
				let xo = expand_column.iter().filter(|v| **v < x).count();
				let yo = expand_line.iter().filter(|v| **v < y).count();
				if mode {
					tiles.insert((x + xo * 999999, y + yo * 999999));
				}
			}
		}
		let tiles = tiles.into_iter().collect::<Vec<_>>();
		let mut sum = 0;
		for (i, start) in tiles.iter().enumerate() {
			for end in tiles[i + 1..].iter() {
				sum += (start.0 as isize - end.0 as isize).abs()
					+ (start.1 as isize - end.1 as isize).abs();
			}
		}
		sum
	});
}
