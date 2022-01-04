use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub(super) struct Opt {
	#[structopt(flatten)]
	pub(super) global: Global,
    
	#[structopt(subcommand)]
	pub(super) cmd: Command,
}

#[derive(Debug, StructOpt)]
pub struct Global {
	/// The file to store events in, $HOME/.config/bday/events by default
	#[structopt(short, long, env = "BDAY_FILE")]
	#[structopt(parse(from_os_str), default_value = "")]
	pub file: PathBuf,
}

#[derive(Debug, StructOpt)]
pub(super) enum Command {
	/// Create a new event.
	Add(super::add::Add),

	/// Forget an existing event.
	Remove(super::remove::Remove),

	/// Open the event file in an editor for direct editing.
	Edit(super::edit::Edit),
	
	/// Display upcoming events.
	Show(super::show::Show),
}