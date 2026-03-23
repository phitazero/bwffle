mod style;
mod parser;
mod command;
mod navigation;
mod action;

use action::{Action, BeforeChange};
use style::{Style, Color, StyledChar};
use command::Command;
use std::io::{BufWriter, Read, Write, stderr, stdin, Stderr};
use std::fs::File;
use std::mem;
use std::collections::HashMap;
use std::sync::LazyLock;

const STYLE_SELECTED: Style = Style {
	foreground: Some(Color { r: 255, g: 0, b: 0 }),
	background: Some(Color { r: 127, g: 0, b: 0 }),
};

const INVERTED: LazyLock<HashMap<char, char>> =
	LazyLock::new(|| maplit::hashmap! {
	'\u{2580}'  => '\u{2584}',
	'\u{2584}'  => '\u{2580}',
	'\u{2581}'  => '\u{1fb86}',
	'\u{1fb86}' => '\u{2581}',
	'\u{2582}'  => '\u{1fb85}',
	'\u{1fb85}' => '\u{2582}',
	'\u{2583}'  => '\u{1fb84}',
	'\u{1fb84}' => '\u{2583}',
	'\u{2585}'  => '\u{1fb83}',
	'\u{1fb83}' => '\u{2585}',
	'\u{2586}'  => '\u{1fb82}',
	'\u{1fb82}' => '\u{2586}',
	'\u{2587}'  => '\u{2594}',
	'\u{2594}'  => '\u{2587}',
	'\u{2588}'  => '\u{20}',
	'\u{20}'    => '\u{2588}',
	'\u{2589}'  => '\u{2595}',
	'\u{2595}'  => '\u{2589}',
	'\u{258a}'  => '\u{1fb87}',
	'\u{1fb87}' => '\u{258a}',
	'\u{258b}'  => '\u{1fb88}',
	'\u{1fb88}' => '\u{258b}',
	'\u{258c}'  => '\u{2590}',
	'\u{2590}'  => '\u{258c}',
	'\u{258d}'  => '\u{1fb89}',
	'\u{1fb89}' => '\u{258d}',
	'\u{258e}'  => '\u{1fb8a}',
	'\u{1fb8a}' => '\u{258e}',
	'\u{258f}'  => '\u{1fb8b}',
	'\u{1fb8b}' => '\u{258f}',
	'\u{2596}'  => '\u{259c}',
	'\u{259c}'  => '\u{2596}',
	'\u{2597}'  => '\u{259b}',
	'\u{259b}'  => '\u{2597}',
	'\u{2598}'  => '\u{259f}',
	'\u{259f}'  => '\u{2598}',
	'\u{2599}'  => '\u{259d}',
	'\u{259d}'  => '\u{2599}',
	'\u{259a}'  => '\u{259e}',
	'\u{259e}'  => '\u{259a}',
});


fn filter_text(text: &str) -> String {
	// trim off chafa's inversion/background trickery
	// apparently it does something useful, but not in my terminal
	text.replace(
		"\x1b[7m\x1b[38;2;0;0;0m \x1b[0m",
		"\x1b[38;2;255;255;255;48;2;0;0;0m "
	)

	// replace ESC[0m with explicit color specification
	.replace(
		"\x1b[0m",
		"\x1b[38;2;255;255;255;48;2;0;0;0m"
	)

	// remove ESC[?25l and ESC[?25h
	.replace("\x1b[?25l", "")
	.replace("\x1b[?25h", "")
}

fn main() {
	let mut text = String::new();

	stdin().read_to_string(&mut text).expect("failed to read stdin");

	text = filter_text(&text);
	let mut chars = parser::digest(&text);

	let mut cursor_pos: usize = 0;
	let mut actions: Vec<Action> = Vec::new();
	let stderr_buf = &mut BufWriter::new(stderr());

	term_canonical_mode(false);


	loop {
		write!(stderr_buf, "\x1b[2J").expect("couldn't write");
		render_preview(stderr_buf, &chars, cursor_pos);

		match command::read() {
			Command::Exit => {
				term_canonical_mode(true);

				render_final(chars);

				std::process::exit(0);
			}

			Command::MoveRight =>
				cursor_pos = navigation::right(&chars, cursor_pos),

			Command::MoveLeft =>
				cursor_pos = navigation::left(&chars, cursor_pos),

			Command::MoveDown =>
				cursor_pos = navigation::down(&chars, cursor_pos),

			Command::MoveUp =>
				cursor_pos = navigation::up(&chars, cursor_pos),

			Command::Undo =>
				action::try_undo(&mut chars, &mut actions),

			Command::SetBlack =>
				set_bg_color(
					&mut chars,
					&mut actions,
					cursor_pos,
					Color { r: 0, g: 0, b: 1 },
				),

			Command::SetTransparent =>
				set_bg_color(
					&mut chars,
					&mut actions,
					cursor_pos,
					Color { r: 0, g: 0, b: 0 },
				),

			Command::Erase =>
				erase_char(&mut chars,
					&mut actions,
					cursor_pos
				),

			Command::Invert => {
				invert(
					&mut chars,
					&mut actions,
					cursor_pos,
				);
			},
		}
	}
}

fn set_bg_color(
	chars: &mut Vec<StyledChar>,
	actions: &mut Vec<Action>,
	pos: usize,
	color: Color,
) {
	// assume it doesn't out of bound
	let styled_char = chars.get_mut(pos).unwrap();

	let background_old = mem::replace(
		&mut styled_char.style.background,
		Some(color),
	);

	actions.push(Action {
		pos,
		prev: BeforeChange::BgColor(background_old)
	});
}

fn erase_char(
	chars: &mut Vec<StyledChar>,
	actions: &mut Vec<Action>,
	pos: usize,
) {
	// assume it doesn't out of bound
	let styled_char = chars.get_mut(pos).unwrap();

	let old_char = styled_char.c;
	styled_char.c = ' ';

	actions.push(Action {
		pos,
		prev: BeforeChange::Char(old_char),
	});
}

fn invert(
	chars: &mut Vec<StyledChar>,
	actions: &mut Vec<Action>,
	pos: usize,
) {
	let styled_char = chars.get_mut(pos).unwrap();

	let Some(&replacement) = INVERTED.get(&styled_char.c) else {
		return;
	};

	mem::swap(
		&mut styled_char.style.foreground,
		&mut styled_char.style.background,
	);

	let old_char = mem::replace(
		&mut styled_char.c,
		replacement,
	);

	actions.push(Action {
		pos,
		prev: BeforeChange::Uninverted(old_char),
	});
}

fn term_canonical_mode(canon_mode: bool) {
	let tty = File::open("/dev/tty")
		.expect("no controlling terminal");

	let fd = std::os::fd::AsRawFd::as_raw_fd(&tty);

	let mut term = termios::Termios::from_fd(fd)
		.expect("couldn't set terminal attributes");

	if canon_mode {
		term.c_lflag |= termios::ICANON | termios::ECHO;
	} else {
		term.c_lflag &= !(termios::ICANON | termios::ECHO);
	}

	termios::tcsetattr(fd, termios::TCSANOW, &term)
		.expect("couldn't set terminal attributes");
}

fn render_preview(
	writer: &mut BufWriter<Stderr>,
	chars: &[StyledChar],
	cursor_pos: usize,
) {
	for (idx, styled_char) in chars.iter().enumerate() {
		if idx == cursor_pos {
			let to_print = StyledChar {
				c: styled_char.c,
				style: STYLE_SELECTED,
			};

			write!(writer, "{to_print}").expect("couldn't write");
		} else {
			write!(writer, "{styled_char}").expect("couldn't write");
		}
	}

	writer.flush().expect("couldn't flush writer");
}

fn render_final(chars: Vec<StyledChar>) {
	let mut prev_style: Style = Style::default();

	print!("\x1b[?25l");

	for StyledChar { c, style } in chars {
		if style.foreground != prev_style.foreground
		&& style.background != prev_style.background
		{
			// all the styles emitted are Some()'s
			print!("\x1b[38;2;{};{};{};48;2;{};{};{}m",
				style.foreground.as_ref().unwrap().r,
				style.foreground.as_ref().unwrap().g,
				style.foreground.as_ref().unwrap().b,
				style.background.as_ref().unwrap().r,
				style.background.as_ref().unwrap().g,
				style.background.as_ref().unwrap().b,
			);

			prev_style = style;
		}
		else if style.foreground != prev_style.foreground
			 && style.background == prev_style.background
		{
			// all the styles emitted are Some()'s
			print!("\x1b[38;2;{};{};{}m",
				style.foreground.as_ref().unwrap().r,
				style.foreground.as_ref().unwrap().g,
				style.foreground.as_ref().unwrap().b,
			);

			prev_style = style;
		}
		else if style.foreground == prev_style.foreground
			 && style.background != prev_style.background
		{
			// all the styles emitted are Some()'s
			print!("\x1b[48;2;{};{};{}m",
				style.background.as_ref().unwrap().r,
				style.background.as_ref().unwrap().g,
				style.background.as_ref().unwrap().b,
			);

			prev_style = style;
		}

		print!("{c}");		
	}

	print!("\x1b[?25h");
}
