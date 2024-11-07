use std::{
	error::Error as StdError,
	fmt::{self, Display, Formatter},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Kind {
	Request,
	Parse,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Error {
	kind: Kind,
	source: Box<dyn StdError>,
}

impl Error {
	pub(crate) fn new(kind: Kind, source: Box<dyn StdError>) -> Self {
		Self { kind, source }
	}

	pub(crate) fn request<E>(source: E) -> Self
	where
		E: StdError + 'static,
	{
		Self::new(Kind::Request, Box::new(source))
	}

	pub(crate) fn parse<E>(source: E) -> Self
	where
		E: StdError + 'static,
	{
		Self::new(Kind::Parse, Box::new(source))
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self.kind {
			Kind::Request => f.write_str("api request error"),
			Kind::Parse => f.write_str("response parse error"),
		}?;

		Ok(())
	}
}
