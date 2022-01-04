use structopt::StructOpt;
use date_time::date_tuple::Date;
use super::{misc, opt::Global};
use std::{error::Error, fs::File, io::Write};

#[derive(Debug, StructOpt)]
pub(super) struct Add {
	/// A short name for the event, like who's birthday it is.
	name: String,

	/// The day this event takes place.
	date: String,

	/// What the event is about, where it takes place, or other things to remember.
	description: String,
}

impl Add {
	pub(super) fn run(&mut self, global: Global, file: &mut File)
		-> Result<(), Box<dyn Error>>
	{
		let date = misc::parse_date(&self.date)?;

		misc::sanitize(&mut self.name);
		misc::sanitize(&mut self.description);

		file.write_all(
			format!("{}\t{}\t{}\r\n", self.name, date, self.description)
				.as_bytes()
		)?;

		Ok(())
	}
}