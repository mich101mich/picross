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

#[derive(Debug, Serialize, Deserialize)]
pub struct Picross {
	pub width: usize,
	pub height: usize,
	pub grid: Vec<Vec<Value>>,
	pub horizontal: Vec<Vec<usize>>,
	pub vertical: Vec<Vec<usize>>,
}

impl Picross {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			grid: vec![vec![Value::HiddenNothing; width]; height],
			horizontal: vec![vec![]; height],
			vertical: vec![vec![]; width],
		}
	}
}
