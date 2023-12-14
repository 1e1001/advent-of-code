use aoc2023::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
	None,
	Stationary,
	Moving,
}
impl Tile {
	fn ch(self) -> char {
		match self {
			Self::None => '.',
			Self::Stationary => '#',
			Self::Moving => 'O',
		}
	}
}

fn main() {
	part1(|input| {
		let grid = input
			.lines()
			.map(|v| {
				v.chars()
					.map(|v| match v {
						'.' => Tile::None,
						'#' => Tile::Stationary,
						'O' => Tile::Moving,
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let mut transpose = vec![Vec::new(); grid[0].len()];
		for line in grid {
			for (i, ch) in line.into_iter().enumerate() {
				transpose[i].push(ch);
			}
		}
		for (i, row) in transpose.iter_mut().enumerate() {
			println!("row {i}");
			let mut prev_air;
			let mut change = true;
			while change {
				change = false;
				prev_air = None;
				for i in 0..row.len() {
					match row[i] {
						Tile::None => prev_air = Some(i),
						Tile::Stationary => prev_air = None,
						Tile::Moving => {
							if let Some(prev) = prev_air {
								row.swap(prev, i);
								prev_air = None;
								change = true;
							}
						}
					}
				}
			}
		}
		for row in &transpose {
			for col in row {
				print!("{}", col.ch());
			}
			println!();
		}
		transpose
			.iter()
			.map(|row| {
				row.iter()
					.enumerate()
					.filter_map(|(i, v)| match v {
						Tile::Moving => Some(row.len() - i),
						_ => None,
					})
					.sum::<usize>()
			})
			.sum::<usize>()
	});
	part2(|input| {
		// partially done by hand
		let grid = input
			.lines()
			.map(|v| {
				v.chars()
					.map(|v| match v {
						'.' => Tile::None,
						'#' => Tile::Stationary,
						'O' => Tile::Moving,
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		fn transpose(grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
			let mut transpose = vec![Vec::new(); grid[0].len()];
			for line in grid {
				for (i, ch) in line.into_iter().enumerate() {
					transpose[i].push(ch);
				}
			}
			transpose
		}
		fn mirror_v(grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
			grid.into_iter().rev().collect()
		}
		fn shift_l(mut grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
			for row in &mut grid {
				let mut prev_air;
				let mut change = true;
				while change {
					change = false;
					prev_air = None;
					for i in 0..row.len() {
						match row[i] {
							Tile::None => prev_air = Some(i),
							Tile::Stationary => prev_air = None,
							Tile::Moving => {
								if let Some(prev) = prev_air {
									row.swap(prev, i);
									prev_air = None;
									change = true;
								}
							}
						}
					}
				}
			}
			grid
		}
		fn cycle(grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
			// north
			let grid = shift_l(transpose(grid));
			// west
			let grid = shift_l(transpose(grid));
			// south
			let grid = shift_l(transpose(mirror_v(grid)));
			// east
			let grid = shift_l(transpose(mirror_v(grid)));
			mirror_v(transpose(mirror_v(transpose(grid))))
		}
		fn evaluate(grid: Vec<Vec<Tile>>) -> usize {
			transpose(grid)
				.iter()
				.map(|row| {
					row.iter()
						.enumerate()
						.filter_map(|(i, v)| match v {
							Tile::Moving => Some(row.len() - i),
							_ => None,
						})
						.sum::<usize>()
				})
				.sum::<usize>()
		}
		let mut grid = grid;
		let mut hist = Vec::new();
		for i in 1.. {
			grid = cycle(grid);
			let eval = evaluate(grid.clone());
			hist.push(eval);
			println!(
				"{i}: {eval}, {:>2?}",
				hist.iter()
					.rev()
					.enumerate()
					.skip(1)
					.filter(|v| *v.1 == eval)
					.map(|v| v.0)
					.take(10)
					.collect::<Vec<_>>()
			);
		}
	});
}
