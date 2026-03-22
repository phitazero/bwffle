use std::sync::LazyLock;
use regex::{Regex, Captures};
use crate::{Style, Color, StyledChar};

static FG_REGEX: LazyLock<Regex> = LazyLock::new(|| { Regex::new(r"^\x1b\[38;+2;+(\d{1,3});+(\d{1,3});+(\d{1,3})m").unwrap() });
static BG_REGEX: LazyLock<Regex> = LazyLock::new(|| { Regex::new(r"^\x1b\[48;+2;+(\d{1,3});+(\d{1,3});+(\d{1,3})m").unwrap() });
static FG_BG_REGEX: LazyLock<Regex> = LazyLock::new(|| { Regex::new(r"^\x1b\[38;+2;+(\d{1,3});+(\d{1,3});+(\d{1,3});+48;+2;+(\d{1,3});+(\d{1,3});+(\d{1,3})m").unwrap() });
static BG_FG_REGEX: LazyLock<Regex> = LazyLock::new(|| { Regex::new(r"^\x1b\[48;+2;+(\d{1,3});+(\d{1,3});+(\d{1,3});+38;+2;+(\d{1,3});+(\d{1,3});+(\d{1,3})m").unwrap() });

fn eat_captures(text: &mut &str, caps: &Captures) {
	let parts = text.split_at(caps.get(0).unwrap().len());
	*text = parts.1;
}

fn eat_fg(text: &mut &str) -> Option<Style> {
	FG_REGEX.captures(text)
		.map(|caps| {
			eat_captures(text, &caps);

			Style {
				foreground: Some(Color {
					r: caps[1].parse().unwrap(),
					g: caps[2].parse().unwrap(),
					b: caps[3].parse().unwrap(),
				}),
				background: None,
			}
		})
}

fn eat_bg(text: &mut &str) -> Option<Style> {
	BG_REGEX.captures(text)
		.map(|caps| {
			eat_captures(text, &caps);

			Style {
				foreground: None,
				background: Some(Color {
					r: caps[1].parse().unwrap(),
					g: caps[2].parse().unwrap(),
					b: caps[3].parse().unwrap(),
				}),
			}
		})
}

fn eat_fg_bg(text: &mut &str) -> Option<Style> {
	FG_BG_REGEX.captures(text)
		.map(|caps| {
			eat_captures(text, &caps);

			Style {
				foreground: Some(Color {
					r: caps[1].parse().unwrap(),
					g: caps[2].parse().unwrap(),
					b: caps[3].parse().unwrap(),
				}),
				background: Some(Color {
					r: caps[4].parse().unwrap(),
					g: caps[5].parse().unwrap(),
					b: caps[6].parse().unwrap(),
				}),
			}
		})
}

fn eat_bg_fg(text: &mut &str) -> Option<Style> {
	BG_FG_REGEX.captures(text)
		.map(|caps| {
			eat_captures(text, &caps);

			Style {
				foreground: Some(Color {
					r: caps[4].parse().unwrap(),
					g: caps[5].parse().unwrap(),
					b: caps[6].parse().unwrap(),
				}),
				background: Some(Color {
					r: caps[1].parse().unwrap(),
					g: caps[2].parse().unwrap(),
					b: caps[3].parse().unwrap(),
				}),
			}
		})
}

fn eat_style(text: &mut &str) -> Style {
	let mut result: Style = Style::default();

	loop {
		let parsed = eat_fg(text)
		.or(eat_bg(text))
		.or(eat_fg_bg(text))
		.or(eat_bg_fg(text));

		match parsed {
			Some(parsed) => result.update(parsed.into()),
			None => break,
		}
	}

	result
}

pub fn digest(mut text: &str) -> Vec<StyledChar> {
	let mut styled_chars: Vec<StyledChar> = Vec::new();
	let mut current_style = Style::default();

	loop {
		current_style.update(eat_style(&mut text));

		if let Some(curr_char) = text.chars().next() {
			text = text.split_at(curr_char.len_utf8()).1;

			styled_chars.push(StyledChar {
				c: curr_char,
				style: current_style.clone(),
			})
		} else {
			break;
		}
	}

	styled_chars
}
