use std::ops::Range;

use aoc2023::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IdRange {
	src: usize,
	dst: usize,
	len: usize,
}
impl IdRange {
	#[allow(clippy::unnecessary_lazy_evaluations)]
	fn map(self, id: usize) -> Option<usize> {
		(self.src..self.src + self.len)
			.contains(&id)
			.then(|| id - self.src + self.dst)
	}
	fn map_range(map: &[Self], mut id: Range<usize>) -> Vec<Range<usize>> {
		let mut out = Vec::new();
		// println!("map_range {id:?} {map:?}");
		for m in map {
			if id.start > m.src + m.len {
				continue;
			}
			if id.end < m.src {
				break;
			}
			if id.start < m.src {
				// println!("  a {:?}", id.start..m.src);
				out.push(id.start..m.src);
				id.start = m.src;
			}
			let start = id.start.max(m.src);
			let end = id.end.min(m.src + m.len);
			// println!("  b {:?}", start - m.src + m.dst..end - m.src + m.dst);
			out.push(start - m.src + m.dst..end - m.src + m.dst);
			id.start = end;
		}
		if !id.is_empty() {
			out.push(id);
		}
		out
	}
}

fn main() {
	part1(|input| {
		let mut input = input.lines();
		let initial = input
			.next()
			.unwrap()
			.split_once(": ")
			.unwrap()
			.1
			.split(' ')
			.map(|v| v.parse().unwrap())
			.collect::<Vec<usize>>();
		let mut maps = Vec::new();
		input.next();
		while input.next().is_some() {
			let mut map = Vec::new();
			loop {
				let text = input.next().unwrap_or_default();
				if text.is_empty() {
					break;
				}
				let next = text
					.split(' ')
					.map(|v| v.parse().unwrap())
					.collect::<Vec<usize>>();
				assert_eq!(next.len(), 3);
				map.push(IdRange {
					dst: next[0],
					src: next[1],
					len: next[2],
				});
			}
			map.sort_unstable();
			maps.push(map)
		}
		let mut valid = initial;
		for (i, map) in maps.into_iter().enumerate() {
			println!("{i} {}", valid.len());
			for v in &mut valid {
				for r in &map {
					if let Some(res) = r.map(*v) {
						println!("map {v} {r:?}\t-> {res}");
						*v = res;
						break;
					}
				}
			}
		}
		valid.into_iter().min()
	});
	part2(|input| {
		let mut input = input.lines();
		let initial = input
			.next()
			.unwrap()
			.split_once(": ")
			.unwrap()
			.1
			.split(' ')
			.map(|v| v.parse().unwrap())
			.collect::<Vec<usize>>();
		let mut maps = Vec::new();
		input.next();
		while input.next().is_some() {
			let mut map = Vec::new();
			loop {
				let text = input.next().unwrap_or_default();
				if text.is_empty() {
					break;
				}
				let next = text
					.split(' ')
					.map(|v| v.parse().unwrap())
					.collect::<Vec<usize>>();
				assert_eq!(next.len(), 3);
				map.push(IdRange {
					dst: next[0],
					src: next[1],
					len: next[2],
				});
			}
			map.sort_unstable();
			maps.push(map)
		}
		let mut valid = Vec::new();
		let mut iter = initial.into_iter();
		while let Some(a) = iter.next() {
			let l = iter.next().unwrap();
			valid.push(a..a + l);
		}
		for (i, map) in maps.into_iter().enumerate() {
			println!("{i} {}", valid.len());
			let mut next = Vec::new();
			for v in valid {
				next.extend_from_slice(&IdRange::map_range(&map, v));
			}
			valid = next;
		}
		valid.into_iter().map(|v| v.start).min()
	});
}
