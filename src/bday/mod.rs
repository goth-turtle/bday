use std::{env, error::Error, fmt, fs::{self, OpenOptions}, str::FromStr};
use structopt::StructOpt;
use opt::{Opt, Command};

mod opt;
mod misc;

mod add;
mod remove;
mod edit;
mod show;

pub struct Bday {
    options: Opt,
}

impl Bday {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let mut options = Opt::from_args_safe()?;
		
		// Fall back to default location
		if options.global.file.as_os_str().is_empty() {
			options.global.file.push(
				env::var_os("HOME")
					.ok_or(BdayError {msg: "could not find $HOME".into()})?
			);
			options.global.file.push(".config/bday/events");
		}
		
		// Create the file and directories if necessary
		if !options.global.file.exists() {
			if let Some(dir) = options.global.file.parent() {
				fs::create_dir_all(dir)?;
			}
			fs::File::create(&options.global.file)?;
		}
		
        Ok(Self {options})
    }

	pub fn run(self) -> Result<(), Box<dyn Error>> {
		let global = self.options.global;
        
        // This command just needs the file name.
		if let Command::Edit(e) = self.options.cmd {
			return e.run(global);
		}

        // All other commands need direct file access.
		let mut file = OpenOptions::new()
			.read(true)
			.write(true)
			.open(&global.file)?;
        
		match self.options.cmd {
			Command::Remove(mut r)	=> r.run(global, &mut file),
			Command::Add(mut a)		=> a.run(global, &mut file),
			Command::Show(mut s)	=> s.run(global, &mut file),
            _ => Err(Box::new(BdayError {
				msg: "Unsupported subcommand in Bday::run()".into()}))
		}
	}
}

#[derive(Debug, StructOpt)]
pub(super) enum TimeUnit {
	Day,
	Week,
	Month,
	Year,
}

impl FromStr for TimeUnit {
	type Err = String;

	fn from_str(string: &str) -> std::result::Result<Self, Self::Err> {
		match string.to_ascii_lowercase().as_str() {
			"day" 	| "days" 	=> Ok(Self::Day),
			"week" 	| "weeks" 	=> Ok(Self::Week),
			"month" | "months" 	=> Ok(Self::Month),
			"year" 	| "years" 	=> Ok(Self::Year),
			_ => Err(format!("unknown unit \"{}\"", string)),
		}
	}
}

#[derive(Debug)]
pub struct BdayError {
	msg: String,
}

impl fmt::Display for BdayError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.msg)
	}
}

impl Error for BdayError {}