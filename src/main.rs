#[macro_use]
extern crate stdweb;

use crate::stdweb::web::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod picross;
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

	// Your code goes here!
	log!("Hello world!");

	if let Some((width, height)) = parse_params() {
		log!("width: {}, height: {}", width, height);
	} else {
		let intro = include_str!("html/intro.html");
		document().body().unwrap().append_html(intro).unwrap();
	}
}

use std::str::FromStr;

fn parse_params() -> Option<(usize, usize)> {
	let search = &window().location()?.search().ok()?[1..];
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
