use ahash::AHashSet;
use aoc2023::*;

fn main() {
	part1(|input| {
		input
			.lines()
			.enumerate()
			.map(|(index, v)| {
				let (nums, vals) = v.split_once(' ').unwrap();
				let nums = nums
					.chars()
					.map(|v| match v {
						'#' => Some(true),
						'.' => Some(false),
						'?' => None,
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>();
				let vals = vals
					.split(',')
					.map(|v| v.parse().unwrap())
					.collect::<Vec<usize>>();
				fn set_nums(v: &[Option<bool>], mut c: usize) -> Vec<bool> {
					v.iter()
						.rev()
						.map(|v| match v {
							Some(v) => *v,
							None => {
								let res = (c & 1) == 1;
								c >>= 1;
								res
							}
						})
						.collect::<Vec<_>>()
						.into_iter()
						.rev()
						.collect()
				}
				let num_opts = nums.iter().filter(|v| v.is_none()).count();
				let mut combs = 0;
				for i in 0..2usize.pow(num_opts as u32) {
					let vec = set_nums(&nums, i);
					let mut spans = Vec::new();
					let mut prev = 0;
					for v in vec.into_iter().chain([false]) {
						if v {
							prev += 1;
						} else if prev > 0 {
							spans.push(prev);
							prev = 0;
						}
					}
					if spans == vals {
						combs += 1;
					}
				}
				println!("line {index} = {combs}");
				combs
			})
			.sum::<usize>()
	});
	part2(|input| {
		input
			.lines()
			.enumerate()
			// .filter(|(i, _)| *i == 11)
			.map(|(index, v)| {
				let (nums, vals) = v.split_once(' ').unwrap();
				let start_nums = nums
					.chars()
					.map(|v| match v {
						'#' => Some(true),
						'.' => Some(false),
						'?' => None,
						_ => panic!("shit {v:?}"),
					})
					.collect::<Vec<_>>();
				let start_vals = vals
					.split(',')
					.map(|v| v.parse().unwrap())
					.collect::<Vec<usize>>();
				let mut nums = Vec::new();
				let mut vals = Vec::new();
				for _ in 0..5 {
					nums.extend_from_slice(&start_nums);
					nums.push(None);
				}
				nums.pop();
				for _ in 0..5 {
					vals.extend_from_slice(&start_vals);
				}
				// nums = start_nums;
				// vals = start_vals;
				#[derive(Debug, Clone, Copy, Eq)]
				struct Runner {
					nums_index: usize,
					vals_index: usize,
					span_depth: usize,
					counter: usize,
				}

				impl std::hash::Hash for Runner {
					fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
						self.nums_index.hash(state);
						self.vals_index.hash(state);
						self.span_depth.hash(state);
					}
				}

				impl PartialEq for Runner {
					fn eq(&self, other: &Self) -> bool {
						self.nums_index == other.nums_index
							&& self.vals_index == other.vals_index
							&& self.span_depth == other.span_depth
					}
				}
				impl Runner {
					fn advance(
						self,
						nums: &mut [Option<bool>],
						vals: &[usize],
						out: &mut impl FnMut(Runner),
						comb: &mut usize,
					) {
						if self.vals_index >= vals.len() {
							if nums[self.nums_index..].iter().all(|v| *v != Some(true)) {
								*comb += self.counter;
							}
							return;
						}
						if self.nums_index >= nums.len() {
							return;
						}
						// println!(
						// 	"adv {self:?} {:?} {:?}",
						// 	nums[self.nums_index], vals[self.vals_index]
						// );
						match nums[self.nums_index] {
							Some(true) => {
								if self.span_depth < vals[self.vals_index] {
									out(Self {
										nums_index: self.nums_index + 1,
										vals_index: self.vals_index,
										span_depth: self.span_depth + 1,
										counter: self.counter,
									});
								}
							}
							Some(false) => {
								if self.span_depth == 0 {
									out(Self {
										nums_index: self.nums_index + 1,
										vals_index: self.vals_index,
										span_depth: 0,
										counter: self.counter,
									});
								} else if self.span_depth == vals[self.vals_index] {
									out(Self {
										nums_index: self.nums_index + 1,
										vals_index: self.vals_index + 1,
										span_depth: 0,
										counter: self.counter,
									});
								}
							}
							None => {
								nums[self.nums_index] = Some(true);
								self.advance(nums, vals, out, comb);
								nums[self.nums_index] = Some(false);
								self.advance(nums, vals, out, comb);
								nums[self.nums_index] = None;
							}
						}
					}
				}
				let mut combs = 0;
				// println!("line {} {}", nums.len(), vals.len());
				let mut runners = AHashSet::new();
				runners.insert(Runner {
					nums_index: 0,
					vals_index: 0,
					span_depth: 0,
					counter: 1,
				});
				let mut iter = 0;
				nums.push(Some(false));
				while runners.len() > 0 {
					// println!(
					// 	"iter {iter} {} {:?} {combs}",
					// 	runners.iter().map(|v| v.counter).sum::<usize>(),
					// 	runners
					// 		.iter()
					// 		.map(|v| (v.nums_index, v.vals_index, v.span_depth, v.counter))
					// 		.collect::<Vec<_>>()
					// );
					let _ = iter;
					let mut runners_out: AHashSet<Runner> = AHashSet::new();
					for r in runners {
						// println!();
						r.advance(
							&mut nums,
							&vals,
							&mut |mut runner| {
								// println!("out {runner:?}");
								if let Some(old) = runners_out.get(&runner) {
									runner.counter += old.counter;
									runners_out.remove(&runner);
								}
								runners_out.insert(runner);
							},
							&mut combs,
						);
					}
					runners = runners_out;
					iter += 1;
				}
				// println!("iter {iter} done {combs}");
				println!("line {index} = {combs}");
				combs
			})
			.sum::<usize>()
	});
}
