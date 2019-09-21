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
	MarkedNothing,
	MarkedTile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Picross {
	pub width: usize,
	pub height: usize,
	pub difficulty: usize,
	pub grid: Vec<Vec<Value>>,
	pub horizontal: Vec<Vec<usize>>,
	pub vertical: Vec<Vec<usize>>,
	pub errors: usize,
}

impl Picross {
	pub fn new(width: usize, height: usize, difficulty: usize) -> Self {
		let mut r = rand::thread_rng();

		let mut grid = vec![vec![Value::HiddenNothing; width]; height];

		let difficulty_scale = 2.0 - difficulty as f32 / 50.0;

		let pre_placed = (difficulty_scale.powf(2.0) * (width + height) as f32).ceil() as usize;

		for _ in 0..pre_placed {
			grid[r.gen_range(0, height)][r.gen_range(0, width)] = Value::HiddenTile;
		}

		let iterations = (difficulty_scale * 3.0).ceil() as usize;

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

					let probability = (count as f32 * 2.5) as usize + 12;
					if r.gen_range(0, probability) > 10 {
						grid[y][x] = Value::HiddenTile;
					}
				}
			}
		}

		let mut horizontal = vec![vec![]; height];

		for (y, out) in horizontal.iter_mut().enumerate() {
			let mut current = 0;
			for v in grid[y].iter().chain(std::iter::once(&Value::HiddenNothing)) {
				if *v == Value::HiddenTile {
					current += 1;
				} else if current > 0 {
					out.push(current);
					current = 0;
				}
			}
		}

		let mut vertical = vec![vec![]; width];

		for (x, out) in vertical.iter_mut().enumerate() {
			let mut current = 0;
			for v in grid
				.iter()
				.map(|row| &row[x])
				.chain(std::iter::once(&Value::HiddenNothing))
			{
				if *v == Value::HiddenTile {
					current += 1;
				} else if current > 0 {
					out.push(current);
					current = 0;
				}
			}
		}

		crate::log!("vertical: {:?}", vertical);
		crate::log!("horizontal: {:?}", horizontal);

		Self {
			width,
			height,
			difficulty,
			grid,
			horizontal,
			vertical,
			errors: 0,
		}
	}
}
