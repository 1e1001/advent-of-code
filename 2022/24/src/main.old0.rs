use std::collections::HashSet;
use std::future::{self, Future};
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Mutex;

use async_recursion::async_recursion;
use tokio::task::JoinError;

//*/
const INPUT: &str = r##"
<<^<<v.v^>v.<v>.^^>vvv<..^><>vvv>v>.v<>^>vv<<^><<^.<v^^<<<<<v^v><v>v^>vv>>.<^>.>^^^.<><><^^<v^><><.<
<<^<<vv^vv<<^<><>>^<^vvv><>>>^.<v^<..<.>v>v>vvv><.v<^>v^v<<<>>>>>vv<^<<<v>^<.>>vv><<.>v<.^v.v<^<>>v>
<<.v>.v<v<<^v.v>><v.<v<^.>>>.<^>^^^><<>.v><<vv.^v^.^<v>v^vv^<.>v<v><.v^.v<v<v>>><^v.v^^^<<v<^^<>v>v>
<>^v>vvv><..^..v<<^v^><.<<<<>>>^v><.<<^^vv<^v.<^^<^v^>>vv<.>v><..^><.vv>v^>>v><^v<<<^v<^v<v<<^v.vv^>
>^v<>>v<^^vv<<<v^v^<<v>v^vv^vv>><>^.^vv^<>^<v^^<.^v<.^<>^>vv.><<^>v<^^>v^<v.^.^<>v>^>v.<^vvvv<>>.>v>
<^^>><v><<^v<v>vvv>v^>v<^^^^><>^v^.<v^>.<>>vvv^v>.<v<<<^v>>>><v^^<>^v<>>^v>^^^^vvv^^><>^^^^<v<..><^>
>><<<><>^v^<<v>>.<v^^^^<vv^>>>v<v><v>>^>>^^^>v.<v^v>><><^v><vv<^.^vvvv<>.^>v><<<^>vv<^^v<v^^<><.^^v.
<>>^^>v<v.>v.v>^<<>^.>>vv^vv<^>v^><.^v<<v^<.^>v^.^v.<^.^<^<..^>><>v.>^v>vvvv<<>v^<vv<>vv>v>.>v^<^^^>
>vv<v>v<v>^<>v^<^vv<>v>^><><<>>v^<>v.<.>>>>v<v>v^<v<^v<>>v^<>v^>v>v<^<^v^^<v.<^vv<<<.>^v^><v>^^<.<>>
>v^.>vv^vv>.><<^.v^.v<>.^>v><<>v^^<<^<.^.^vv^<^^^><<>.v<<v<v<^^>v<<^<vv^^>>vvv>vvv<^vv.^.v>.<v.><^v>
<><>^v^^<.vv<v>^v<<^v>^.<v>^<.>.>^.v<<^vv.vv<v^<v<^v.>^^^<^^><^vv<<<>v<>><vv.<.<<vv<v.<<.>^>>>^>^.^>
<^><v<><v>v^^<^<vv>^^>^v^>v^>^>><v.>^<.<v<>.<^>>^<<^v^v^.><>v><<^^^vvv<><^>^^v.>><>>v<<^.v<v>^<>v^<<
>.><<<^<v<v>vv>v><.<v>.^^<<<<<><v>^.><^>vvv>v^<^<^v^<><>>.<v<v^v><^.vv^^>^vv>v^<v^<<<>^.v^<vv<v^v>.>
>.<>^><v><.^>vv.^.>v<<v^<^<><>...<>.>^<^><<>v>^>^<.^><><v<>v>>.<^v^<.<vv^v>.v^<v>vv>v>>>^.<^<v^<<..<
>^<<><<^>vvv>>>>v<<^v^>><>..^>v>>^<v>>v^^^.>.<>.^<^<v^>>v><.v>^v<v>v<^.^<^v<.v>^v>v>^>>^^v>>>>^^^^<<
>><^^v>vv>v<^<><v>vv^>^^^v^<^<><.<<^^>><<>v><><<>^<^>v^v^v<><v.><v>>..v^><v>><v^<^vvv>v.^^<<><>v.<><
>^^vv<<>>^^^>v<<<<v>.v^>v>v>vvv<^^<<v<.^^<<^^v<<><<>v.vv.^>^>><^><<vv>^.<>>v>^^v>^vv>.^^v>^.vvv>><<<
><^^^>vv.<^>^<><vv<><^^v<v.>.vv><^<.^^.v<vv<v<<<^<vvv.^^v^>^v.>^v<<<^vv><<v..^<v<^>>.^^^>v<><^vv^<v<
<>^^.>><v>v>^^.v<>.^vv^>.<^<vv<<v<v<v>.<^v>.<><v><^<^^>v>>^>.^>>^><.<><^<<v^v^<>vv<<<>>>^>^<vv^v<^^<
<>v^v><<v<v<.v><vv^v>><>><^.>>^vv>^.<v^^<^><>v<v^<v>vv>^v>vv<<v^>>>^>>>><>vvvv<^>v>^^>>^>^^^..^<v^<>
>v^<^^>v^v^<^>.<vv.<^>>>v.v^^<<^^<.<^.v<<^>v.>^v<.vvv^<vv<><>.v^>>vv<<<v>vvv>^<>^<vvv><v^vv<^^v>^<^<
<v^^v><.>>vv^.>>>^^><.v><<>^^><vv<><>v^v<<v^<v><.vv><<>>^^^>vvvv.^<<>vv.<.v.v<><><>.<><^^>v>v>v.^^<<
<>v^^<v^^>^>^<<<><<<^^>v>^><^v^>.<^^.<<.<v>><..^<v^^><^<<<.<.vvv.<<^v^<<<>v^.<^><>>.^>v<>^v><.^^>^><
><<^^^vvv^^>v^v^>v<v.v^<<<v^><>v><<vvv.v<>vv<<^^^><v^vv.>^<^><^>>v>.^<^^v^<<^><<^.^>vvv>>>.<><^v^^<<
<>>vv.^^^<^^>v<<>^>>v^<v.vv<vv^.<<v><v>v^>>>^^<.><^v^vvv..><><>v.^.v<vv><>v>><><>v^<<^.^^^v<^>>^<^^>
<>>.^>>v<>>>^<v>.>^><>^^>^vv>vv^><.<<<<<^<>.^.v^vv><v<vv>vv^^<><^.^>>v^<><><^v<v.>^^^>^v^^<v^><^<..>
><..>vv<<>vv^vv^>>.^^>^v><^^v<<..<^>.>^<vv^v>>><v^.>v^<>..<^<<<^>v^v<>v><vvv^^v^<>vv^<<<v<<vv<>>v^v>
>v<^>v<>v.v<>^vv.^<>v>>^v>.^^^>^<v^^>v>^><..v>v.>vvv^.^<v><^^v.^^<^vv..v^^^.>><>vvv>^vv^v<>>v^>v^v^.
>v><<^<.<^<<v.^^v^^^>>vv<^<^^.^^^v.<<.^>^>.<vv<v<^.>.^v^^v<><><^<^<^>v<><>^^<<<<>^..>v>><>^.v^.<>^^>
>>^^^^^v.<<>^^v>^vv.^<.<v^.^.^^>v<^^v<>v<^.>v<<<^>v^vv^><>>^^.><<>.v<^v^<>>^<>^v>v>>^^.<v<>v>.>vv^<.
<<^v>^^>^>.<^<>vvv^^v>v<<>v<.><^<<v><v<.v>v^>>>^v<vv>>>>>v^>.^^^>^.<<>vv<^v<>v>v^vv>v><.v.>.v><><.><
>v.^>v^^<>>v^.>^<<>><vv><<v<^>^<>.^<>>^^>^^>.><^>vv<<<v><^<<vv>^.^<v<<>v^.<<<.<>^<^v<^^<^>v>><.^>.<<
><<<<.^.vvvv^^<v^^^...<>v<<<v^^<v<.>>>><<^^<>^vv<<<v^^><.<^><<.<>vv<><>vv^>>vvv^^.><^<<v>^^^^.<v>>>>
<^.>>^vv<v<<v>>^>v.>^.vv^.>^<^v^.^<..><<.vvv>>vv>.^v^.>^<vv^vv^<^>>^>^>><<<^vv.^.>^>><v>v^v><^<>vv^>
..v^>..>v....v<vv^<^.^>v.<.>^<<vv^^^>v^<<>v^v^>^<<>v^^^>.<^>^<<.v.v<<<^>>><^<^<<>.^^^<<<><^^^vv<vv.<"##;
const GRID_W: isize = 100;
const GRID_H: isize = 35;
// */
/*/
const INPUT: &str = r##">>.<^<
.<..<<
>v.><>
<^v^^>"##;
const GRID_W: isize = 6;
const GRID_H: isize = 4;
// */
const GOAL_X: isize = GRID_W - 1;
const GOAL_Y: isize = GRID_H;

struct GlobalData {
	blizzards_u: HashSet<(isize, isize)>,
	blizzards_d: HashSet<(isize, isize)>,
	blizzards_l: HashSet<(isize, isize)>,
	blizzards_r: HashSet<(isize, isize)>,
}
static CURRENT_TASKS: Mutex<usize> = Mutex::new(0);
const MAX_TASKS: usize = 10000000;

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

#[derive(Clone, Copy)]
struct State {
	data: &'static GlobalData,
	time: isize,
	pos: (isize, isize),
}

impl State {
	fn best_possible(&self) -> isize {
		self.time + (GRID_W - self.pos.0 - 1) + (GRID_H - self.pos.1)
	}
	fn next_states(&self) -> [Self; 5] {
		let mut res = [Self {
			data: self.data,
			time: self.time + 1,
			pos: self.pos,
		}; 5];
		res[0].pos.0 += 1;
		res[1].pos.1 += 1;
		res[2].pos.0 -= 1;
		res[3].pos.1 -= 1;
		res
	}
	fn is_valid(&self, best: &'static AtomicIsize) -> bool {
		//println!("simulate {:?} {}", self.pos, self.time);
		if self.best_possible() > best.load(Ordering::Relaxed) {
			return false;
		}
		if self.pos.1 == GOAL_Y && self.pos.0 == GOAL_X {
			let b = best.fetch_min(self.time, Ordering::SeqCst);
			if self.time < b {
				println!("new best: {}", self.time);
			}
			return false;
		} else if self.pos.1 == -1 {
			if self.pos.0 != 0 {
				return false;
			}
		} else if self.pos.0 < 0 || self.pos.0 >= GRID_W || self.pos.1 < 0 || self.pos.1 >= GRID_H {
			return false;
		}
		if self
			.data
			.blizzards_u
			.contains(&(self.pos.0, (self.pos.1 + self.time).rem_euclid(GRID_H)))
			|| self
				.data
				.blizzards_d
				.contains(&(self.pos.0, (self.pos.1 - self.time).rem_euclid(GRID_H)))
			|| self
				.data
				.blizzards_l
				.contains(&((self.pos.0 + self.time).rem_euclid(GRID_W), self.pos.1))
			|| self
				.data
				.blizzards_r
				.contains(&((self.pos.0 - self.time).rem_euclid(GRID_W), self.pos.1))
		{
			// cannot be in this state
			return false;
		}
		true
	}
	#[async_recursion]
	async fn simulate(self, best: &'static AtomicIsize) {
		let mut tasks = Vec::with_capacity(5);
		for state in self.next_states() {
			if state.is_valid(best) {
				tasks.push(capped_future(state.simulate(best)).await);
			}
		}
		for task in tasks {
			task.await.unwrap();
		}
	}
}
#[tokio::main]
async fn main() {
	let mut data = Box::new(GlobalData {
		blizzards_u: HashSet::new(),
		blizzards_d: HashSet::new(),
		blizzards_l: HashSet::new(),
		blizzards_r: HashSet::new(),
	});
	for (y, line) in INPUT.lines().enumerate() {
		for (x, ch) in line.chars().enumerate() {
			match ch {
				'.' => {}
				'<' => {
					data.blizzards_l.insert((x as isize, y as isize));
				}
				'>' => {
					data.blizzards_r.insert((x as isize, y as isize));
				}
				'v' => {
					data.blizzards_d.insert((x as isize, y as isize));
				}
				'^' => {
					data.blizzards_u.insert((x as isize, y as isize));
				}
				_ => panic!("shit"),
			}
		}
	}
	State {
		data: Box::leak(data),
		time: 0,
		pos: (0, -1),
	}
	.simulate(Box::leak(Box::new(AtomicIsize::new(99999))))
	.await;
}
