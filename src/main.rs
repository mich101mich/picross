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
	} else if let Some((width, height)) = parse_params() {
		let picross = Picross::new(width, height);
		storage
			.insert("picross", &serde_json::to_string(&picross).unwrap())
			.ok();
		js! {
			window.location.replace(window.location.origin);
		};
	} else {
		let intro = include_str!("html/intro.html");
		document().body().unwrap().append_html(intro).unwrap();
	}
}

use std::str::FromStr;

fn parse_params() -> Option<(usize, usize)> {
	let search = window().location()?.search().ok()?;
	if search.is_empty() {
		return None;
	}
	let search = &search[1..];
	let mut params = search.split('&');
	let width = params.next()?;
	let height = params.next()?;
	if !width.starts_with("width=") || !height.starts_with("height=") {
		return None;
	}
	let width = usize::from_str(&width["width=".len()..]).ok()?;
	let height = usize::from_str(&height["height=".len()..]).ok()?;

	if width > 0 && height > 0 {
		Some((width, height))
	} else {
		None
	}
}
