use std::fmt;

#[derive(Debug, Clone)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl From<u8> for Color {
	fn from(value: u8) -> Self {
		Self {
			r: value,
			g: value,
			b: value,
		}
	}
}

#[derive(Debug, Default, Clone)]
pub struct Style {
	pub foreground: Option<Color>,
	pub background: Option<Color>,
}

impl fmt::Display for Style {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "\x1b[")?;

		if let Some(ref foreground) = self.foreground {
			let Color { r, g, b } = foreground;
			write!(f, "38;2;{r};{g};{b};")?;
		}

		if let Some(ref background) = self.background {
			let Color { r, g, b } = background;
			write!(f, "48;2;{r};{g};{b};")?;
		}

		write!(f, "m")
	}
}

impl Style {
	pub fn update(&mut self, other: Style) {
		if other.foreground.is_some() {
			self.foreground = other.foreground;
		}

		if other.background.is_some() {
			self.background = other.background;
		}
	}
}

impl From<Option<Style>> for Style {
    fn from(value: Option<Style>) -> Self {
        match value {
        	Some(style) => style,
        	None => Style::default(),
        }
    }
}

#[derive(Debug)]
pub struct StyledChar {
	pub c: char,
	pub style: Style,
}

impl fmt::Display for StyledChar {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}", self.style, self.c)
	}
}
