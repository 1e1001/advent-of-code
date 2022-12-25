use std::collections::{HashMap, HashSet};

//*
const INPUT: &str = r##"<<^<<v.v^>v.<v>.^^>vvv<..^><>vvv>v>.v<>^>vv<<^><<^.<v^^<<<<<v^v><v>v^>vv>>.<^>.>^^^.<><><^^<v^><><.<
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
/*
const INPUT: &str = r##">>.<^<
.<..<<
>v.><>
<^v^^>"##;
const GRID_W: isize = 6;
const GRID_H: isize = 4;
// */
const GOAL_X: isize = GRID_W - 1;
const GOAL_Y: isize = GRID_H;

#[derive(Debug, Clone, Copy)]
enum Path {
	Up,
	Down,
	Left,
	Right,
	Wait,
}
impl Path {
	fn apply(&self, pos: (isize, isize)) -> (isize, isize) {
		match self {
			Path::Up => (pos.0, pos.1 - 1),
			Path::Down => (pos.0, pos.1 + 1),
			Path::Left => (pos.0 - 1, pos.1),
			Path::Right => (pos.0 + 1, pos.1),
			Path::Wait => (pos.0, pos.1),
		}
	}
}

struct Blizzards {
	u: HashSet<(isize, isize)>,
	d: HashSet<(isize, isize)>,
	l: HashSet<(isize, isize)>,
	r: HashSet<(isize, isize)>,
}

fn is_valid(data: &Blizzards, pos: (isize, isize)) -> bool {
	if pos.1 == GOAL_Y && pos.0 == GOAL_X {
		return true;
	} else if pos.1 == -1 && pos.0 == 0 {
		return true;
	} else if pos.0 < 0 || pos.0 >= GRID_W || pos.1 < 0 || pos.1 >= GRID_H {
		return false;
	}
	if data.u.contains(&pos)
		|| data.d.contains(&pos)
		|| data.l.contains(&pos)
		|| data.r.contains(&pos)
	{
		return false;
	}
	true
}
fn draw_grid(data: &Blizzards, pos: &HashMap<(isize, isize), Vec<Path>>) {
	for y in -1..=GRID_H {
		for x in -1..=GRID_W {
			if pos.contains_key(&(x, y)) {
				print!("?");
			} else if x < 0 || y < 0 || x >= GRID_W || y >= GRID_H {
				print!("#");
			} else {
				print!(" ");
				// let mut c = 0;
				// let mut d = 0;
				// if data.u.contains(&(x, y)) {
				// 	d = 3;
				// 	c += 1;
				// }
				// if data.d.contains(&(x, y)) {
				// 	d = 1;
				// 	c += 1;
				// }
				// if data.l.contains(&(x, y)) {
				// 	d = 2;
				// 	c += 1;
				// }
				// if data.r.contains(&(x, y)) {
				// 	d = 0;
				// 	c += 1;
				// }
				// match c {
				// 	0 => print!(" "),
				// 	1 => match d {
				// 		0 => print!(">"),
				// 		1 => print!("v"),
				// 		2 => print!("<"),
				// 		3 => print!("^"),
				// 		_ => unreachable!(),
				// 	},
				// 	_ => print!("{c}"),
				// }
			}
		}
		println!();
	}
}
fn next_pos(pos: (isize, isize)) -> [((isize, isize), Path); 5] {
	[Path::Wait, Path::Up, Path::Left, Path::Down, Path::Right].map(|v| (v.apply(pos), v))
}

fn main() {
	let mut data = Blizzards {
		u: HashSet::new(),
		d: HashSet::new(),
		l: HashSet::new(),
		r: HashSet::new(),
	};
	for (y, line) in INPUT.lines().enumerate() {
		for (x, ch) in line.chars().enumerate() {
			match ch {
				'.' => {}
				'<' => {
					data.l.insert((x as isize, y as isize));
				}
				'>' => {
					data.r.insert((x as isize, y as isize));
				}
				'v' => {
					data.d.insert((x as isize, y as isize));
				}
				'^' => {
					data.u.insert((x as isize, y as isize));
				}
				_ => panic!("shit"),
			}
		}
	}
	let time = 0;
	let time = scan(&mut data, (0, -1), (GOAL_X, GOAL_Y), time);
	println!("part 1: {time}");
	let time = scan(&mut data, (GOAL_X, GOAL_Y), (0, -1), time);
	let time = scan(&mut data, (0, -1), (GOAL_X, GOAL_Y), time);
	println!("part 2: {time}");
}
fn scan(
	data: &mut Blizzards,
	start_pos: (isize, isize),
	end_pos: (isize, isize),
	start_time: isize,
) -> isize {
	let mut visited_positions = HashMap::new();
	visited_positions.insert(start_pos, Vec::new());
	//draw_grid(&data, &visited_positions);
	let mut done = false;
	for time in start_time + 1.. {
		let mut u = HashSet::new();
		let mut d = HashSet::new();
		let mut l = HashSet::new();
		let mut r = HashSet::new();
		for (x, y) in &data.u {
			u.insert((*x, if *y == 0 { GRID_H - 1 } else { y - 1 }));
		}
		for (x, y) in &data.d {
			d.insert((*x, if *y == GRID_H - 1 { 0 } else { y + 1 }));
		}
		for (x, y) in &data.l {
			l.insert((if *x == 0 { GRID_W - 1 } else { x - 1 }, *y));
		}
		for (x, y) in &data.r {
			r.insert((if *x == GRID_W - 1 { 0 } else { x + 1 }, *y));
		}
		*data = Blizzards { u, d, l, r };
		//println!("time = {time}");
		let mut new_positions = HashMap::new();
		for (pos, path) in visited_positions {
			for (new_pos, new_path) in next_pos(pos) {
				if is_valid(&data, new_pos) {
					let mut path = path.clone();
					path.push(new_path);
					if new_pos.1 == end_pos.1 {
						//println!("goal!");
						//println!("path: {path:?}");
						done = true;
					}
					new_positions.insert(new_pos, path);
				}
			}
		}
		// draw_grid(&data, &new_positions);
		visited_positions = new_positions;
		if done {
			// println!("{visited_positions:?}");
			return time;
		}
	}
	unreachable!();
}
