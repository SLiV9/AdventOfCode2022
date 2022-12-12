/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> usize
{
	solve(input, false)
}

fn two(input: &str) -> usize
{
	solve(input, true)
}

fn solve(input: &str, is_part_two: bool) -> usize
{
	let num_rows = input.lines().count();
	let num_cols = input.lines().next().unwrap().len();
	let num_cells = num_rows * num_cols;
	let stride = num_cols + 1;
	let mut grid = vec![b'.'; num_rows * stride];
	for r in 0..num_rows
	{
		grid[r * stride + num_cols] = b'\n';
	}
	let data = input.as_bytes();
	let end_offset = data.iter().position(|x| *x == b'E').unwrap();
	grid[end_offset] = b'E';
	let mut stack = vec![end_offset];
	let mut processing = Vec::new();
	for step in 0..num_cells
	{
		std::mem::swap(&mut stack, &mut processing);
		for from in processing.drain(..)
		{
			let current_height = match data[from]
			{
				b'S' =>
				{
					println!("Grid:\n{}", std::str::from_utf8(&grid).unwrap());
					return step;
				}
				b'a' if is_part_two =>
				{
					println!("Grid:\n{}", std::str::from_utf8(&grid).unwrap());
					return step;
				}
				b'E' => b'z',
				h => h,
			};
			let r = from / stride;
			let c = from % stride;
			if c > 0
			{
				let to = r * stride + c - 1;
				if grid[to] == b'.'
					&& (data[to] == b'S' || data[to] + 1 >= current_height)
				{
					grid[to] = b'>';
					stack.push(to);
				}
			}
			if c + 1 < num_cols
			{
				let to = r * stride + c + 1;
				if grid[to] == b'.'
					&& (data[to] == b'S' || data[to] + 1 >= current_height)
				{
					grid[to] = b'<';
					stack.push(to);
				}
			}
			if r > 0
			{
				let to = (r - 1) * stride + c;
				if grid[to] == b'.'
					&& (data[to] == b'S' || data[to] + 1 >= current_height)
				{
					grid[to] = b'v';
					stack.push(to);
				}
			}
			if r + 1 < num_rows
			{
				let to = (r + 1) * stride + c;
				if grid[to] == b'.'
					&& (data[to] == b'S' || data[to] + 1 >= current_height)
				{
					grid[to] = b'^';
					stack.push(to);
				}
			}
		}
		stack.sort();
		stack.dedup();
		assert!(!stack.is_empty());
	}
	unreachable!()
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 31);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 29);
	}
}
