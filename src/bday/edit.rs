use structopt::StructOpt;
use super::opt::Global;
use std::{env, error::Error, process};

#[derive(Debug, StructOpt)]
pub(super) struct Edit {}

impl Edit {
	pub(super) fn run(self, global: Global) -> Result<(), Box<dyn Error>> {
		let program = match env::var_os("EDITOR") {
			Some(val) => val,
			None => "nano".into(),
		};
		
		process::Command::new(program)
			.arg(global.file)
			.spawn()?
			.wait()?;

		Ok(())
	}
}