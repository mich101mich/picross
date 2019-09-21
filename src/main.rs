#[macro_use]
extern crate stdweb;

use crate::stdweb::web::*;

mod picross;
use picross::Picross;
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

pub fn main() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	log!("Hello world!");

	let storage = window().local_storage();

	let parsed = storage
		.get("picross")
		.and_then(|json| serde_json::from_str(&json).ok());

	if let Some(p) = parsed {
		let picross: Picross = p;
		log!("{}", picross.width);

		renderer::render(&picross);
	} else if let Some((width, height, difficulty)) = parse_params() {
		let picross = Picross::new(width, height, difficulty);
		storage
			.insert("picross", &serde_json::to_string(&picross).unwrap())
			.ok();
		js! {
			window.location.replace(window.location.origin + window.location.pathname);
		};
	} else {
		let intro = include_str!("html/intro.html");
		document().body().unwrap().append_html(intro).unwrap();
	}
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
