use aoc2023::*;

const HAND_CHARS: &str = "AKQT98765432J";

fn hand_types(hand: &str) -> usize {
	let mut hand_types = hand.chars().collect::<Vec<_>>();
	hand_types.sort();
	let mut hand_types = hand_types
		.into_iter()
		.chain(['!'])
		.fold((Vec::new(), '!', 0), |(mut v, p, r), c| {
			if c == p {
				(v, p, r + 1)
			} else {
				if r > 0 {
					v.push(r);
				}
				(v, c, 1)
			}
		})
		.0;
	hand_types.sort();
	match &*hand_types {
		[1, 1, 1, 1, 1] => 0,
		[1, 1, 1, 2] => 1,
		[1, 2, 2] => 2,
		[1, 1, 3] => 3,
		[2, 3] => 4,
		[1, 4] => 5,
		[5] => 6,
		_ => panic!("shit {hand_types:?}"),
	}
}

fn main() {
	part1(|input| {
		let mut hands = input
			.lines()
			.map(|v| {
				let (hand, bid) = v.split_once(' ').unwrap();
				let bid: usize = bid.parse().unwrap();
				fn card_strength(c: u8) -> usize {
					match c {
						b'A' => 13,
						b'K' => 12,
						b'Q' => 11,
						b'J' => 10,
						b'T' => 9,
						b'9' => 8,
						b'8' => 7,
						b'7' => 6,
						b'6' => 5,
						b'5' => 4,
						b'4' => 3,
						b'3' => 2,
						b'2' => 1,
						_ => panic!("shit {c}"),
					}
				}
				let hand_types = hand_types(hand);
				let hand = hand.as_bytes();
				let hand = hand_types << 20
					| card_strength(hand[0]) << 16
					| card_strength(hand[1]) << 12
					| card_strength(hand[2]) << 8
					| card_strength(hand[3]) << 4
					| card_strength(hand[4]);
				(hand, bid)
			})
			.collect::<Vec<_>>();
		hands.sort();
		hands
			.into_iter()
			.enumerate()
			.map(|(a, (_, b))| (a + 1) * b)
			.sum::<usize>()
	});
	part2(|input| {
		let mut hands = input
			.lines()
			.map(|v| {
				let (hand, bid) = v.split_once(' ').unwrap();
				let bid: usize = bid.parse().unwrap();
				fn card_strength(c: u8) -> usize {
					match c {
						b'A' => 13,
						b'K' => 12,
						b'Q' => 11,
						b'T' => 9,
						b'9' => 8,
						b'8' => 7,
						b'7' => 6,
						b'6' => 5,
						b'5' => 4,
						b'4' => 3,
						b'3' => 2,
						b'2' => 1,
						b'J' => 0,
						_ => panic!("shit {c}"),
					}
				}
				let hand_types = HAND_CHARS
					.chars()
					.map(|v| hand_types(&hand.replace('J', &format!("{v}"))))
					.max()
					.unwrap();
				let hand = hand.as_bytes();
				let hand = hand_types << 20
					| card_strength(hand[0]) << 16
					| card_strength(hand[1]) << 12
					| card_strength(hand[2]) << 8
					| card_strength(hand[3]) << 4
					| card_strength(hand[4]);
				(hand, bid)
			})
			.collect::<Vec<_>>();
		hands.sort();
		hands
			.into_iter()
			.enumerate()
			.map(|(a, (_, b))| (a + 1) * b)
			.sum::<usize>()
	});
}
