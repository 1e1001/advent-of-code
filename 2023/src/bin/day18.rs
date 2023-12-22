use std::collections::VecDeque;

use ahash::AHashMap;
use aoc2023::*;

fn main() {
	part1(|input| {
		let mut grid = AHashMap::new();
		let mut cursor = (0isize, 0isize);
		let mut min = (0isize, 0isize);
		let mut max = (0isize, 0isize);
		for i in input.lines() {
			let mut iter = i.split(' ');
			let dir = iter.next().unwrap();
			let len: usize = iter.next().unwrap().parse().unwrap();
			// let color = &iter.next().unwrap()[2..8];
			let diff = match dir {
				"U" => |(x, y)| (x, y - 1),
				"D" => |(x, y)| (x, y + 1),
				"L" => |(x, y)| (x - 1, y),
				"R" => |(x, y)| (x + 1, y),
				_ => panic!("shit {dir:?}"),
			};
			for _ in 0..len {
				grid.insert(cursor, true);
				cursor = diff(cursor);
				max.0 = max.0.max(cursor.0);
				max.1 = max.1.max(cursor.1);
				min.0 = min.0.min(cursor.0);
				min.1 = min.1.min(cursor.1);
			}
		}
		// flood fill
		let mut queue = VecDeque::new();
		queue.push_back(min);
		while let Some(pos) = queue.pop_front() {
			if grid.contains_key(&pos)
				|| pos.0 < min.0 - 1
				|| pos.1 < min.1 - 1
				|| pos.0 > max.0 + 1
				|| pos.1 > max.1 + 1
			{
				continue;
			}
			grid.insert(pos, false);
			queue.push_back((pos.0, pos.1 + 1));
			queue.push_back((pos.0, pos.1 - 1));
			queue.push_back((pos.0 + 1, pos.1));
			queue.push_back((pos.0 - 1, pos.1));
		}
		let mut count = 0usize;
		for y in min.1..=max.1 {
			for x in min.0..=max.0 {
				let ch = match grid.get(&(x, y)) {
					Some(true) => {
						count += 1;
						'#'
					}
					Some(false) => '.',
					None => {
						count += 1;
						'%'
					}
				};
				print!("{ch}");
			}
			println!();
		}
		count
	});
	part2(|input| {
		let mut verts = Vec::new();
		let mut cursor = (0isize, 0isize);
		let mut parea = 2;
		for i in input.lines() {
			let mut iter = i.split(' ');
			let _dir = iter.next().unwrap();
			let _len: isize = iter.next().unwrap().parse().unwrap();
			let last = iter.next().unwrap();
			let dir = match &last[7..8] {
				"0" => "R",
				"1" => "D",
				"2" => "L",
				"3" => "U",
				_ => panic!("shit {last:?}"),
			};
			let len = isize::from_str_radix(&last[2..7], 16).unwrap();
			let diff = match dir {
				"U" => |(x, y), v| (x, y - v),
				"D" => |(x, y), v| (x, y + v),
				"L" => |(x, y), v| (x - v, y),
				"R" => |(x, y), v| (x + v, y),
				_ => panic!("shit {dir:?}"),
			};
			verts.push(cursor);
			cursor = diff(cursor, len);
			parea += len;
		}
		// https://algorithmtutor.com/Computational-Geometry/Area-of-a-polygon-given-a-set-of-points/
		let mut psum = 0;
		let mut nsum = 0;
		for (v, nv) in verts
			.iter()
			.copied()
			.zip(verts.iter().copied().skip(1).chain([verts[0]]))
		{
			psum += v.0 * nv.1;
			nsum += nv.0 * v.1;
		}
		((psum - nsum) / 2).abs() + parea / 2
	});
}
