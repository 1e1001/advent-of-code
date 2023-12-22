use std::ops::Range;

use ahash::AHashMap;
use aoc2023::*;

fn main() {
	part1(|input| {
		let mut input = input.lines();
		let mut rules = AHashMap::new();
		loop {
			let next = input.next().unwrap();
			if next.is_empty() {
				break;
			}
			let (name, data) = next.split_once('{').unwrap();
			let (data, _) = data.split_once('}').unwrap();
			let data = data
				.split(',')
				.map(|v| {
					let Some((first, next)) = v.split_once(['<', '>']) else {
						return Err(v);
					};
					let (second, label) = next.split_once(':').unwrap();
					let op = match &v[first.len()..first.len() + 1] {
						"<" => false,
						">" => true,
						_ => panic!("shit {v:?}"),
					};
					let first = match first {
						"x" => 0usize,
						"m" => 1,
						"a" => 2,
						"s" => 3,
						_ => panic!("shit {first:?}"),
					};
					let second: isize = second.parse().unwrap();
					Ok((first, second, op, label))
				})
				.collect::<Vec<_>>();
			rules.insert(name, data);
		}
		let mut sum = 0;
		for rest in input {
			let (_, rest) = rest.split_once("{x=").unwrap();
			let (xv, rest) = rest.split_once(",m=").unwrap();
			let (mv, rest) = rest.split_once(",a=").unwrap();
			let (av, rest) = rest.split_once(",s=").unwrap();
			let (sv, _) = rest.split_once('}').unwrap();
			let xv: isize = xv.parse().unwrap();
			let mv: isize = mv.parse().unwrap();
			let av: isize = av.parse().unwrap();
			let sv: isize = sv.parse().unwrap();
			let v = [xv, mv, av, sv];
			let mut label = "in";
			while !matches!(label, "A" | "R") {
				for val in &rules[label] {
					match val {
						Ok((l, r, o, goto)) => {
							let l = v[*l];
							if if *o { l > *r } else { l < *r } {
								label = goto;
								break;
							}
						}
						Err(goto) => {
							label = goto;
						}
					}
				}
			}
			if label == "A" {
				sum += xv + mv + av + sv;
			}
		}
		sum
	});
	part2(|input| {
		let mut input = input.lines();
		type Data<'a> = AHashMap<&'a str, Vec<Result<(usize, isize, bool, &'a str), &'a str>>>;
		let mut rules = AHashMap::new();
		loop {
			let next = input.next().unwrap();
			if next.is_empty() {
				break;
			}
			let (name, data) = next.split_once('{').unwrap();
			let (data, _) = data.split_once('}').unwrap();
			let data = data
				.split(',')
				.map(|v| {
					let Some((first, next)) = v.split_once(['<', '>']) else {
						return Err(v);
					};
					let (second, label) = next.split_once(':').unwrap();
					let op = match &v[first.len()..first.len() + 1] {
						"<" => false,
						">" => true,
						_ => panic!("shit {v:?}"),
					};
					let first = match first {
						"x" => 0usize,
						"m" => 1,
						"a" => 2,
						"s" => 3,
						_ => panic!("shit {first:?}"),
					};
					let second: isize = second.parse().unwrap();
					Ok((first, second, op, label))
				})
				.collect::<Vec<_>>();
			rules.insert(name, data);
		}
		fn evaluate_set(
			rules: &Data,
			label: &str,
			mut set: [Range<isize>; 4],
			depth: usize,
		) -> isize {
			for _ in 0..depth {
				print!(" ");
			}
			println!("eval {label} {set:?}");
			if label == "R" || set.iter().any(|v| v.is_empty()) {
				return 0;
			}
			if label == "A" {
				return set.map(|v| v.end - v.start).into_iter().product();
			}
			let mut out = 0;
			for val in &rules[label] {
				match val {
					Ok((l, r, o, goto)) => {
						let (r, o) = (*r, *o);
						let mut copy = set.clone();
						if o {
							set[*l].end = set[*l].end.min(r + 1);
							copy[*l].start = copy[*l].start.max(r + 1);
						} else {
							set[*l].start = set[*l].start.max(r);
							copy[*l].end = copy[*l].end.min(r);
						};
						out += evaluate_set(rules, goto, copy, depth + 1);
					}
					Err(goto) => {
						out += evaluate_set(rules, goto, set, depth + 1);
						break;
					}
				}
			}
			for _ in 0..depth {
				print!(" ");
			}
			println!("-> {out}");
			out
		}
		evaluate_set(&rules, "in", [1..4001, 1..4001, 1..4001, 1..4001], 0)
	});
}
