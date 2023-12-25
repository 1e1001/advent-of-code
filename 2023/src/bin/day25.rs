use ahash::{AHashMap, AHashSet};
use aoc2023::*;

fn main() {
	part1(|input| {
		const SEP: &[(&str, &str)] = &[("kpc", "nnl"), ("mnf", "hrs"), ("rkh", "sph")];
		// const SEP: &[(&str, &str)] = &[("hfx", "pzl"), ("bvb", "cmg"),
		// ("nvd", "jqt")];
		// println!("graph asd {{");
		let mut graph: AHashMap<&str, AHashSet<&str>> = AHashMap::new();
		for (name, conn) in input.lines().map(|v| {
			let (name, v) = v.split_once(": ").unwrap();
			(name, v.split(' ').collect::<Vec<_>>())
		}) {
			for conn in conn {
				graph.entry(name).or_default().insert(conn);
				graph.entry(conn).or_default().insert(name);
			}
			// println!("{name} -- {}", conn.join(", "));
		}
		for (a, b) in SEP {
			graph.get_mut(a).unwrap().remove(b);
			graph.get_mut(b).unwrap().remove(a);
		}
		// println!("}}");
		fn count_nodes(graph: &AHashMap<&str, AHashSet<&str>>, start: &str) -> usize {
			let mut explored = AHashSet::new();
			let mut queue = Vec::new();
			queue.push(start);
			while let Some(next) = queue.pop() {
				if explored.insert(next) {
					queue.extend(graph.get(next).unwrap());
				}
			}
			explored.len()
		}
		[SEP[0].0, SEP[0].1]
			.into_iter()
			.map(|v| count_nodes(&graph, v))
			.product::<usize>()
	});
	part2(|_input| "lol, lmao");
}
