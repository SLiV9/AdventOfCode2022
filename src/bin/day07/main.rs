/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut discovery = Discovery::new();
	for line in input.lines().map(|line| line.parse().unwrap())
	{
		discovery.discover(line);
	}
	let filesystem = discovery.filesystem;
	filesystem
		.directory_sizes
		.into_values()
		.filter(|size| *size <= 100000)
		.sum()
}

fn two(input: &str) -> usize
{
	let mut discovery = Discovery::new();
	for line in input.lines().map(|line| line.parse().unwrap())
	{
		discovery.discover(line);
	}
	let filesystem = discovery.filesystem;
	let current_free_space = TOTAL_DISK_SPACE - filesystem.total_size();
	let extra_size_needed = NEEDED_FREE_DISK_SPACE - current_free_space;
	filesystem
		.directory_sizes
		.into_values()
		.filter(|size| *size >= extra_size_needed)
		.min()
		.unwrap()
}

const TOTAL_DISK_SPACE: usize = 70000000;
const NEEDED_FREE_DISK_SPACE: usize = 30000000;

#[derive(Debug)]
struct Filesystem
{
	directory_sizes: std::collections::HashMap<std::path::PathBuf, usize>,
}

impl Filesystem
{
	fn new() -> Filesystem
	{
		let mut filesystem = Filesystem {
			directory_sizes: std::collections::HashMap::new(),
		};
		filesystem.create_directory("/".parse().unwrap());
		filesystem
	}

	fn create_directory(&mut self, dir_path: std::path::PathBuf)
	{
		self.directory_sizes.insert(dir_path, 0);
	}

	fn add_to_directory(&mut self, dir_path: &std::path::Path, size: usize)
	{
		for path in dir_path.ancestors()
		{
			let dsize = self.directory_sizes.get_mut(path).unwrap();
			*dsize += size;
		}
	}

	fn total_size(&self) -> usize
	{
		let root: std::path::PathBuf = "/".parse().unwrap();
		self.directory_sizes.get(&root).unwrap().to_owned()
	}
}

#[derive(Debug)]
struct Discovery
{
	filesystem: Filesystem,
	current_path: std::path::PathBuf,
}

impl Discovery
{
	fn new() -> Self
	{
		Self {
			filesystem: Filesystem::new(),
			current_path: std::path::PathBuf::new(),
		}
	}

	fn discover(&mut self, line: Line)
	{
		match line
		{
			Line::CdRoot =>
			{
				self.current_path.push("/");
			}
			Line::CdUp =>
			{
				self.current_path.pop();
			}
			Line::CdDown { name } =>
			{
				self.current_path.push(name);
			}
			Line::Ls => (),
			Line::Dir { name } =>
			{
				let mut dir_path = self.current_path.clone();
				dir_path.push(name);
				self.filesystem.create_directory(dir_path);
			}
			Line::File { name: _, size } =>
			{
				self.filesystem.add_to_directory(&self.current_path, size);
			}
		}
	}
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
enum Line
{
	#[display("$ cd /")]
	CdRoot,
	#[display("$ cd ..")]
	CdUp,
	#[display("$ cd {name}")]
	CdDown
	{
		name: String
	},
	#[display("$ ls")]
	Ls,
	#[display("dir {name}")]
	Dir
	{
		name: String
	},
	#[display("{size} {name}")]
	File
	{
		name: String, size: usize
	},
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
		assert_eq!(one(PROVIDED), 95437);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 24933642);
	}
}
