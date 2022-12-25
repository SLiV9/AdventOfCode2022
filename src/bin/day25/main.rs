/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
}

fn one(input: &str) -> Snafu
{
	input.lines().map(|line| line.parse().unwrap()).sum()
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Snafu(i64);

impl std::ops::Add<Snafu> for Snafu
{
	type Output = Snafu;

	fn add(self, rhs: Snafu) -> Snafu
	{
		Snafu(self.0 + rhs.0)
	}
}

impl std::iter::Sum for Snafu
{
	fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Snafu
	{
		Snafu(iter.map(|x| x.0).sum())
	}
}

impl std::fmt::Debug for Snafu
{
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> Result<(), std::fmt::Error>
	{
		write!(f, "{} ({})", self, self.0)
	}
}

impl std::fmt::Display for Snafu
{
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> Result<(), std::fmt::Error>
	{
		let mut value = self.0;
		let mut num_digits = 1;
		let mut scale = 1;
		let mut max = 2;
		let mut bonus = 0;
		while value.abs() > max
		{
			num_digits += 1;
			scale *= 5;
			bonus = max;
			max += 2 * scale;
		}
		for place in (0..num_digits).rev()
		{
			let digit_value = (value + bonus + 3 * scale) / scale - 3;
			match digit_value
			{
				-2 => write!(f, "=")?,
				-1 => write!(f, "-")?,
				_ => write!(f, "{}", digit_value)?,
			}
			value -= digit_value * scale;
			if place > 0
			{
				scale /= 5;
				bonus -= 2 * scale;
			}
		}
		debug_assert_eq!(value, 0);
		debug_assert_eq!(scale, 1);
		debug_assert_eq!(bonus, 0);
		Ok(())
	}
}

impl std::str::FromStr for Snafu
{
	type Err = std::io::Error;

	fn from_str(input: &str) -> Result<Snafu, std::io::Error>
	{
		let mut value = 0;
		for digit in input.chars()
		{
			let digit_value = match digit
			{
				'2' => 2,
				'1' => 1,
				'0' => 0,
				'-' => -1,
				'=' => -2,
				_ => unreachable!(),
			};
			value = value * 5 + digit_value;
		}
		Ok(Snafu(value))
	}
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
		let snafu = one(PROVIDED);
		assert_eq!(snafu.0, 4890);
		assert_eq!(snafu.to_string(), "2=-1=0");
	}

	#[test]
	fn parse()
	{
		let text = "2=-01";
		let x: Snafu = text.parse().unwrap();
		assert_eq!(x.0, 976);
		assert_eq!(x.to_string(), text);
	}

	#[test]
	fn conversions()
	{
		let x = Snafu(1);
		let text = "1";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(2);
		let text = "2";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(3);
		let text = "1=";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(4);
		let text = "1-";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(5);
		let text = "10";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(6);
		let text = "11";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(7);
		let text = "12";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(8);
		let text = "2=";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(9);
		let text = "2-";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(10);
		let text = "20";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(15);
		let text = "1=0";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(20);
		let text = "1-0";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(2022);
		let text = "1=11-2";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(12345);
		let text = "1-0---0";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
		let x = Snafu(314159265);
		let text = "1121-1110-1=0";
		assert_eq!(x.to_string(), text);
		let y: Snafu = text.parse().unwrap();
		assert_eq!(y, x);
	}
}
