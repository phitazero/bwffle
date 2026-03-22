use crate::StyledChar;

pub fn right(chars: &[StyledChar], cursor_pos: usize) -> usize {
	match chars.get(cursor_pos + 1)
		.map(|styled_char| styled_char.c)
	{
		Some('\n') => right(chars, cursor_pos + 1),
		Some(_) => cursor_pos + 1,
		None => cursor_pos,
	}
}

pub fn left(chars: &[StyledChar], cursor_pos: usize) -> usize {
	if cursor_pos == 0 {
		return cursor_pos;
	}

	match chars.get(cursor_pos - 1)
		.map(|styled_char| styled_char.c)
	{
		Some('\n') => left(chars, cursor_pos - 1),
		Some(_) => cursor_pos - 1,
		None => cursor_pos,
	}
}

pub fn down(chars: &[StyledChar], cursor_pos: usize) -> usize {
	// index of the character after \n
	// this way on the first line it can return 0
	// else index of the imaginary \n before the first line
	// would be -1, and we have usize here
	let line_start_idx = chars[..=cursor_pos]
		.iter()
		.map(|styled_char| styled_char.c)
		.rposition(|c| c == '\n')
		.map(|idx| idx + 1)
		.unwrap_or(0);

	if line_start_idx > cursor_pos {
		return cursor_pos;
	}

	let Some(rel_next_line_start_idx) = chars[cursor_pos..]
		.iter()
		.map(|styled_char| styled_char.c)
		.position(|c| c == '\n')
		.map(|idx| idx + 1)
	else {
		return cursor_pos;
	};

	let line_start_offset = cursor_pos - line_start_idx;
	let next_line_start_idx = cursor_pos + rel_next_line_start_idx;

	let next_cursor_pos = next_line_start_idx + line_start_offset;

	next_cursor_pos.min(chars.len() - 2)
}

pub fn up(chars: &[StyledChar], cursor_pos: usize) -> usize {
	let Some(line_start_idx) = chars[..=cursor_pos]
		.iter()
		.map(|styled_char| styled_char.c)
		.rposition(|c| c == '\n')
		.map(|idx| idx + 1)
	else {
		return cursor_pos;
	};

	// index of the character after \n
	// this way on the first line it can return 0
	// else index of the imaginary \n before the first line
	// would be -1, and we have usize here
	let prev_line_start_idx = chars[..line_start_idx - 1]
		.iter()
		.map(|styled_char| styled_char.c)
		.rposition(|c| c == '\n')
		.map(|idx| idx + 1)
		.unwrap_or(0);

	let line_start_offset = cursor_pos - line_start_idx;
	let next_cursor_pos = prev_line_start_idx + line_start_offset;

	next_cursor_pos
}
