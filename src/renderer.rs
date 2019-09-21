use super::picross::*;
use stdweb::web::*;

pub fn render(picross: &Picross) {
	let output = include_str!("html/picross.html");

	let col_groups: String = (0..picross.width)
		.map(|x| format!("<col id=\"col_{}\">", x))
		.collect();
	
	let col_groups = format!("<colgroup>{}</colgroup>", col_groups);

	let grid: String = picross
		.grid
		.iter()
		.enumerate()
		.map(|(y, row)| {
			let r: String = row
				.iter()
				.enumerate()
				.map(|(x, v)| format!("<td id=\"{}:{}\">{}</td>", y, x, v))
				.collect();

			format!("<tr>{}</tr>", r)
		})
		.collect();

	let content = format!("{}\n{}", col_groups, grid);

	let output = output.replace("{{{picross}}}", &content);

	document().body().unwrap().append_html(&output).ok();
}
