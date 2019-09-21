#[macro_use]
extern crate stdweb;

use crate::stdweb::web::*;

mod picross;
use picross::Picross;
mod input;
mod renderer;

#[macro_export]
macro_rules! log {
	( $( $x: expr ),* ) => {
		let s = format!($( $x ),*);
		js!(
			console.log(@{ s });
		)
	};
}

fn load_picross() -> Option<Picross> {
	window()
		.local_storage()
		.get("picross")
		.and_then(|json| serde_json::from_str(&json).ok())
}

fn save_picross(picross: &Picross) {
	window()
		.local_storage()
		.insert("picross", &serde_json::to_string(&picross).unwrap())
		.expect("Save Picross");
}

pub fn main() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	log!("Hello world!");

	let picross: Picross;

	if let Some(p) = load_picross() {
		picross = p;
		log!("{}", picross.width);
	} else if let Some((width, height, difficulty)) = parse_params() {
		picross = Picross::new(width, height, difficulty);
		save_picross(&picross);
	} else {
		let intro = include_str!("html/intro.html");
		document().body().unwrap().append_html(intro).unwrap();
		return;
	}

	renderer::render(&picross);
	input::setup_inputs();
}

use std::str::FromStr;

fn parse_params() -> Option<(usize, usize, usize)> {
	let url = url::Url::from_str(&window().location()?.href().ok()?).ok()?;
	let query = url.query_pairs();

	let width = query.clone().find(|(name, _)| name == "width")?.1;
	let height = query.clone().find(|(name, _)| name == "height")?.1;
	let difficulty = query.clone().find(|(name, _)| name == "difficulty")?.1;

	let width = usize::from_str(&width).ok()?;
	let height = usize::from_str(&height).ok()?;
	let difficulty = usize::from_str(&difficulty).ok()?;

	if width > 0 && height > 0 && difficulty >= 5 && difficulty <= 95 {
		Some((width, height, difficulty))
	} else {
		None
	}
}
