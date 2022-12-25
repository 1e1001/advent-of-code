use std::future::{self, Future};
use std::sync::Mutex;

use async_recursion::async_recursion;
use tokio::task::JoinError;

fn fmt_s(i: usize) -> &'static str {
	if i == 1 {
		""
	} else {
		"s"
	}
}
fn fmt_si(i: usize) -> &'static str {
	if i == 1 {
		"s"
	} else {
		""
	}
}

#[derive(Clone)]
struct Stack(u8, [u8; 32]);
impl Stack {
	fn new() -> Self {
		Self(0, [0; 32])
	}
	fn to_vec(&self) -> Vec<usize> {
		// let mut tail = Some(self);
		// let mut res = Vec::new();
		// while let Some(part) = tail {
		// 	res.push(part.0);
		// 	tail = part.1.clone();
		// }
		// res.reverse();
		// res
		self.1[..self.0 as usize]
			.iter()
			.copied()
			.map(usize::from)
			.collect()
	}
	fn push(&self, v: usize) -> Self {
		let mut res = self.1.clone();
		res[self.0 as usize] = v as u8;
		Stack(self.0 + 1, res)
		// Arc::new(Self(v, Some(self)))
	}
}

struct Best(Mutex<(usize, Vec<usize>)>);
impl Best {
	fn new() -> Self {
		Self(Mutex::new((0, vec![])))
	}
	fn get(&self) -> usize {
		self.0.lock().unwrap().0
	}
	fn set(&self, out: usize, stack: Vec<usize>) {
		*self.0.lock().unwrap() = (out, stack);
	}
	fn print(&self, mut state: State) {
		let r = self.0.lock().unwrap();
		let (out, stack) = &*r;
		for (i, o) in stack.iter().enumerate() {
			println!("\n== Minute {} ==", i + 1);
			match o {
				0 => {
					println!(
						"Spend {} ore and {} obsidian to start building a geode-cracking robot.",
						state.bp.geode_robot.0, state.bp.geode_robot.1
					);
				}
				1 => {
					println!(
						"Spend {} ore and {} clay to start building an obsidian-collecting robot.",
						state.bp.obsidian_robot.0, state.bp.obsidian_robot.1
					);
				}
				2 => {
					println!(
						"Spend {} ore to start building a clay-collecting robot.",
						state.bp.clay_robot
					);
				}
				3 => {
					println!(
						"Spend {} ore to start building an ore-collecting robot.",
						state.bp.ore_robot
					);
				}
				4 => {}
				_ => unreachable!(),
			}
			let next_state = state.advance(*o);
			if state.ore_robots > 0 {
				println!(
					"{} ore-collecting robot{} collect{} {} ore; you now have {} ore.",
					state.ore_robots,
					fmt_s(state.ore_robots),
					fmt_si(state.ore_robots),
					state.ore_robots,
					next_state.ore
				);
			}
			if state.clay_robots > 0 {
				println!(
					"{} clay-collecting robot{} collect{} {} clay; you now have {} clay.",
					state.clay_robots,
					fmt_s(state.clay_robots),
					fmt_si(state.clay_robots),
					state.clay_robots,
					next_state.clay
				);
			}
			if state.obsidian_robots > 0 {
				println!(
					"{} obsidian-collecting robot{} collect{} {} obsidian; you now have {} obsidian.",
					state.obsidian_robots,
					fmt_s(state.obsidian_robots),
					fmt_si(state.obsidian_robots),
					state.obsidian_robots,
					next_state.obsidian
				);
			}
			if state.geode_robots > 0 {
				println!(
					"{} geode-cracking robot{} crack{} {} geodes; you now have {} open geodes.",
					state.geode_robots,
					fmt_s(state.geode_robots),
					fmt_si(state.geode_robots),
					state.geode_robots,
					next_state.geodes
				);
			}
			match o {
				0 => {
					println!(
						"The new geode-cracking robot is ready; you now have {} of them.",
						next_state.geode_robots,
					);
				}
				1 => {
					println!(
						"The new obsidian-collecting robot is ready; you now have {} of them.",
						next_state.obsidian_robots,
					);
				}
				2 => {
					println!(
						"The new clay-collecting robot is ready; you now have {} of them.",
						next_state.clay_robots,
					);
				}
				3 => {
					println!(
						"The new ore-collecting robot is ready; you now have {} of them.",
						next_state.ore_robots,
					);
				}
				4 => {}
				_ => unreachable!(),
			}
			state = next_state;
		}
		println!("--> {out} geodes");
	}
}

static CURRENT_TASKS: Mutex<usize> = Mutex::new(0);
const MAX_TASKS: usize = 20000000;

async fn capped_future<F, O>(f: F) -> Box<dyn Future<Output = Result<O, JoinError>> + Send + Unpin>
where
	F: Future<Output = O> + Send + 'static,
	O: Send + 'static,
{
	let run = {
		let mut lock = CURRENT_TASKS.lock().unwrap();
		let run = *lock < MAX_TASKS;
		if run {
			*lock += 1;
		}
		run
	};
	if run {
		Box::new(tokio::task::spawn(async move {
			let res = f.await;
			*CURRENT_TASKS.lock().unwrap() -= 1;
			res
		}))
	} else {
		let res = f.await;
		Box::new(future::ready(Ok(res)))
	}
}

struct Blueprint {
	index: usize,
	// ore
	ore_robot: usize,
	// ore
	clay_robot: usize,
	// ore, clay
	obsidian_robot: (usize, usize),
	// ore, obsidian
	geode_robot: (usize, usize),
}
#[derive(Clone, Copy)]
struct State {
	bp: &'static Blueprint,
	time: usize,
	ore_robots: usize,
	clay_robots: usize,
	obsidian_robots: usize,
	geode_robots: usize,
	ore: usize,
	clay: usize,
	obsidian: usize,
	geodes: usize,
}
impl State {
	// geode, obsidian, clay, ore, nothing
	fn can_make(&self) -> [bool; 5] {
		[
			self.bp.geode_robot.0 <= self.ore && self.bp.geode_robot.1 <= self.obsidian,
			self.bp.obsidian_robot.0 <= self.ore && self.bp.obsidian_robot.1 <= self.clay,
			self.bp.clay_robot <= self.ore,
			self.bp.ore_robot <= self.ore,
			true,
		]
	}
	fn advance(mut self, op: usize) -> Self {
		self.time -= 1;
		self.ore += self.ore_robots;
		self.clay += self.clay_robots;
		self.obsidian += self.obsidian_robots;
		self.geodes += self.geode_robots;
		match op {
			0 => {
				self.ore -= self.bp.geode_robot.0;
				self.obsidian -= self.bp.geode_robot.1;
				self.geode_robots += 1;
			}
			1 => {
				self.ore -= self.bp.obsidian_robot.0;
				self.clay -= self.bp.obsidian_robot.1;
				self.obsidian_robots += 1;
			}
			2 => {
				self.ore -= self.bp.clay_robot;
				self.clay_robots += 1;
			}
			3 => {
				self.ore -= self.bp.ore_robot;
				self.ore_robots += 1;
			}
			4 => {}
			_ => unreachable!(),
		}
		self
	}
	fn upper_limit(self) -> usize {
		self.time * (self.time - 1) / 2 + self.geodes + self.geode_robots * self.time
	}
	#[async_recursion]
	async fn simulate(self, best: &'static Best, stack: Stack) {
		if self.time == 0 {
			if self.geodes > best.get() {
				println!("{}: new best: {}", self.bp.index, self.geodes);
				best.set(self.geodes, stack.to_vec());
			}
			return;
		}
		if self.upper_limit() <= best.get() {
			return;
		}
		//print!("\rsimulate {stack:?}\x1b[K");
		//std::io::stdout().flush().unwrap();
		let mut inner = Vec::new();
		for (i, v) in self.can_make().into_iter().enumerate() {
			if v {
				inner.push(capped_future(self.advance(i).simulate(best, stack.push(i))).await);
			}
		}
		for i in inner {
			i.await.unwrap();
		}
	}
}
#[rustfmt::skip]
const INPUT: &[Blueprint] = &[
	// Blueprint { index: 1, ore_robot: 4, clay_robot: 2, obsidian_robot: (3, 14), geode_robot: (2,  7) },
	// Blueprint { index: 2, ore_robot: 2, clay_robot: 3, obsidian_robot: (3,  8), geode_robot: (3, 12) },

	Blueprint { index: 1, ore_robot: 3, clay_robot: 4, obsidian_robot: (3, 18), geode_robot: (4, 19), },
	Blueprint { index: 2, ore_robot: 2, clay_robot: 4, obsidian_robot: (3, 19), geode_robot: (4, 12), },
	Blueprint { index: 3, ore_robot: 4, clay_robot: 4, obsidian_robot: (4, 12), geode_robot: (3,  8), },
	// Blueprint { index: 4, ore_robot: 2, clay_robot: 4, obsidian_robot: (3, 19), geode_robot: (4, 13), },
	// Blueprint { index: 5, ore_robot: 2, clay_robot: 3, obsidian_robot: (3, 17), geode_robot: (3, 10), },
	// Blueprint { index: 6, ore_robot: 3, clay_robot: 3, obsidian_robot: (3, 17), geode_robot: (4,  8), },
	// Blueprint { index: 7, ore_robot: 4, clay_robot: 3, obsidian_robot: (3,  7), geode_robot: (2,  7), },
	// Blueprint { index: 8, ore_robot: 3, clay_robot: 4, obsidian_robot: (4,  6), geode_robot: (3, 16), },
	// Blueprint { index: 9, ore_robot: 3, clay_robot: 3, obsidian_robot: (3, 19), geode_robot: (3, 17), },
	// Blueprint { index: 10F, ore_robot: 4, clay_robot: 4, obsidian_robot: (4,  9), geode_robot: (4, 16), },
	// Blueprint { index: 11, ore_robot: 4, clay_robot: 4, obsidian_robot: (2,  7), geode_robot: (4, 18), },
	// Blueprint { index: 12, ore_robot: 3, clay_robot: 3, obsidian_robot: (3, 20), geode_robot: (2, 12), },
	// Blueprint { index: 13, ore_robot: 4, clay_robot: 4, obsidian_robot: (3,  5), geode_robot: (3, 18), },
	// Blueprint { index: 14, ore_robot: 4, clay_robot: 4, obsidian_robot: (3,  7), geode_robot: (4, 11), },
	// Blueprint { index: 15, ore_robot: 4, clay_robot: 3, obsidian_robot: (2, 14), geode_robot: (2,  7), },
	// Blueprint { index: 16, ore_robot: 4, clay_robot: 4, obsidian_robot: (3,  7), geode_robot: (3, 20), },
	// Blueprint { index: 17, ore_robot: 2, clay_robot: 4, obsidian_robot: (4, 18), geode_robot: (2, 11), },
	// Blueprint { index: 18, ore_robot: 2, clay_robot: 4, obsidian_robot: (4, 17), geode_robot: (3, 11), },
	// Blueprint { index: 19, ore_robot: 2, clay_robot: 4, obsidian_robot: (2, 20), geode_robot: (2, 17), },
	// Blueprint { index: 20, ore_robot: 4, clay_robot: 4, obsidian_robot: (4,  5), geode_robot: (3,  7), },
	// Blueprint { index: 21, ore_robot: 4, clay_robot: 4, obsidian_robot: (4, 15), geode_robot: (4, 17), },
	// Blueprint { index: 22, ore_robot: 4, clay_robot: 3, obsidian_robot: (3, 15), geode_robot: (2, 13), },
	// Blueprint { index: 23, ore_robot: 3, clay_robot: 3, obsidian_robot: (2, 16), geode_robot: (3, 14), },
	// Blueprint { index: 24, ore_robot: 4, clay_robot: 3, obsidian_robot: (4,  5), geode_robot: (3, 10), },
	// Blueprint { index: 25, ore_robot: 3, clay_robot: 3, obsidian_robot: (2, 20), geode_robot: (2, 20), },
	// Blueprint { index: 26, ore_robot: 4, clay_robot: 4, obsidian_robot: (4, 14), geode_robot: (2, 16), },
	// Blueprint { index: 27, ore_robot: 3, clay_robot: 4, obsidian_robot: (4,  8), geode_robot: (2, 10), },
	// Blueprint { index: 28, ore_robot: 3, clay_robot: 3, obsidian_robot: (3,  8), geode_robot: (2, 12), },
	// Blueprint { index: 29, ore_robot: 4, clay_robot: 4, obsidian_robot: (4,  8), geode_robot: (2, 15), },
	// Blueprint { index: 30, ore_robot: 3, clay_robot: 3, obsidian_robot: (2,  7), geode_robot: (2,  9), },
];

const PART1: bool = false;

#[tokio::main]
async fn main() {
	let mut res = 0;
	if !PART1 {
		res = 1;
	}
	let mut tasks = Vec::new();
	for (id, bp) in INPUT.iter().enumerate() {
		let id = id + 1;
		println!("{id}: start");
		let initial_state = State {
			bp,
			time: if PART1 { 24 } else { 32 },
			ore_robots: 1,
			clay_robots: 0,
			obsidian_robots: 0,
			geode_robots: 0,
			ore: 0,
			clay: 0,
			obsidian: 0,
			geodes: 0,
		};
		tasks.push(tokio::task::spawn(async move {
			let best = Box::leak(Box::new(Best::new()));
			initial_state.simulate(best, Stack::new()).await;
			best.print(initial_state);
			let res = if PART1 { best.get() * id } else { best.get() };
			println!("{id}: done {res}");
			res
		}));
	}
	for task in tasks {
		let best = task.await.unwrap();
		if PART1 {
			res += best;
		} else {
			res *= best;
		}
	}
	println!("part {}: {res}", if PART1 { "1" } else { "2" });
}
