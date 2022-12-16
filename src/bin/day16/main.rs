/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i32
{
	let mut cave = parse_input(input);
	sort_and_filter_valves(&mut cave);
	calculate_max_total_pressure(&cave)
}

fn two(_input: &str) -> i32
{
	0
}

const MAX_TIME: i32 = 30;
const MAX_VALVES: usize = 128;

#[derive(Debug)]
struct Cave
{
	num_valves: usize,
	valve_labels: [[u8; 2]; MAX_VALVES],
	flow_rate: [i32; MAX_VALVES],
	distance: [[u8; MAX_VALVES]; MAX_VALVES],
}

impl Cave
{
	fn starting_position(&self) -> usize
	{
		self.valve_labels
			.iter()
			.position(|label| label == &[b'A', b'A'])
			.unwrap()
	}
}

fn calculate_max_total_pressure(cave: &Cave) -> i32
{
	let mut initial_state = State {
		is_open: 0,
		time_remaining: MAX_TIME,
		position: cave.starting_position() as u8,
		total_pressure_added: 0,
		heuristic: 0,
	};
	initial_state.perform_heuristic(cave);
	let mut queue: Vec<State> = Vec::new();
	queue.push(initial_state);
	let mut max_total_pressure = 0;
	while let Some(i_of_max) = queue
		.iter()
		.enumerate()
		.max_by_key(|(_i, state)| state.heuristic)
		.map(|(i, _)| i)
	{
		let current = queue.swap_remove(i_of_max);
		if current.total_pressure_added > max_total_pressure
		{
			max_total_pressure = current.total_pressure_added;
			// The heuristic is a loose upper bound on total pressure.
			queue.retain(|state| state.heuristic >= max_total_pressure);
		}

		if current.time_remaining <= 0
		{
			continue;
		}

		for i in 0..cave.num_valves
		{
			let distance = cave.distance[current.position as usize][i];
			let time_needed = distance as i32 + 1;
			if current.time_remaining >= time_needed
				&& !current.has_been_opened(i as u8)
			{
				let mut next: State = current;
				next.travel(i as u8, cave);
				next.open(cave);
				queue.push(next);
			}
		}
	}
	max_total_pressure
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
		distance: [[0; MAX_VALVES]; MAX_VALVES],
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
			let earlier_valve = cave.valve_labels[0..i]
				.iter()
				.position(|label| label == exit.as_bytes());
			if let Some(j) = earlier_valve
			{
				cave.distance[i][j] = 1;
				cave.distance[j][i] = 1;
			}
		}
		for j in 0..i
		{
			if cave.distance[i][j] == 1
			{
				for k in 0..i
				{
					if cave.distance[j][k] > 0
					{
						if cave.distance[i][k] == 0
						{
							cave.distance[i][k] = cave.distance[j][k] + 1;
						}
						else if cave.distance[i][k] > 1
						{
							cave.distance[i][k] = std::cmp::max(
								cave.distance[i][k],
								cave.distance[j][k] + 1,
							);
						}
						cave.distance[k][i] = cave.distance[i][k];
					}
				}
			}
		}
	}
	cave
}

fn sort_and_filter_valves(cave: &mut Cave)
{
	// Sort the valves from high flow rate to low, in particular such that
	// the valves with positive flow rate are all at the start.
	let mut perm = permutation::sort_by_key(&cave.flow_rate, |i| -(*i as i32));
	perm.apply_slice_in_place(&mut cave.valve_labels);
	perm.apply_slice_in_place(&mut cave.flow_rate);
	perm.apply_slice_in_place(&mut cave.distance);
	for i in 0..cave.num_valves
	{
		perm.apply_slice_in_place(&mut cave.distance[i]);
	}
	cave.num_valves = cave.flow_rate.iter().position(|i| *i == 0).unwrap();
}

#[derive(Debug, Default, Clone, Copy)]
struct State
{
	is_open: u128,
	time_remaining: i32,
	position: u8,
	total_pressure_added: i32,
	heuristic: i32,
}

impl State
{
	fn has_been_opened(&self, pos: u8) -> bool
	{
		(self.is_open & (1 << pos)) != 0
	}

	fn travel(&mut self, to: u8, cave: &Cave)
	{
		let from = self.position;
		let distance = cave.distance[from as usize][to as usize];
		self.time_remaining -= distance as i32;
		self.position = to;
	}

	fn open(&mut self, cave: &Cave)
	{
		self.is_open |= 1 << self.position;
		self.time_remaining -= 1;
		let t = self.time_remaining as i32;
		let flow = cave.flow_rate[self.position as usize];
		self.total_pressure_added += t * flow;
	}

	fn perform_heuristic(&mut self, cave: &Cave)
	{
		let flow_available: i32 = (0..cave.num_valves)
			.filter(|i| !self.has_been_opened(*i as u8))
			.map(|i| cave.flow_rate[i])
			.sum();
		let t = self.time_remaining as i32;
		self.heuristic = self.total_pressure_added + t * flow_available;
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
