use aoc2023::*;

fn main() {
	part1(|input| {
		input
			.split(',')
			.map(|v| {
				let mut hash = 0;
				for b in v.bytes().filter(|v| *v != b'\n') {
					hash += b as usize;
					hash *= 17;
					hash %= 256;
				}
				dbg!(hash)
			})
			.sum::<usize>()
	});
	part2(|input| {
		let mut boxes = vec![Vec::<(&str, usize)>::new(); 256];
		input.split(',').for_each(|mut v| {
			if v.ends_with('\n') {
				v = &v[..v.len() - 1];
			}
			let mut hash = 0;
			let split_index = v.find(['-', '=']).unwrap();
			let label = &v[..split_index];
			dbg!(label);
			for b in label.bytes() {
				hash += b as usize;
				hash *= 17;
				hash %= 256;
			}
			dbg!(hash);
			let index = boxes[hash]
				.iter()
				.enumerate()
				.find_map(|v| (v.1 .0 == label).then_some(v.0));
			match v.as_bytes()[split_index] {
				b'=' => {
					let focal: usize = v[split_index + 1..].parse().unwrap();
					dbg!(focal);
					if let Some(index) = index {
						boxes[hash][index] = (label, focal);
					} else {
						boxes[hash].push((label, focal))
					}
				}
				b'-' => {
					index.map(|index| boxes[hash].remove(index));
				}
				_ => panic!("shit {v:?}"),
			};
		});
		dbg!(boxes)
			.into_iter()
			.enumerate()
			.flat_map(|(i, v)| {
				v.into_iter()
					.enumerate()
					.map(move |(ii, vv)| dbg!((1 + i) * (1 + ii) * vv.1))
			})
			.sum::<usize>()
	});
}
