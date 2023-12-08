use ahash::AHashMap;
use aoc2023::*;

fn main() {
	part1(|input| {
		let mut nodes = AHashMap::new();
		let mut input = input.lines();
		let instr = input.next().unwrap();
		input.next();
		for i in input {
			// RTF = (TRM, KNP)
			let name = &i[0..3];
			let left = &i[7..10];
			let right = &i[12..15];
			nodes.insert(name, [left, right]);
		}
		let mut node = "AAA";
		let mut c = 0;
		for i in instr.chars().cycle() {
			node = nodes[node][match i {
				'L' => 0,
				'R' => 1,
				_ => panic!("shit {i}"),
			}];
			c += 1;
			if node == "ZZZ" {
				break;
			}
		}
		c
	});
	part2(|input| {
		// done manually
		// https://www.desmos.com/calculator/tofvbnn2j5
		let mut nodes = AHashMap::new();
		let mut input = input.lines();
		let instr = input.next().unwrap();
		input.next();
		for i in input {
			// RTF = (TRM, KNP)
			let name = &i[0..3];
			let left = &i[7..10];
			let right = &i[12..15];
			nodes.insert(name, [left, right]);
		}
		let node_id = 5;
		let mut zs = nodes.keys().filter(|v| &v[2..3] == "Z").collect::<Vec<_>>();
		zs.sort();
		let mut node = *zs[node_id];
		println!("node {}/{}", node_id, zs.len());
		let mut o = 0usize;
		let mut c = 0usize;
		for i in instr.chars().cycle() {
			node = nodes[node][match i {
				'L' => 0,
				'R' => 1,
				_ => panic!("shit {i}"),
			}];
			c += 1;
			if &node[2..3] == "Z" {
				println!("{} {}", c - o, c % (c - o));
				o = c;
			}
		}
		c
	});
}
