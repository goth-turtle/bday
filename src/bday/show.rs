use structopt::StructOpt;
use super::opt::Global;
use std::{error::Error, fs::File};

#[derive(Debug, StructOpt)]
pub(super) struct Show {
	/// Show descriptions.
	#[structopt(short, long)]
	long: bool,
	
	#[structopt(subcommand)]
	mode: ShowMode,
}

#[derive(Debug, StructOpt)]
enum ShowMode {
	/// Show the next N dates. (eg. "next 5").
	Next {
		amount: u16,
	},

	/// Show all dates in this time span (eg. "in 3 months").
	In {
		amount: u16,
		unit: super::TimeUnit,
	},

	/// Show all dates up until this day. (eg. "until Sep 17").
	Until {
		/// Don't show events on this exact day.
		#[structopt(short = "x", long)]
		excluding: bool,

		date: String,
	}
}

impl Show {
	pub(super) fn run(&mut self, global: Global, file: &mut File)
		-> Result<(), Box<dyn Error>>
	{
		Ok(())
	}
}