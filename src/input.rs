use super::{load_picross, picross::*, save_picross};
use event::*;
use std::str::FromStr;
use stdweb::web::*;

fn coords_from_element(elem: &Element) -> (usize, usize) {
	let id = elem.get_attribute("id").expect("Cell without ID");
	let mut coords = id
		.split(':')
		.map(|s| usize::from_str(s).expect("Invalid Number in ID"));
	let x = coords.next().expect("Invalid ID");
	let y = coords.next().expect("Invalid ID");
	(x, y)
}

fn mouse_enter(event: MouseEnterEvent) {
	let left = event.buttons().is_down(MouseButton::Left);
	let right = event.buttons().is_down(MouseButton::Right);
	handle_event(left, right);
	event.cancel_bubble();
	event.prevent_default();
}

fn mouse_down(event: MouseDownEvent) {
	let left = event.buttons().is_down(MouseButton::Left);
	let right = event.buttons().is_down(MouseButton::Right);
	handle_event(left, right);
	event.cancel_bubble();
	event.prevent_default();
}

fn mouse_up(event: MouseUpEvent) {
	event.cancel_bubble();
	event.prevent_default();
}

fn context_menu(event: ContextMenuEvent) {
	event.cancel_bubble();
	event.prevent_default();
}

fn handle_event(left: bool, right: bool) {
	if !left && !right {
		return;
	}
	let marking = right;

	if marking {
		if let Some(cell) = document().query_selector("td.cell.marked:hover").unwrap() {
			cell.class_list().remove("marked").expect("Class change");
			cell.class_list().add("hidden").expect("Class change");

			let mut picross = load_picross().unwrap();
			let (x, y) = coords_from_element(&cell);
			match picross.grid[y][x] {
				Value::MarkedNothing => {
					picross.grid[y][x] = Value::HiddenNothing;
				}
				Value::MarkedTile => {
					picross.grid[y][x] = Value::HiddenTile;
				}
				v => panic!("Picross != Table at ({}, {}): {:?} vs marked", x, y, v),
			}

			save_picross(&picross);
			return;
		}
	}

	if let Some(cell) = document().query_selector("td.cell.hidden:hover").unwrap() {
		cell.class_list().remove("hidden").expect("Class change");

		let mut picross = load_picross().unwrap();
		let (x, y) = coords_from_element(&cell);

		match picross.grid[y][x] {
			Value::HiddenNothing => {
				if marking {
					cell.class_list().add("marked").expect("Class change");
					picross.grid[y][x] = Value::MarkedNothing;
				} else {
					cell.class_list().add("incorrect").expect("Class change");
					picross.grid[y][x] = Value::Incorrect;
					picross.errors += 1;
					let errors = picross.errors.to_string();
					js! {
						setTimeout(() => alert("Whoops! Error\n" + @{errors} + " Errors Total"));
					};
				}
			}
			Value::HiddenTile => {
				if marking {
					cell.class_list().add("marked").expect("Class change");
					picross.grid[y][x] = Value::MarkedTile;
				} else {
					cell.class_list().add("correct").expect("Class change");
					picross.grid[y][x] = Value::Correct;
				}
			}
			v => panic!("Picross != Table at ({}, {}): {:?} vs hidden", x, y, v),
		}

		save_picross(&picross);
	}
}

pub fn setup_inputs() {
	let table = document().get_element_by_id("picross").unwrap();

	table.add_event_listener(mouse_down);
	table.add_event_listener(mouse_up);
	table.add_event_listener(context_menu);

	for cell in table.query_selector_all("td.cell").unwrap() {
		cell.add_event_listener(mouse_enter);
	}
}
