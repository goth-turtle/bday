use structopt::StructOpt;
use super::opt::Global;
use std::{error::Error, fs::File};

#[derive(Debug, StructOpt)]
pub(super) struct Remove {
	/// The name of the event to remove.
	name: String,
}

impl Remove {
	pub(super) fn run(&mut self, global: Global, file: &mut File)
		-> Result<(), Box<dyn Error>>
	{
		Ok(())
	}
}