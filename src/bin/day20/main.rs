/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i16
{
	let numbers = input.lines().map(|line| line.parse().unwrap());
	let commands = input.lines().map(|line| line.parse().unwrap());
	let mut data: Vec<i16> = numbers.collect();
	let n = data.len();
	for value in commands
	{
		let i = data.iter().position(|v| *v == value).unwrap();
		if value > 0
		{
			let shift: usize = usize::try_from(value).unwrap() % n;
			if i + shift < n
			{
				let from = i;
				let to = i + shift;
				data[from..=to].rotate_left(1);
			}
			else
			{
				let from = (i + shift + 1) % n;
				let to = i;
				data[from..=to].rotate_right(1);
			}
		}
		else if value < 0
		{
			let shift: usize = usize::try_from(-value).unwrap() % n;
			if i >= shift
			{
				let from = i - shift;
				let to = i;
				data[from..=to].rotate_right(1);
			}
			else
			{
				let from = i;
				let to = n + i - shift - 1;
				assert!(to < n);
				data[from..=to].rotate_left(1);
			}
		}
	}
	let start = data.iter().position(|v| *v == 0).unwrap();
	let x = data[(start + 1000) % n];
	let y = data[(start + 2000) % n];
	let z = data[(start + 3000) % n];
	x + y + z
}

fn two(_input: &str) -> i32
{
	0
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
		assert_eq!(one(PROVIDED), 3);
	}
}
