use super::picross::*;
use stdweb::web::*;

pub fn render(picross: &Picross) {
	let output = include_str!("html/picross.html");

	let max_horizontal: usize = picross
		.horizontal
		.iter()
		.map(|v| v.len())
		.max()
		.unwrap_or(0);

	let max_vertical: usize = picross.vertical.iter().map(|v| v.len()).max().unwrap_or(0);

	let col_groups: String = (0..max_horizontal)
		.map(|_| "<col>".to_string())
		.chain((0..picross.width).map(|x| format!("<col id=\"col_{}\">", x)))
		.collect();
	let col_groups = format!("<colgroup>{}</colgroup>", col_groups);

	let mut rows = vec![];

	for y in 0..max_vertical {
		let mut row = String::from("<tr>");
		for _ in 0..max_horizontal {
			row += "<td class=\"empty\"></td>";
		}

		for col in picross.vertical.iter() {
			let offset = max_vertical - col.len();
			if y >= offset {
				row += &format!("<td class=\"vertical\">{}</td>", col[y - offset]);
			} else {
				row += "<td class=\"vertical\"></td>";
			}
		}

		rows.push(row + "</tr>");
	}

	for y in 0..picross.height {
		let mut row = String::from("<tr class=\"cell-row\">");

		let numbers = &picross.horizontal[y];
		for _ in numbers.len()..max_horizontal {
			row += "<td class=\"horizontal\"></td>";
		}

		for v in numbers.iter() {
			row += &format!("<td class=\"horizontal\">{}</td>", v);
		}

		for (x, v) in picross.grid[y].iter().enumerate() {
			let mut class = String::from("cell");
			if x == 0 {
				class += " cell-left";
			} else if x == picross.width - 1 {
				class += " cell-right";
			} else if x % 5 == 4 {
				class += " highlight-right"
			}
			if y == 0 {
				class += " cell-top";
			} else if y == picross.height - 1 {
				class += " cell-bottom";
			} else if y % 5 == 4 {
				class += " highlight-bottom"
			}

			use Value::*;
			match v {
				HiddenNothing | HiddenTile => {
					class += " hidden";
				}
				Correct => {
					class += " correct";
				}
				Incorrect => {
					class += " incorrect";
				}
				MarkedNothing | MarkedTile => {
					class += " marked";
				}
			}

			row += &format!("<td id=\"{}:{}\" class=\"{}\"></td>", x, y, class);
		}

		rows.push(row + "</tr>");
	}

	let content = format!("{}\n{}", col_groups, rows.join("\n"));

	let output = output.replace("{{{picross}}}", &content);

	document().body().unwrap().append_html(&output).ok();
}
