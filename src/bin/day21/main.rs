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
	let root: Name = "root".parse().unwrap();
	computation.answer(root).unwrap()
}

fn two(input: &str) -> i64
{
	let root: Name = "root".parse().unwrap();
	let humn: Name = "humn".parse().unwrap();
	let monkeys = input.lines().map(|line| line.parse().unwrap());
	let monkeys = monkeys.map(|monkey| correct(monkey, root, humn));
	let mut computation = Computation::default();
	computation.store(root, 0);
	for monkey in monkeys
	{
		computation.handle(monkey);
		computation.resolve();
	}
	computation.answer(humn).unwrap_or_else(|| {
		dbg!(&computation);
		computation.find_resolution_issue();
		panic!("unresolved");
	})
}

fn correct(monkey: Monkey, root: Name, humn: Name) -> Monkey
{
	if monkey.name == root
	{
		match monkey.expr
		{
			Expression::Operation { op: _, left, right } => Monkey {
				name: root,
				expr: Expression::SolveOperation {
					op: Operator::Sub,
					left,
					right,
				},
			},
			_ => unreachable!(),
		}
	}
	else if monkey.name == humn
	{
		Monkey {
			name: humn,
			expr: Expression::Null,
		}
	}
	else
	{
		let expr = match monkey.expr
		{
			Expression::Constant(_) => monkey.expr,
			Expression::Operation { op, left, right } =>
			{
				Expression::SolveOperation { op, left, right }
			}
			_ => unreachable!(),
		};
		Monkey {
			name: monkey.name,
			expr,
		}
	}
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
			Expression::Null => (),
			Expression::Constant(answer) => self.store(monkey.name, answer),
			Expression::Operation { op, left, right } =>
			{
				let waiting = WaitingMonkey {
					name: monkey.name,
					op,
					left,
					right,
				};
				self.handle_waiting(waiting)
			}
			Expression::SolveOperation { op, left, right } =>
			{
				let waiting = WaitingMonkey {
					name: monkey.name,
					op,
					left,
					right,
				};
				self.handle_unsolved(waiting)
			}
		}
	}

	fn handle_waiting(&mut self, monkey: WaitingMonkey)
	{
		let result = self.query(monkey.left, monkey.right);
		match result
		{
			QueryResult::Resolved { left, right } =>
			{
				let answer = calculate(monkey.op, left, right);
				self.store(monkey.name, answer);
			}
			QueryResult::WaitingForLeft | QueryResult::WaitingForRight =>
			{
				self.waiting_for_one.push(monkey);
			}
			QueryResult::WaitingForBoth =>
			{
				self.waiting_for_two.push(monkey);
			}
		}
	}

	fn handle_unsolved(&mut self, monkey: WaitingMonkey)
	{
		let result = self.query(monkey.left, monkey.right);
		match result
		{
			QueryResult::Resolved { left, right } =>
			{
				let answer = calculate(monkey.op, left, right);
				self.store(monkey.name, answer);
			}
			QueryResult::WaitingForLeft =>
			{
				if self.answer(monkey.name).is_some()
				{
					self.handle_solved(monkey.solve_left());
				}
				else
				{
					self.waiting_for_one.push(monkey.solve_left());
					self.waiting_for_one.push(monkey);
				}
			}
			QueryResult::WaitingForRight =>
			{
				if self.answer(monkey.name).is_some()
				{
					self.handle_solved(monkey.solve_right());
				}
				else
				{
					self.waiting_for_one.push(monkey.solve_right());
					self.waiting_for_one.push(monkey);
				}
			}
			QueryResult::WaitingForBoth =>
			{
				if self.answer(monkey.name).is_some()
				{
					self.waiting_for_one.push(monkey.solve_left());
					self.waiting_for_one.push(monkey.solve_right());
				}
				else
				{
					self.waiting_for_two.push(monkey.solve_left());
					self.waiting_for_two.push(monkey.solve_right());
					self.waiting_for_two.push(monkey);
				}
			}
		}
	}

	fn handle_solved(&mut self, monkey: WaitingMonkey)
	{
		let result = self.query(monkey.left, monkey.right);
		match result
		{
			QueryResult::Resolved { left, right } =>
			{
				let answer = calculate(monkey.op, left, right);
				self.store(monkey.name, answer);
			}
			_ => unreachable!(),
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
		let old = self.answers.insert(name.into(), answer);
		if let Some(old_answer) = old
		{
			assert_eq!(old_answer, answer);
		}
		self.resolved_names.push_back(name);
	}

	fn query(&self, left: Name, right: Name) -> QueryResult
	{
		let left = self.answers.get(&left.into()).cloned();
		let right = self.answers.get(&right.into()).cloned();
		match (left, right)
		{
			(Some(left), Some(right)) => QueryResult::Resolved { left, right },
			(Some(_), None) => QueryResult::WaitingForRight,
			(None, Some(_)) => QueryResult::WaitingForLeft,
			(None, None) => QueryResult::WaitingForBoth,
		}
	}

	fn answer(&self, name: Name) -> Option<i64>
	{
		self.answers.get(&name.into()).cloned()
	}

	fn find_resolution_issue(&self)
	{
		for monkey in &self.waiting_for_one
		{
			match self.query(monkey.left, monkey.right)
			{
				QueryResult::Resolved { .. } =>
				{
					panic!("not resolved in time: {:?}", monkey)
				}
				QueryResult::WaitingForLeft => (),
				QueryResult::WaitingForRight => (),
				QueryResult::WaitingForBoth => unreachable!(),
			}
		}
		for monkey in &self.waiting_for_two
		{
			match self.query(monkey.left, monkey.right)
			{
				QueryResult::Resolved { .. } => unreachable!(),
				QueryResult::WaitingForLeft => unreachable!(),
				QueryResult::WaitingForRight => unreachable!(),
				QueryResult::WaitingForBoth => (),
			}
		}
	}
}

#[derive(Clone, Copy)]
struct WaitingMonkey
{
	name: Name,
	op: Operator,
	left: Name,
	right: Name,
}

impl std::fmt::Debug for WaitingMonkey
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
	{
		write!(
			f,
			"\"{} <- {} {} {}\"",
			self.name, self.left, self.op, self.right
		)
	}
}

impl WaitingMonkey
{
	fn is_waiting_for(&self, other: Name) -> bool
	{
		self.left == other || self.right == other
	}

	fn solve_left(self) -> Self
	{
		WaitingMonkey {
			name: self.left,
			op: self.op.inverted(),
			left: self.name,
			right: self.right,
		}
	}

	fn solve_right(self) -> Self
	{
		match self.op
		{
			Operator::Add | Operator::Mul => WaitingMonkey {
				name: self.right,
				op: self.op.inverted(),
				left: self.name,
				right: self.left,
			},
			Operator::Sub | Operator::Div => WaitingMonkey {
				name: self.right,
				op: self.op,
				left: self.left,
				right: self.name,
			},
		}
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
	WaitingForLeft,
	WaitingForRight,
	WaitingForBoth,
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
	#[display("solve ({left} {op} {right})")]
	SolveOperation
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

impl Operator
{
	fn inverted(self) -> Self
	{
		match self
		{
			Operator::Add => Operator::Sub,
			Operator::Sub => Operator::Add,
			Operator::Mul => Operator::Div,
			Operator::Div => Operator::Mul,
		}
	}
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
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 301);
	}

	#[test]
	fn one_mini()
	{
		assert_eq!(one(MINI), 200);
	}

	#[test]
	fn two_mini()
	{
		assert_eq!(two(MINI), 50);
	}
}
