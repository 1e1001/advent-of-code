use aoc2023::*;

fn main() {
	part1(|input| {
		let hails = input
			.lines()
			.map(|v| {
				let (x, v) = v.split_once(", ").unwrap();
				let (y, v) = v.split_once(", ").unwrap();
				let (z, v) = v.split_once(" @ ").unwrap();
				let (vx, v) = v.split_once(", ").unwrap();
				let (vy, v) = v.split_once(", ").unwrap();
				let vz = v.split_once('\n').map_or(v, |v| v.0);
				let x: isize = x.trim().parse().unwrap();
				let y: isize = y.trim().parse().unwrap();
				let _z: isize = z.trim().parse().unwrap();
				let vx: isize = vx.trim().parse().unwrap();
				let vy: isize = vy.trim().parse().unwrap();
				let _vz: isize = vz.trim().parse().unwrap();
				(x, y, vx, vy)
			})
			.collect::<Vec<_>>();
		let mut count = 0;
		for (i, a) in hails.iter().enumerate() {
			for b in &hails[i + 1..] {
				// https://a.opnxng.com/questions/2931573/determining-if-two-rays-intersect
				let dx = b.0 - a.0;
				let dy = b.1 - a.1;
				let det = b.2 * a.3 - b.3 * a.2;
				if det == 0 {
					continue;
				}
				let u = (dy * b.2 - dx * b.3) as f64 / det as f64;
				let v = (dy * a.2 - dx * a.3) as f64 / det as f64;
				if u < 0.0 || v < 0.0 {
					continue;
				}
				let pos = (a.0 as f64 + a.2 as f64 * u, a.1 as f64 + a.3 as f64 * u);
				const MIN: f64 = 200000000000000.0;
				const MAX: f64 = 400000000000000.0;
				if pos.0 < MIN || pos.1 < MIN || pos.0 > MAX || pos.1 > MAX {
					continue;
				}
				println!("{pos:?} {u:?} {v:?}");
				count += 1;
			}
		}
		count
	});
	part2(|input| {
		// solved externally
		let hails = input
			.lines()
			.map(|v| {
				let (x, v) = v.split_once(", ").unwrap();
				let (y, v) = v.split_once(", ").unwrap();
				let (z, v) = v.split_once(" @ ").unwrap();
				let (vx, v) = v.split_once(", ").unwrap();
				let (vy, v) = v.split_once(", ").unwrap();
				let vz = v.split_once('\n').map_or(v, |v| v.0);
				let x: isize = x.trim().parse().unwrap();
				let y: isize = y.trim().parse().unwrap();
				let z: isize = z.trim().parse().unwrap();
				let vx: isize = vx.trim().parse().unwrap();
				let vy: isize = vy.trim().parse().unwrap();
				let vz: isize = vz.trim().parse().unwrap();
				(x, y, z, vx, vy, vz)
			})
			.collect::<Vec<_>>();
		print!("[");
		for i in hails.iter().take(3) {
			print!("({},{},{},{},{},{}),", i.0, i.1, i.2, i.3, i.4, i.5);
		}
		println!("]");
	});
}
