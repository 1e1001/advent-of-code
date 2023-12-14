use aoc2023::*;

fn main() {
	part1(|input| {
		let mut grids = Vec::new();
		let mut grid = Vec::new();
		for v in input.lines().chain([""]) {
			if v.is_empty() {
				grids.push(grid);
				grid = Vec::new();
				continue;
			}
			grid.push(v.chars().map(|v| v == '#').collect::<Vec<_>>());
		}
		let mut sum = 0;
		for grid in grids {
			let mirror_row = (1..grid.len()).find(|row| {
				grid[..*row]
					.iter()
					.rev()
					.zip(&grid[*row..])
					.all(|(a, b)| a == b)
			});
			let mirror_col = (1..grid[0].len()).find(|col| {
				grid.iter().all(|row| {
					row[..*col]
						.iter()
						.rev()
						.zip(&row[*col..])
						.all(|(a, b)| a == b)
				})
			});
			if mirror_col.is_none() && mirror_row.is_none() {
				panic!("no mirror");
			}
			sum += mirror_col.unwrap_or(0) + 100 * mirror_row.unwrap_or(0);
		}
		sum
	});
	part2(|input| {
		let mut grids = Vec::new();
		let mut grid = Vec::new();
		for v in input.lines().chain([""]) {
			if v.is_empty() {
				grids.push(grid);
				grid = Vec::new();
				continue;
			}
			grid.push(v.chars().map(|v| v == '#').collect::<Vec<_>>());
		}
		let mut sum = 0;
		for grid in grids {
			let mirror_row = (1..grid.len()).find(|row| {
				grid[..*row]
					.iter()
					.rev()
					.zip(&grid[*row..])
					.map(|(a, b)| {
						a.iter()
							.zip(b)
							.map(|(a, b)| (a != b) as usize)
							.sum::<usize>()
					})
					.sum::<usize>() == 1
			});
			let mirror_col = (1..grid[0].len()).find(|col| {
				grid.iter()
					.map(|row| {
						row[..*col]
							.iter()
							.rev()
							.zip(&row[*col..])
							.map(|(a, b)| (a != b) as usize)
							.sum::<usize>()
					})
					.sum::<usize>() == 1
			});
			if mirror_col.is_none() && mirror_row.is_none() {
				panic!("no mirror");
			}
			sum += mirror_col.unwrap_or(0) + 100 * mirror_row.unwrap_or(0);
		}
		sum
	});
}
