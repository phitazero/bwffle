use crate::{Color, StyledChar};

pub struct Action {
	pub pos: usize,
	pub background_old: Option<Color>,
}

pub fn try_undo(
	chars: &mut Vec<StyledChar>,
	actions: &mut Vec<Action>,
) {
	if let Some(action) = actions.pop() {
		// chars doesn't change after initial read
		// actions shouldn't be out of bounds (ideally)
		chars
			.get_mut(action.pos)
			.unwrap()
			.style.background = action.background_old;
	}
}
