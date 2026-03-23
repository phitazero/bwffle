use crate::{Color, StyledChar};
use std::mem;

pub struct Action {
	pub pos: usize,
	pub prev: BeforeChange,
}

pub enum BeforeChange {
	BgColor(Option<Color>),
	Char(char),
	Uninverted(char),
}

pub fn try_undo(
	chars: &mut Vec<StyledChar>,
	actions: &mut Vec<Action>,
) {
	if let Some(action) = actions.pop() {
		// chars doesn't change after initial read
		// actions shouldn't be out of bounds (ideally)
		let styled_char = chars
			.get_mut(action.pos)
			.unwrap();

		match action.prev {
			BeforeChange::BgColor(old_bg) =>
				styled_char.style.background = old_bg,

			BeforeChange::Char(old_char) =>
				styled_char.c = old_char,

			BeforeChange::Uninverted(old_char) => {
				styled_char.c = old_char;

				mem::swap(
					&mut styled_char.style.foreground,
					&mut styled_char.style.background,
				);
			}
		}
	}
}
