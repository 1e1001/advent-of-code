use std::collections::VecDeque;

use ahash::AHashMap;
use aoc2023::*;

fn main() {
	part1(|input| {
		enum Machine<'a> {
			Flip(Vec<&'a str>, bool),
			Conj(Vec<&'a str>, AHashMap<&'a str, bool>),
			Broadcast(Vec<&'a str>),
		}
		let mut machines = input
			.lines()
			.map(|v| {
				let (name, out) = v.split_once(" -> ").unwrap();
				let out = out.split(", ").collect::<Vec<_>>();
				if let Some(name) = name.strip_prefix('%') {
					(name, Machine::Flip(out, false))
				} else if let Some(name) = name.strip_prefix('&') {
					(name, Machine::Conj(out, AHashMap::new()))
				} else {
					assert_eq!(name, "broadcaster");
					(name, Machine::Broadcast(out))
				}
			})
			.collect::<AHashMap<_, _>>();
		// update refs
		let mut queue = Vec::new();
		for (id, machine) in &machines {
			for o in match machine {
				Machine::Flip(o, _) => o,
				Machine::Conj(o, _) => o,
				Machine::Broadcast(o) => o,
			} {
				queue.push(((*id).to_owned(), (*o).to_owned()));
			}
		}
		for (id, o) in queue {
			if let Some(Machine::Conj(_, v)) = machines.get_mut(&*o) {
				v.insert(id.leak(), false);
			}
		}
		struct Pulse<'a> {
			id: &'a str,
			from: &'a str,
			dir: bool,
		}
		fn pulse<'a>(id: &'a str, from: &'a str, dir: bool) -> Pulse<'a> {
			Pulse { id, from, dir }
		}
		let mut queue = VecDeque::new();
		let mut counters = [0, 0];
		for _ in 0..1000 {
			queue.push_back(pulse("broadcaster", "button", false));
			while let Some(Pulse { id, from, dir }) = queue.pop_front() {
				println!("{from} -> {id} {dir}");
				counters[dir as usize] += 1;
				match machines.get_mut(id) {
					Some(Machine::Flip(out, state)) => {
						if !dir {
							*state = !*state;
							let out = out.clone();
							let state = *state;
							for i in out {
								queue.push_back(pulse(i, id, state));
							}
						}
					}
					Some(Machine::Conj(out, state)) => {
						*state.get_mut(from).unwrap() = dir;
						let dir = !state.values().all(|v| *v);
						let out = out.clone();
						for i in out {
							queue.push_back(pulse(i, id, dir));
						}
					}
					Some(Machine::Broadcast(out)) => {
						for i in out.clone() {
							queue.push_back(pulse(i, id, dir));
						}
					}
					None => continue,
				}
			}
		}
		// todo!()
		counters[0] * counters[1]
	});
	part2(|input| {
		// solved externally
		enum Machine<'a> {
			Flip(Vec<&'a str>, bool),
			Conj(Vec<&'a str>, AHashMap<&'a str, bool>),
			Broadcast(Vec<&'a str>),
		}
		let mut machines = input
			.lines()
			.map(|v| {
				let (name, out) = v.split_once(" -> ").unwrap();
				let out = out.split(", ").collect::<Vec<_>>();
				if let Some(name) = name.strip_prefix('%') {
					(name, Machine::Flip(out, false))
				} else if let Some(name) = name.strip_prefix('&') {
					(name, Machine::Conj(out, AHashMap::new()))
				} else {
					assert_eq!(name, "broadcaster");
					(name, Machine::Broadcast(out))
				}
			})
			.collect::<AHashMap<_, _>>();
		// update refs
		let mut queue = Vec::new();
		for (id, machine) in &machines {
			for o in match machine {
				Machine::Flip(o, _) => o,
				Machine::Conj(o, _) => o,
				Machine::Broadcast(o) => o,
			} {
				queue.push(((*id).to_owned(), (*o).to_owned()));
			}
		}
		for (id, o) in queue {
			if let Some(Machine::Conj(_, v)) = machines.get_mut(&*o) {
				v.insert(id.leak(), false);
			}
		}
		struct Pulse<'a> {
			id: &'a str,
			from: &'a str,
			dir: bool,
		}
		fn pulse<'a>(id: &'a str, from: &'a str, dir: bool) -> Pulse<'a> {
			Pulse { id, from, dir }
		}
		let mut queue = VecDeque::new();
		let mut counters = [0, 0];
		let mut prev_cycle = AHashMap::new();
		for i in 1usize.. {
			// if i % 100000 == 0 {
			// 	println!("{i}");
			// }
			queue.push_back(pulse("broadcaster", "button", false));
			while let Some(Pulse { id, from, dir }) = queue.pop_front() {
				// println!("{from} -> {id} {dir}");
				counters[dir as usize] += 1;
				match machines.get_mut(id) {
					Some(Machine::Flip(out, state)) => {
						if !dir {
							*state = !*state;
							let out = out.clone();
							let state = *state;
							for i in out {
								queue.push_back(pulse(i, id, state));
							}
						}
					}
					Some(Machine::Conj(out, state)) => {
						*state.get_mut(from).unwrap() = dir;
						let dir = !state.values().all(|v| *v);
						let out = out.clone();
						if dir && matches!(id, "dc" | "vp" | "cq" | "rv") {
							println!(
								"{id} {:>8}",
								i - std::mem::replace(prev_cycle.entry(id).or_default(), i)
							);
						}
						for i in out {
							queue.push_back(pulse(i, id, dir));
						}
					}
					Some(Machine::Broadcast(out)) => {
						for i in out.clone() {
							queue.push_back(pulse(i, id, dir));
						}
					}
					None => {
						if id == "rx" && !dir {
							return i;
						}
					}
				}
			}
		}
		usize::MAX
	});
}
