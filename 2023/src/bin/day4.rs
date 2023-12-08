use ahash::AHashSet;
use aoc2023::*;

fn main() {
	part1(|input| {
		let mut sum = 0;
		for i in input.lines() {
			let (winning, card) = i.split_once(": ").unwrap().1.split_once(" | ").unwrap();
			let winning = winning.split(' ').collect::<AHashSet<_>>();
			let nums = card.split(' ').collect::<Vec<_>>();
			let mut mul = 1;
			for i in nums {
				if i.is_empty() {
					continue;
				}
				if winning.contains(i) {
					mul *= 2;
				}
			}
			sum += mul / 2;
		}
		sum
	});
	part2(|input| {
		let cards = input
			.lines()
			.map(|i| {
				let (winning, card) = i.split_once(": ").unwrap().1.split_once(" | ").unwrap();
				let winning = winning.split(' ').collect::<AHashSet<_>>();
				let nums = card.split(' ').collect::<Vec<_>>();
				let mut n_won = 0;
				for i in nums {
					if i.is_empty() {
						continue;
					}
					if winning.contains(i) {
						n_won += 1;
					}
				}
				n_won
			})
			.collect::<Vec<_>>();
		fn process_card(cards: &[usize], card: usize) -> usize {
			let mut out = 1;
			let n_won = cards[card - 1];
			for i in 0..n_won {
				out += process_card(cards, card + i + 1);
			}
			out
		}
		let mut sum = 0;
		for i in 0..cards.len() {
			sum += process_card(&cards, i + 1);
		}
		sum
	});
}
