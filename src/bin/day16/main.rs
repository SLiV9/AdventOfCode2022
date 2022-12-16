/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i32
{
	let cave = parse_input(input);
	calculate_max_total_pressure(&cave)
}

fn two(_input: &str) -> i32
{
	0
}

const MAX_TIME: usize = 30;
const MAX_VALVES: usize = 128;

#[derive(Debug)]
struct Cave
{
	num_valves: usize,
	valve_labels: [[u8; 2]; MAX_VALVES],
	flow_rate: [i32; MAX_VALVES],
	is_connected: [u128; MAX_VALVES],
}

impl Cave
{
	fn starting_position(&self) -> usize
	{
		self.valve_labels[0..self.num_valves]
			.iter()
			.position(|label| label == &[b'A', b'A'])
			.unwrap()
	}

	fn are_connected(&self, from: u8, to: u8) -> bool
	{
		(self.is_connected[from as usize] & (1 << to)) != 0
	}
}

fn calculate_max_total_pressure(cave: &Cave) -> i32
{
	let initial_state = State {
		is_open: 0,
		position: cave.starting_position() as u8,
		suggestion: 0,
		flow: 0,
		total_pressure: 0,
	};
	let mut stack = [State::default(); MAX_TIME + 1];
	stack[0] = initial_state;
	let mut t = 0;
	let mut max_total_pressure = 0;
	loop
	{
		while (stack[t].suggestion as usize) >= cave.num_valves
		{
			if t == 0
			{
				return max_total_pressure;
			}
			t -= 1;
			stack[t].suggestion += 1;
		}

		let current = &mut stack[t];
		//println!("t = {t}, current = {current:?}");

		if t == MAX_TIME
		{
			let total_pressure = current.total_pressure;
			if total_pressure > max_total_pressure
			{
				max_total_pressure = total_pressure;
			}
			current.suggestion = 0xFF;
		}
		else if current.suggestion == current.position
			&& !current.has_been_opened(current.position)
		{
			let valve = current.position;
			stack[t + 1] = current.advance();
			stack[t + 1].open(valve, cave);
			t += 1;
		}
		else if cave.are_connected(current.position, current.suggestion)
		{
			let destination = current.suggestion;
			stack[t + 1] = current.advance();
			stack[t + 1].position = destination;
			t += 1;
		}
		else
		{
			current.suggestion += 1;
		}
	}
}

const READING_REGEX: &str = "Valve (?P<label>[A-Z][A-Z]) has flow \
                             rate=(?P<flowrate>[0-9]+); tunnels? leads? to \
                             valves? (?P<exits>[A-Z]+(, [A-Z]+)*)";

fn parse_input(input: &str) -> Cave
{
	let reading_regex = regex::Regex::new(READING_REGEX).unwrap();
	let mut cave = Cave {
		num_valves: 0,
		valve_labels: [[0; 2]; MAX_VALVES],
		flow_rate: [0; MAX_VALVES],
		is_connected: [0; MAX_VALVES],
	};
	for line in input.lines()
	{
		let captures = reading_regex.captures(line).unwrap();
		let i = cave.num_valves;
		assert!(i < MAX_VALVES);
		cave.num_valves += 1;
		let label = captures.name("label").unwrap().as_str();
		cave.valve_labels[i] = label.as_bytes().try_into().unwrap();
		let flowrate_str = captures.name("flowrate").unwrap().as_str();
		cave.flow_rate[i] = flowrate_str.parse().unwrap();
		let exits_str = captures.name("exits").unwrap().as_str();
		let exits = exits_str.split(", ");
		for exit in exits
		{
			for j in 0..i
			{
				if &cave.valve_labels[j] == exit.as_bytes()
				{
					cave.is_connected[i] |= 1 << j;
					cave.is_connected[j] |= 1 << i;
				}
			}
		}
	}
	cave
}

#[derive(Debug, Default, Clone, Copy)]
struct State
{
	is_open: u128,
	position: u8,
	suggestion: u8,
	flow: i32,
	total_pressure: i32,
}

impl State
{
	fn has_been_opened(&self, pos: u8) -> bool
	{
		(self.is_open & (1 << pos)) != 0
	}

	fn open(&mut self, pos: u8, cave: &Cave)
	{
		self.is_open |= 1 << pos;
		self.flow += cave.flow_rate[self.position as usize];
	}

	fn advance(&self) -> State
	{
		let total_pressure = self.total_pressure + self.flow;
		State {
			is_open: self.is_open,
			position: self.position,
			suggestion: 0,
			flow: self.flow,
			total_pressure,
		}
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
		assert_eq!(one(PROVIDED), 1651);
	}

	#[test]
	fn one_mini()
	{
		assert_eq!(one(MINI), 931);
	}
}
