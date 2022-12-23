/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

// Instead of east and west, use LESS significant and MORE significant.
const PROPOSAL_SEQUENCE_LEN: usize = 4;
const PROPOSAL_SEQUENCE: [u8; PROPOSAL_SEQUENCE_LEN] = [b'N', b'S', b'L', b'M'];

fn one(input: &str) -> usize
{
	let mut grid = Grid::create();
	grid.parse_input(input);
	grid.dbg_print();
	let count = grid.count();
	for round in 1..=10
	{
		println!("Round {}", round);
		grid.expand();
		grid.dbg_print();
		grid.diffuse();
		grid.dbg_print();
		grid.collapse();
		assert_eq!(grid.count(), count);
		if !grid.is_active
		{
			break;
		}
	}
	println!("Done");
	grid.dbg_print();
	grid.count_empty_spaces()
}

fn two(input: &str) -> usize
{
	let mut grid = Grid::create();
	grid.parse_input(input);
	grid.dbg_print();
	let count = grid.count();
	let mut round = 1;
	while grid.is_active
	{
		println!("Round {}", round);
		grid.expand();
		grid.dbg_print();
		grid.diffuse();
		grid.dbg_print();
		grid.collapse();
		assert_eq!(grid.count(), count);
		round += 1;
	}
	println!("Done");
	grid.dbg_print();
	grid.count_empty_spaces()
}

fn propose(
	current: u128,
	above: u128,
	below: u128,
	proposal_sequence: &[u8; PROPOSAL_SEQUENCE_LEN],
) -> Proposal
{
	if current == 0
	{
		return Proposal::default();
	}
	let l = current << 1;
	let m = current >> 1;
	let above3 = above | (above >> 1) | (above << 1);
	let below3 = below | (below >> 1) | (below << 1);
	let happy = current & !l & !m & !above3 & !below3;
	let mut unhappy = current & !happy;
	let mut proposal = Proposal::default();
	for proposed_direction in proposal_sequence.iter().map(|x| *x)
	{
		match proposed_direction
		{
			b'N' =>
			{
				proposal.north = unhappy & !above3;
				unhappy &= !proposal.north;
			}
			b'S' =>
			{
				proposal.south = unhappy & !below3;
				unhappy &= !proposal.south;
			}
			b'L' =>
			{
				let from = unhappy & !l & !(above << 1) & !(below << 1);
				proposal.less = from >> 1;
				unhappy &= !from;
			}
			b'M' =>
			{
				let from = unhappy & !m & !(above >> 1) & !(below >> 1);
				proposal.more = from << 1;
				unhappy &= !from;
			}
			_ => unreachable!(),
		}
	}
	proposal.stay = happy | unhappy;
	proposal
}

#[derive(Debug, Default, Clone)]
struct Proposal
{
	stay: u128,
	north: u128,
	south: u128,
	less: u128,
	more: u128,
}

fn block(a: u128, b: u128, c: u128, d: u128) -> u128
{
	(a & (b | c | d)) | (b & (c | d)) | (c & d)
}

fn resolve_block(blocked: u128, proposed: &mut u128, backup: &mut u128)
{
	let canceled = blocked & *proposed;
	*proposed &= !canceled;
	*backup |= canceled;
}

const MAX_ROWS: usize = 128 + 1;
const MAX_COLS: usize = 128;

struct Grid
{
	data: [u128; MAX_ROWS],
	width: usize,
	height: usize,
	proposal_sequence: [u8; PROPOSAL_SEQUENCE_LEN],
	is_active: bool,
}

impl Grid
{
	fn create() -> Grid
	{
		Grid {
			data: [0u128; MAX_ROWS],
			width: 0,
			height: 0,
			proposal_sequence: PROPOSAL_SEQUENCE,
			is_active: true,
		}
	}

	fn parse_input(&mut self, input: &str)
	{
		for (r, line) in input.lines().enumerate()
		{
			for (c, byte) in line.bytes().enumerate()
			{
				match byte
				{
					b'#' => self.set_at_rc(r, c),
					_ => (),
				}
			}
		}
	}

	fn set_at_rc(&mut self, r: usize, c: usize)
	{
		self.data[r] |= 1u128 << c;
		if r + 1 > self.height
		{
			self.height = r + 1;
		}
		if c + 1 > self.width
		{
			self.width = c + 1;
		}
	}

	fn expand(&mut self)
	{
		// Add an empty row at the bottom and two at the top.
		self.data[0..(self.height + 1)].rotate_left(self.height);
		self.height += 3;
		assert!(self.height <= MAX_ROWS);
		// Add an empty column at the start and the end.
		for row in &mut self.data
		{
			*row <<= 1;
		}
		self.width += 2;
		assert!(self.width <= MAX_COLS);
	}

	fn diffuse(&mut self)
	{
		self.is_active = false;
		assert!(self.height >= 3);
		let mut prev = Proposal::default();
		let mut curr = propose(
			self.data[1],
			self.data[0],
			self.data[2],
			&self.proposal_sequence,
		);
		self.data[0] = curr.north;
		for r_of_prev in 0..(self.height - 3)
		{
			let r_of_current = r_of_prev + 1;
			let r_of_next = r_of_prev + 2;
			let mut next = propose(
				self.data[r_of_next],
				self.data[r_of_next - 1],
				self.data[r_of_next + 1],
				&self.proposal_sequence,
			);
			let blocked = block(prev.south, curr.less, curr.more, next.north);
			resolve_block(blocked, &mut prev.south, &mut self.data[r_of_prev]);
			resolve_block(blocked, &mut curr.less, &mut curr.stay);
			resolve_block(blocked, &mut curr.more, &mut curr.stay);
			resolve_block(blocked, &mut next.north, &mut next.stay);
			let arrived = prev.south | curr.less | curr.more | next.north;
			if arrived != 0
			{
				self.is_active = true;
			}
			self.data[r_of_current] = curr.stay | arrived;
			prev = curr;
			curr = next;
		}
		self.data[self.height - 2] |= prev.south;
		assert_eq!(curr.less, 0);
		assert_eq!(curr.stay, 0);
		assert_eq!(curr.more, 0);
		self.proposal_sequence.rotate_left(1);
	}

	fn collapse(&mut self)
	{
		// Trim empty rows.
		let start = self.data.iter().position(|&row| row != 0).unwrap();
		let end = self.data.iter().rposition(|&row| row != 0).unwrap() + 1;
		self.data[0..end].rotate_left(start);
		self.height = end - start;
		// Trim empty columns.
		let mask = self.data[0..self.height].iter().fold(0, |a, row| (a | row));
		let n = mask.trailing_zeros();
		for row in &mut self.data
		{
			*row >>= n;
		}
		self.width = MAX_COLS - n as usize - mask.leading_zeros() as usize;
	}

	fn count(&self) -> usize
	{
		self.data[0..self.height]
			.iter()
			.map(|word| word.count_ones() as usize)
			.sum()
	}

	fn count_empty_spaces(&self) -> usize
	{
		self.data[0..self.height]
			.iter()
			.map(|word| self.width - word.count_ones() as usize)
			.sum()
	}

	#[allow(unused)]
	fn dbg_print(&self)
	{
		println!();
		for row in &self.data[0..self.height]
		{
			print!("| ");
			let mut mask = *row;
			for c in 0..self.width
			{
				if mask & 0x01 != 0
				{
					print!("#");
				}
				else
				{
					print!(".");
				}
				mask >>= 1;
			}
			println!();
		}
		println!();
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const MINI: &str = include_str!("mini.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 110);
	}

	#[test]
	fn one_mini()
	{
		assert_eq!(one(MINI), 25);
	}
}
