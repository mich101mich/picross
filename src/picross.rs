
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
	Hidden(bool),
	Correct,
	Incorrect,
	Marked,
}

#[derive(Debug)]
pub struct Picross {
	width: usize,
	height: usize,
	grid: Vec<Vec<Value>>,
	horizontal: Vec<Vec<usize>>,
	vertical: Vec<Vec<usize>>,
}

impl Picross {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			grid: vec![vec![Value::Hidden(false); width]; height],
			horizontal: vec![vec![]; height],
			vertical: vec![vec![]; width],
		}
	}
}
