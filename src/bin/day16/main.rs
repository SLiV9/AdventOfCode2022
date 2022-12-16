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
	let starting_position = cave.starting_position() as u8;
	let initial_state = State {
		is_open: 0,
		time_remaining_for_traveler: MAX_TIME,
		time_remaining_for_elephant: 0,
		traveler_position: starting_position,
		elephant_position: starting_position,
		total_pressure_added: 0,
		greedy_lower_bound: 0,
		loose_upper_bound: 0,
	};
	calculate_max_total_pressure(&cave, initial_state)
}

fn two(input: &str) -> i32
{
	let mut cave = parse_input(input);
	sort_and_filter_valves(&mut cave);
	let starting_position = cave.starting_position() as u8;
	let initial_state = State {
		is_open: 0,
		time_remaining_for_traveler: MAX_TIME - 4,
		time_remaining_for_elephant: MAX_TIME - 4,
		traveler_position: starting_position,
		elephant_position: starting_position,
		total_pressure_added: 0,
		greedy_lower_bound: 0,
		loose_upper_bound: 0,
	};
	calculate_max_total_pressure(&cave, initial_state)
}

const MAX_TIME: i8 = 30;
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

fn calculate_max_total_pressure(cave: &Cave, mut initial_state: State) -> i32
{
	initial_state.perform_heuristics(cave);
	let mut queue: Vec<State> = Vec::new();
	queue.push(initial_state);
	let mut max_total_pressure = 0;
	while let Some(i_of_max) = queue
		.iter()
		.enumerate()
		.max_by_key(|(_i, state)| state.greedy_lower_bound)
		.map(|(i, _)| i)
	{
		let current = queue.swap_remove(i_of_max);
		//println!("current = {current:?}");

		for i in 0..cave.num_valves
		{
			if current.time_remaining_for_traveler == 0
			{
				break;
			}
			let distance = cave.distance[current.traveler_position as usize][i];
			let time_needed = distance as i8 + 1;
			if distance > 0
				&& current.time_remaining_for_traveler >= time_needed
				&& !current.has_been_opened(i as u8)
			{
				let mut next: State = current;
				next.travel(i as u8, cave);
				next.open(cave);

				decide_what_to_do(
					next,
					&cave,
					&mut queue,
					&mut max_total_pressure,
				);
			}
		}

		for i in 0..cave.num_valves
		{
			if current.time_remaining_for_elephant == 0
			{
				break;
			}
			let distance = cave.distance[current.elephant_position as usize][i];
			let time_needed = distance as i8 + 1;
			if distance > 0
				&& current.time_remaining_for_elephant >= time_needed
				&& !current.has_been_opened(i as u8)
			{
				let mut next: State = current;
				next.lumber(i as u8, cave);
				next.break_open(cave);

				decide_what_to_do(
					next,
					&cave,
					&mut queue,
					&mut max_total_pressure,
				);
			}
		}

		queue.retain(|state| state.loose_upper_bound > max_total_pressure);
		//println!(
		//	"len = {}, min = {}, max = {}, best = {}",
		//	queue.len(),
		//	queue
		//		.iter()
		//		.map(|state| state.loose_upper_bound)
		//		.min()
		//		.unwrap_or(0),
		//	queue
		//		.iter()
		//		.map(|state| state.loose_upper_bound)
		//		.max()
		//		.unwrap_or(0),
		//	max_total_pressure
		//);
	}
	max_total_pressure
}

fn decide_what_to_do(
	mut next: State,
	cave: &Cave,
	queue: &mut Vec<State>,
	max_total_pressure: &mut i32,
)
{
	if next.total_pressure_added > *max_total_pressure
	{
		*max_total_pressure = next.total_pressure_added;
		dbg!(*max_total_pressure);
		//dbg!(&next);
	}

	assert!(next.time_remaining_for_traveler >= 0);
	assert!(next.time_remaining_for_elephant >= 0);
	if next.time_remaining_for_traveler == 0
		&& next.time_remaining_for_elephant == 0
	{
		//dbg!(&next);
		return;
	}

	next.perform_heuristics(cave);
	if next.loose_upper_bound <= *max_total_pressure
	{
		//dbg!(&next);
		return;
	}

	// If we are in the same position and have opened the same
	// valves, the only thing that's changed is the order in which
	// we opened them. So keep the one that is better.
	if let Some(other) = queue.iter_mut().find(|other| {
		other.is_open == next.is_open
			&& other.traveler_position == next.traveler_position
			&& other.elephant_position == next.elephant_position
	})
	{
		if next.total_pressure_added > other.total_pressure_added
		{
			*other = next;
		}
		//dbg!(other);
		return;
	}

	//dbg!(&next);
	queue.push(next);
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
	}
	for _ in 0..cave.num_valves
	{
		for i in 0..cave.num_valves
		{
			for j in 0..i
			{
				for k in 0..j
				{
					fix_distances(&mut cave, i, j, k);
					fix_distances(&mut cave, j, k, i);
					fix_distances(&mut cave, k, i, j);
				}
			}
		}
	}
	for i in 0..cave.num_valves
	{
		for j in 0..i
		{
			assert_eq!(cave.distance[i][j], cave.distance[j][i]);
			assert!(cave.distance[i][j] > 0);
		}
	}
	cave
}

fn fix_distances(cave: &mut Cave, i: usize, j: usize, k: usize)
{
	assert_ne!(i, j);
	assert_ne!(j, k);
	assert_ne!(i, k);
	if cave.distance[i][j] > 0 && cave.distance[j][k] > 0
	{
		let total = cave.distance[i][j] + cave.distance[j][k];
		if cave.distance[i][k] == 0
		{
			cave.distance[i][k] = total;
		}
		else
		{
			cave.distance[i][k] = std::cmp::min(cave.distance[i][k], total);
		}
		cave.distance[k][i] = cave.distance[i][k];
	}
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

	for i in 0..cave.num_valves
	{
		for j in 0..i
		{
			assert_eq!(cave.distance[i][j], cave.distance[j][i]);
			assert!(cave.distance[i][j] > 0);
		}
	}

	cave.num_valves = cave.flow_rate.iter().position(|i| *i == 0).unwrap();

	let s = cave.starting_position();
	for i in 0..cave.num_valves
	{
		if i != s
		{
			assert!(cave.distance[s][i] > 0);
		}
	}
}

#[derive(Debug, Default, Clone, Copy)]
struct State
{
	is_open: u128,
	time_remaining_for_traveler: i8,
	time_remaining_for_elephant: i8,
	traveler_position: u8,
	elephant_position: u8,
	total_pressure_added: i32,
	greedy_lower_bound: i32,
	loose_upper_bound: i32,
}

impl State
{
	fn has_been_opened(&self, pos: u8) -> bool
	{
		(self.is_open & (1u128 << (pos as u128))) != 0
	}

	fn travel(&mut self, to: u8, cave: &Cave)
	{
		let from = self.traveler_position;
		let distance = cave.distance[from as usize][to as usize];
		self.time_remaining_for_traveler -= distance as i8;
		self.traveler_position = to;
	}

	fn lumber(&mut self, to: u8, cave: &Cave)
	{
		let from = self.elephant_position;
		let distance = cave.distance[from as usize][to as usize];
		self.time_remaining_for_elephant -= distance as i8;
		self.elephant_position = to;
	}

	fn open(&mut self, cave: &Cave)
	{
		self.is_open |= 1u128 << (self.traveler_position as u128);
		self.time_remaining_for_traveler -= 1;
		let t = self.time_remaining_for_traveler as i32;
		let flow = cave.flow_rate[self.traveler_position as usize];
		self.total_pressure_added += t * flow;
	}

	fn break_open(&mut self, cave: &Cave)
	{
		self.is_open |= 1u128 << (self.elephant_position as u128);
		self.time_remaining_for_elephant -= 1;
		let t = self.time_remaining_for_elephant as i32;
		let flow = cave.flow_rate[self.elephant_position as usize];
		self.total_pressure_added += t * flow;
	}

	fn perform_heuristics(&mut self, cave: &Cave)
	{
		let mut greedy_traveler_max = 0;
		let mut greedy_elephant_max = 0;
		let mut loose_total: i32 = 0;
		for i in 0..cave.num_valves
		{
			if self.has_been_opened(i as u8)
			{
				continue;
			}
			let flow = cave.flow_rate[i];
			let dt = cave.distance[self.traveler_position as usize][i] as i32;
			let de = cave.distance[self.elephant_position as usize][i] as i32;
			assert!(dt > 0);
			assert!(de > 0);
			let tt = self.time_remaining_for_traveler as i32 - dt - 1;
			let te = self.time_remaining_for_elephant as i32 - de - 1;
			if tt > 0
			{
				let possible_flow = flow * tt;
				if possible_flow > greedy_traveler_max
				{
					greedy_traveler_max = possible_flow;
				}
			}
			if te > 0
			{
				let possible_flow = flow * te;
				if possible_flow > greedy_elephant_max
				{
					greedy_elephant_max = possible_flow;
				}
			}
			let t = std::cmp::max(tt, te);
			if t > 0
			{
				let possible_flow = flow * t;
				loose_total += possible_flow;
			}
		}
		self.greedy_lower_bound = self.total_pressure_added
			+ greedy_traveler_max
			+ greedy_elephant_max;
		self.loose_upper_bound = self.total_pressure_added + loose_total;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const MINI: &str = include_str!("mini.txt");
	const MINIGAP: &str = include_str!("minigap.txt");
	const TESTCASE1: &str = include_str!("testcase1.txt");
	const TESTCASE2: &str = include_str!("testcase2.txt");
	const TESTCASE3: &str = include_str!("testcase3.txt");
	const TESTCASE4: &str = include_str!("testcase4.txt");

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

	#[test]
	fn one_minigap()
	{
		assert_eq!(one(MINIGAP), 543);
	}

	#[test]
	fn one_testcase1()
	{
		assert_eq!(one(TESTCASE1), 2640);
	}

	#[test]
	fn one_testcase2()
	{
		assert_eq!(one(TESTCASE2), 13468);
	}

	#[test]
	fn one_testcase3()
	{
		assert_eq!(one(TESTCASE3), 1288);
	}

	#[test]
	fn one_testcase4()
	{
		assert_eq!(one(TESTCASE4), 2400);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 1707);
	}

	#[test]
	fn two_testcase1()
	{
		assert_eq!(two(TESTCASE1), 2652);
	}

	#[test]
	fn two_testcase2()
	{
		assert_eq!(two(TESTCASE2), 12355);
	}

	#[test]
	fn two_testcase3()
	{
		assert_eq!(two(TESTCASE3), 1484);
	}

	#[test]
	fn two_testcase4()
	{
		assert_eq!(two(TESTCASE4), 3680);
	}
}
