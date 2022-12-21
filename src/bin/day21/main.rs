/**/

use std::collections::HashMap;
use std::collections::VecDeque;

use vec_drain_where::VecDrainWhereExt;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i64
{
	let monkeys = input.lines().map(|line| line.parse().unwrap());
	let mut computation = Computation::default();
	for monkey in monkeys
	{
		computation.handle(monkey);
		computation.resolve();
	}
	//dbg!(&computation);
	computation.answer()
}

fn two(_input: &str) -> i32
{
	0
}

#[derive(Debug, Default)]
struct Computation
{
	answers: HashMap<Name, i64>,
	waiting_for_two: Vec<WaitingMonkey>,
	waiting_for_one: Vec<WaitingMonkey>,
	resolved_names: VecDeque<Name>,
}

impl Computation
{
	fn handle(&mut self, monkey: Monkey)
	{
		match monkey.expr
		{
			Expression::Null => unreachable!(),
			Expression::Constant(answer) => self.store(monkey.name, answer),
			Expression::Operation { op, left, right } =>
			{
				let result = self.query(left, right);
				match result
				{
					QueryResult::Resolved { left, right } =>
					{
						let answer = calculate(op, left, right);
						self.store(monkey.name, answer);
					}
					QueryResult::WaitingForOne =>
					{
						let monkey = WaitingMonkey {
							name: monkey.name,
							op,
							left,
							right,
						};
						self.waiting_for_one.push(monkey);
					}
					QueryResult::WaitingForTwo =>
					{
						let monkey = WaitingMonkey {
							name: monkey.name,
							op,
							left,
							right,
						};
						self.waiting_for_two.push(monkey);
					}
				}
			}
		}
	}

	fn resolve(&mut self)
	{
		while let Some(resolved) = self.resolved_names.pop_front()
		{
			let finished: Vec<WaitingMonkey> = self
				.waiting_for_one
				.e_drain_where(|x| x.is_waiting_for(resolved))
				.collect();
			for waiting in finished
			{
				match self.query(waiting.left, waiting.right)
				{
					QueryResult::Resolved { left, right } =>
					{
						let answer = calculate(waiting.op, left, right);
						self.store(waiting.name, answer);
					}
					_ => unreachable!(),
				};
			}

			self.waiting_for_one
				.extend(self.waiting_for_two.e_drain_where(|waiting| {
					waiting.left == resolved || waiting.right == resolved
				}));
		}
	}

	fn store(&mut self, name: Name, answer: i64)
	{
		self.answers.insert(name.into(), answer);
		self.resolved_names.push_back(name);
	}

	fn query(&self, left: Name, right: Name) -> QueryResult
	{
		let left = self.answers.get(&left.into()).cloned();
		let right = self.answers.get(&right.into()).cloned();
		match (left, right)
		{
			(Some(left), Some(right)) => QueryResult::Resolved { left, right },
			(Some(_), None) => QueryResult::WaitingForOne,
			(None, Some(_)) => QueryResult::WaitingForOne,
			(None, None) => QueryResult::WaitingForTwo,
		}
	}

	fn answer(&self) -> i64
	{
		let root: Name = "root".parse().unwrap();
		self.answers.get(&root.into()).cloned().unwrap()
	}
}

#[derive(Debug, Clone, Copy)]
struct WaitingMonkey
{
	name: Name,
	op: Operator,
	left: Name,
	right: Name,
}

impl WaitingMonkey
{
	fn is_waiting_for(&self, other: Name) -> bool
	{
		self.left == other || self.right == other
	}
}

#[derive(Debug, Clone, Copy)]
enum QueryResult
{
	Resolved
	{
		left: i64,
		right: i64,
	},
	WaitingForOne,
	WaitingForTwo,
}

#[derive(Debug, Default, Clone, Copy)]
#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{name}: {expr}")]
struct Monkey
{
	name: Name,
	expr: Expression,
}

#[derive(Debug, Default, Clone, Copy)]
#[derive(parse_display::Display, parse_display::FromStr)]
enum Expression
{
	#[default]
	#[display("null")]
	Null,
	#[display("{0}")]
	Constant(i64),
	#[display("{left} {op} {right}")]
	Operation
	{
		op: Operator,
		left: Name,
		right: Name,
	},
}

#[derive(Debug, Clone, Copy)]
#[derive(parse_display::Display, parse_display::FromStr)]
#[repr(u8)]
enum Operator
{
	#[display("+")]
	Add,
	#[display("-")]
	Sub,
	#[display("*")]
	Mul,
	#[display("/")]
	Div,
}

fn calculate(op: Operator, left: i64, right: i64) -> i64
{
	match op
	{
		Operator::Add => left + right,
		Operator::Sub => left - right,
		Operator::Mul => left * right,
		Operator::Div => left / right,
	}
}

const MONKEY_NAME_LEN: usize = 4;
//const MONKEY_ARRAY_SIZE: usize = 26 * 26 * 26 * 26;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Name([u8; MONKEY_NAME_LEN]);

impl std::fmt::Debug for Name
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
	{
		write!(f, "\"{}\"", self)
	}
}

impl std::fmt::Display for Name
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
	{
		write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
	}
}

impl std::str::FromStr for Name
{
	type Err = std::array::TryFromSliceError;

	fn from_str(input: &str) -> Result<Name, Self::Err>
	{
		let data: [u8; MONKEY_NAME_LEN] = input.as_bytes().try_into()?;
		for x in &data
		{
			let letter: u8 = *x;
			assert!(letter >= b'a' && letter <= b'z');
		}
		Ok(Name(data))
	}
}

impl From<Name> for u32
{
	fn from(name: Name) -> u32
	{
		let mut offset: u32 = 0;
		for x in name.0
		{
			offset = 26 * offset + u32::from(x - b'a');
		}
		offset
	}
}

impl From<Name> for usize
{
	fn from(name: Name) -> usize
	{
		u32::from(name).try_into().unwrap()
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
		assert_eq!(one(PROVIDED), 152);
	}

	#[test]
	fn one_mini()
	{
		assert_eq!(one(MINI), 200);
	}
}
