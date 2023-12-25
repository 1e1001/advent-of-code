use std::collections::HashSet;

use ahash::{AHashMap, AHashSet};
use aoc2023::*;

fn main() {
	part1(|input| {
		fn rect_iter(
			r: (isize, isize, isize, isize, isize, isize),
		) -> impl Iterator<Item = (isize, isize, isize)> {
			if r.0 > r.3 || r.1 > r.4 || r.2 > r.5 {
				panic!("shit");
			}
			(r.0..=r.3).flat_map(move |x| {
				(r.1..=r.4).flat_map(move |y| (r.2..=r.5).map(move |z| (x, y, z)))
			})
		}
		let mut map = AHashMap::new();
		let mut rects = Vec::new();
		println!("# collect");
		for v in input.lines() {
			let (x1, v) = v.split_once(',').unwrap();
			let (y1, v) = v.split_once(',').unwrap();
			let (z1, v) = v.split_once('~').unwrap();
			let (x2, v) = v.split_once(',').unwrap();
			let (y2, z2) = v.split_once(',').unwrap();
			let x1: isize = x1.parse().unwrap();
			let y1: isize = y1.parse().unwrap();
			let z1: isize = z1.parse().unwrap();
			let x2: isize = x2.parse().unwrap();
			let y2: isize = y2.parse().unwrap();
			let z2: isize = z2.parse().unwrap();
			let rect = (x1, y1, z1, x2, y2, z2);
			rects.push(rect);
		}
		println!("# map");
		for (i, rect) in rects.iter().enumerate() {
			for pos in rect_iter(*rect) {
				map.insert(pos, i);
			}
		}
		fn grid_at(
			map: &AHashMap<(isize, isize, isize), usize>,
			pos: (isize, isize, isize),
		) -> Option<usize> {
			if pos.2 <= 0 {
				Some(usize::MAX)
			} else {
				map.get(&pos).copied()
			}
		}
		println!("# physics");
		let mut i = 0;
		loop {
			println!("# {i}");
			i += 1;
			let mut change = false;
			#[allow(clippy::needless_range_loop)]
			for i in 0..rects.len() {
				let rect = rects[i];
				if !rect_iter(rect).any(|v| {
					let v = grid_at(&map, (v.0, v.1, v.2 - 1));
					v.is_some() && v != Some(i)
				}) {
					for pos in rect_iter(rect) {
						map.remove(&pos);
						map.insert((pos.0, pos.1, pos.2 - 1), i);
					}
					rects[i].2 -= 1;
					rects[i].5 -= 1;
					change = true;
				}
			}
			if !change {
				break;
			}
		}
		println!("# count");
		let mut invalid = AHashSet::new();
		for (i, rect) in rects.iter().enumerate() {
			let support = rect_iter(*rect)
				.filter_map(|v| grid_at(&map, (v.0, v.1, v.2 - 1)))
				.filter(|v| *v != i)
				.collect::<AHashSet<_>>()
				.into_iter()
				.collect::<Vec<_>>();
			if support.is_empty() {
				panic!("bad phys");
			}
			if support.len() == 1 && support[0] != usize::MAX {
				invalid.insert(support[0]);
			}
		}
		println!("# model");
		let mut vc = 1;
		for (cube, _) in &map {
			println!("v {} {} {}", cube.0, cube.2, cube.1);
			println!("v {} {} {}", cube.0, cube.2, cube.1 + 1);
			println!("v {} {} {}", cube.0, cube.2 + 1, cube.1);
			println!("v {} {} {}", cube.0, cube.2 + 1, cube.1 + 1);
			println!("v {} {} {}", cube.0 + 1, cube.2, cube.1);
			println!("v {} {} {}", cube.0 + 1, cube.2, cube.1 + 1);
			println!("v {} {} {}", cube.0 + 1, cube.2 + 1, cube.1);
			println!("v {} {} {}", cube.0 + 1, cube.2 + 1, cube.1 + 1);
			println!("f {} {} {} {}", vc, vc + 2, vc + 6, vc + 4);
			println!("f {} {} {} {}", vc + 1, vc + 3, vc + 7, vc + 5);
			println!("f {} {} {} {}", vc, vc + 1, vc + 5, vc + 4);
			println!("f {} {} {} {}", vc + 2, vc + 3, vc + 7, vc + 6);
			println!("f {} {} {} {}", vc, vc + 1, vc + 3, vc + 2);
			println!("f {} {} {} {}", vc + 4, vc + 5, vc + 7, vc + 6);
			vc += 8;
		}
		rects.len() - invalid.len()
	});
	part2(|input| {
		fn rect_iter(
			r: (isize, isize, isize, isize, isize, isize),
		) -> impl Iterator<Item = (isize, isize, isize)> {
			if r.0 > r.3 || r.1 > r.4 || r.2 > r.5 {
				panic!("shit");
			}
			(r.0..=r.3).flat_map(move |x| {
				(r.1..=r.4).flat_map(move |y| (r.2..=r.5).map(move |z| (x, y, z)))
			})
		}
		let mut map = AHashMap::new();
		let mut rects = Vec::new();
		for v in input.lines() {
			let (x1, v) = v.split_once(',').unwrap();
			let (y1, v) = v.split_once(',').unwrap();
			let (z1, v) = v.split_once('~').unwrap();
			let (x2, v) = v.split_once(',').unwrap();
			let (y2, z2) = v.split_once(',').unwrap();
			let x1: isize = x1.parse().unwrap();
			let y1: isize = y1.parse().unwrap();
			let z1: isize = z1.parse().unwrap();
			let x2: isize = x2.parse().unwrap();
			let y2: isize = y2.parse().unwrap();
			let z2: isize = z2.parse().unwrap();
			let rect = (x1, y1, z1, x2, y2, z2);
			rects.push(rect);
		}
		for (i, rect) in rects.iter().enumerate() {
			for pos in rect_iter(*rect) {
				map.insert(pos, i);
			}
		}
		fn grid_at(
			map: &AHashMap<(isize, isize, isize), usize>,
			pos: (isize, isize, isize),
		) -> Option<usize> {
			if pos.2 <= 0 {
				Some(usize::MAX)
			} else {
				map.get(&pos).copied()
			}
		}
		loop {
			let mut change = false;
			#[allow(clippy::needless_range_loop)]
			for i in 0..rects.len() {
				let rect = rects[i];
				if !rect_iter(rect).any(|v| {
					let v = grid_at(&map, (v.0, v.1, v.2 - 1));
					v.is_some() && v != Some(i)
				}) {
					for pos in rect_iter(rect) {
						map.remove(&pos);
						map.insert((pos.0, pos.1, pos.2 - 1), i);
					}
					rects[i].2 -= 1;
					rects[i].5 -= 1;
					change = true;
				}
			}
			if !change {
				break;
			}
		}
		println!("digraph what{{");
		let mut incoming: AHashMap<usize, AHashSet<usize>> = AHashMap::new();
		for (i, rect) in rects.iter().enumerate() {
			for support in rect_iter(*rect)
				.filter_map(|v| grid_at(&map, (v.0, v.1, v.2 - 1)))
				.filter(|v| *v != i)
				.collect::<AHashSet<_>>()
			{
				incoming.entry(i).or_default().insert(support);
				println!("{support} -> {i}");
			}
		}
		println!("}}");
		(0..rects.len())
			.map(|i| {
				let mut invalid = HashSet::new();
				invalid.insert(i);
				loop {
					let mut change = false;
					for i in 0..rects.len() {
						if invalid.contains(&i) {
							continue;
						}
						if !incoming
							.get(&i)
							.unwrap()
							.iter()
							.any(|v| !invalid.contains(v))
						{
							invalid.insert(i);
							change = true;
						}
					}
					if !change {
						break;
					}
				}
				invalid.len() - 1
			})
			.sum::<usize>()
	});
}
