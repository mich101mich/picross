use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Value {
	HiddenNothing,
	HiddenTile,
	Correct,
	Incorrect,
	Marked,
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		use Value::*;
		let s = match *self {
			HiddenNothing => "[ ]",
			HiddenTile => "[O]",
			Correct => " O ",
			Incorrect => " X ",
			Marked => "<X>",
		};
		write!(f, "{}", s)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Picross {
	pub width: usize,
	pub height: usize,
	pub grid: Vec<Vec<Value>>,
	pub horizontal: Vec<Vec<usize>>,
	pub vertical: Vec<Vec<usize>>,
}

impl Picross {
	pub fn new(width: usize, height: usize, difficulty: usize) -> Self {
		let mut r = rand::thread_rng();

		let mut grid = vec![vec![Value::HiddenNothing; width]; height];

		let difficulty_scale = 2.0 - difficulty as f32 / 50.0;
		let cell_count = (width + height) as f32;

		let pre_placed = (difficulty_scale.powf(2.0) * cell_count).ceil() as usize;

		for _ in 0..pre_placed {
			grid[r.gen_range(0, height)][r.gen_range(0, width)] = Value::HiddenTile;
		}

		let iterations = (difficulty_scale * (cell_count / 10.0 + 1.0)).ceil() as usize;

		for _ in 0..iterations {
			for y in 0..height {
				for x in 0..width {
					if grid[y][x] == Value::HiddenTile {
						continue;
					}
					let count: usize = [(0, 1), (1, 0), (0, -1), (-1, 0)]
						.into_iter()
						.map(|(dx, dy)| (x as isize + dx, y as isize + dy))
						.filter(|&(x, y)| {
							x > 0 && x < width as isize && y > 0 && y < height as isize
						})
						.map(|(x, y)| grid[y as usize][x as usize] == Value::HiddenTile)
						.map(|tile| if tile { 1 } else { 0 })
						.sum();

					if r.gen_range(0, count * 2 + 12) > 10 {
						grid[y][x] = Value::HiddenTile;
					}
				}
			}
		}

		Self {
			width,
			height,
			grid: grid,
			horizontal: vec![vec![]; height],
			vertical: vec![vec![]; width],
		}
	}
}
