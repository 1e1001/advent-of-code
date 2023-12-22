use ahash::{AHashMap, AHashSet};
use aoc2023::*;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
	Air,
	MirrorRight,
	MirrorLeft,
	SplitV,
	SplitH,
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
}

fn tuple_add(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
	(a.0 + b.0, a.1 + b.1)
}

fn main() {
	part1(|input| {
		let grid = input
			.lines()
			.map(|v| {
				v.chars()
					.map(|v| match v {
						'.' => Tile::Air,
						'/' => Tile::MirrorRight,
						'\\' => Tile::MirrorLeft,
						'|' => Tile::SplitV,
						'-' => Tile::SplitH,
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let mut beams = AHashMap::new();
		beams.insert((0, 0), AHashSet::from_iter([Dir::E]));
		let mut i = 0;
		loop {
			println!("{i}");
			let mut beams_out = beams.clone();
			for (pos, dirs) in beams.iter() {
				if pos.0 < 0
					|| pos.1 < 0 || pos.0 as usize >= grid[0].len()
					|| pos.1 as usize >= grid.len()
				{
					continue;
				}
				for dir in dirs {
					match grid[pos.1 as usize][pos.0 as usize] {
						Tile::Air => {
							beams_out
								.entry(tuple_add(*pos, dir.offset()))
								.or_default()
								.insert(*dir);
						}
						Tile::MirrorRight => {
							let new_dir = match dir {
								Dir::N => Dir::E,
								Dir::E => Dir::N,
								Dir::S => Dir::W,
								Dir::W => Dir::S,
							};
							beams_out
								.entry(tuple_add(*pos, new_dir.offset()))
								.or_default()
								.insert(new_dir);
						}
						Tile::MirrorLeft => {
							let new_dir = match dir {
								Dir::N => Dir::W,
								Dir::E => Dir::S,
								Dir::S => Dir::E,
								Dir::W => Dir::N,
							};
							beams_out
								.entry(tuple_add(*pos, new_dir.offset()))
								.or_default()
								.insert(new_dir);
						}
						Tile::SplitV => match dir {
							Dir::E | Dir::W => {
								for dir in [Dir::N, Dir::S] {
									beams_out
										.entry(tuple_add(*pos, dir.offset()))
										.or_default()
										.insert(dir);
								}
							}
							Dir::N | Dir::S => {
								beams_out
									.entry(tuple_add(*pos, dir.offset()))
									.or_default()
									.insert(*dir);
							}
						},
						Tile::SplitH => match dir {
							Dir::N | Dir::S => {
								for dir in [Dir::E, Dir::W] {
									beams_out
										.entry(tuple_add(*pos, dir.offset()))
										.or_default()
										.insert(dir);
								}
							}
							Dir::E | Dir::W => {
								beams_out
									.entry(tuple_add(*pos, dir.offset()))
									.or_default()
									.insert(*dir);
							}
						},
					}
				}
			}
			if beams == beams_out {
				break;
			}
			beams = beams_out;
			i += 1;
		}
		let mut c = 0;
		for row in 0..grid.len() as isize {
			for col in 0..grid[0].len() as isize {
				print!(
					"{}",
					if beams.contains_key(&(col, row)) {
						c += 1;
						'#'
					} else {
						match grid[col as usize][row as usize] {
							Tile::Air => '.',
							Tile::MirrorRight => '/',
							Tile::MirrorLeft => '\\',
							Tile::SplitV => '|',
							Tile::SplitH => '-',
						}
					}
				);
			}
			println!();
		}
		c
	});
	part2(|input| {
		let grid = input
			.lines()
			.map(|v| {
				v.chars()
					.map(|v| match v {
						'.' => Tile::Air,
						'/' => Tile::MirrorRight,
						'\\' => Tile::MirrorLeft,
						'|' => Tile::SplitV,
						'-' => Tile::SplitH,
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let initials = (0..grid.len() as isize)
			.map(|v| (0, v, Dir::E))
			.chain((0..grid.len() as isize).map(|v| (grid[0].len() as isize - 1, v, Dir::W)))
			.chain((0..grid[0].len() as isize).map(|v| (v, 0, Dir::S)))
			.chain((0..grid[0].len() as isize).map(|v| (v, grid.len() as isize - 1, Dir::N)));
		initials
			.par_bridge()
			.map(|(x, y, d)| {
				println!("{:?}", (x, y, d));
				let mut beams = AHashMap::new();
				beams.insert((x, y), AHashSet::from_iter([d]));
				loop {
					let mut beams_out = beams.clone();
					for (pos, dirs) in beams.iter() {
						if pos.0 < 0
							|| pos.1 < 0 || pos.0 as usize >= grid[0].len()
							|| pos.1 as usize >= grid.len()
						{
							continue;
						}
						for dir in dirs {
							match grid[pos.1 as usize][pos.0 as usize] {
								Tile::Air => {
									beams_out
										.entry(tuple_add(*pos, dir.offset()))
										.or_default()
										.insert(*dir);
								}
								Tile::MirrorRight => {
									let new_dir = match dir {
										Dir::N => Dir::E,
										Dir::E => Dir::N,
										Dir::S => Dir::W,
										Dir::W => Dir::S,
									};
									beams_out
										.entry(tuple_add(*pos, new_dir.offset()))
										.or_default()
										.insert(new_dir);
								}
								Tile::MirrorLeft => {
									let new_dir = match dir {
										Dir::N => Dir::W,
										Dir::E => Dir::S,
										Dir::S => Dir::E,
										Dir::W => Dir::N,
									};
									beams_out
										.entry(tuple_add(*pos, new_dir.offset()))
										.or_default()
										.insert(new_dir);
								}
								Tile::SplitV => match dir {
									Dir::E | Dir::W => {
										for dir in [Dir::N, Dir::S] {
											beams_out
												.entry(tuple_add(*pos, dir.offset()))
												.or_default()
												.insert(dir);
										}
									}
									Dir::N | Dir::S => {
										beams_out
											.entry(tuple_add(*pos, dir.offset()))
											.or_default()
											.insert(*dir);
									}
								},
								Tile::SplitH => match dir {
									Dir::N | Dir::S => {
										for dir in [Dir::E, Dir::W] {
											beams_out
												.entry(tuple_add(*pos, dir.offset()))
												.or_default()
												.insert(dir);
										}
									}
									Dir::E | Dir::W => {
										beams_out
											.entry(tuple_add(*pos, dir.offset()))
											.or_default()
											.insert(*dir);
									}
								},
							}
						}
					}
					if beams == beams_out {
						break;
					}
					beams = beams_out;
				}
				let mut c = 0;
				for row in 0..grid.len() as isize {
					for col in 0..grid[0].len() as isize {
						if beams.contains_key(&(col, row)) {
							c += 1;
						}
					}
				}
				c
			})
			.max()
	});
}
