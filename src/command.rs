use crossterm::event::{self, Event, KeyCode};

pub fn read() -> Command {
	loop {
		let event = event::read().unwrap();

		let key_event = if let Event::Key(ke) = event { ke } else {
			continue;
		};

		match key_event.code {
			KeyCode::Char('q') => return Command::Exit,
			KeyCode::Char('t') => return Command::SetTransparent,
			KeyCode::Char('b') => return Command::SetBlack,
			KeyCode::Char('d') | KeyCode::Right
				=> return Command::MoveRight,
			KeyCode::Char('a') | KeyCode::Left
				=> return Command::MoveLeft,

			KeyCode::Char('s') | KeyCode::Down
				=> return Command::MoveDown,

			KeyCode::Char('w') | KeyCode::Up
				=> return Command::MoveUp,

			KeyCode::Char('u') | KeyCode::Char('z')
				=> return Command::Undo,

			_ => (),
		}
	}
}

#[derive(Debug)]
pub enum Command {
	Exit,
	SetBlack,
	SetTransparent,
	MoveRight,
	MoveLeft,
	MoveDown,
	MoveUp,
	Undo,
}
