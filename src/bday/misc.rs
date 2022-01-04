use date_time::date_tuple::Date;
use super::BdayError;
use std::str::FromStr;

/// Replace newlines, tabs, and carriage returns with spaces.
pub(super) fn sanitize(input: &mut str) {
	for mut c in input.chars() {
		if c == '\n' || c == '\t' || c == '\r' { c = ' ' }
	}
}

/// Parse a date in day/month/year orderm while allowing month names to be used.
pub(super) fn parse_date(input: &str) -> Result<Date, BdayError> {
	let tokens: Vec<&str> = input.split_whitespace().collect();
	
	if tokens.len() != 3 {
		return Err(BdayError {msg: format!("Invalid date: {}", input)});
	}
	
	let day = u8::from_str(tokens[0]).map_err(|_| BdayError {
		msg: format!("Invalid day in date: {}", input)
	})?;
	
	let year = u16::from_str(tokens[2]).map_err(|_| BdayError {
		msg: format!("Invalid year in date: {}", input)
	})?;
	
	// Try to parse the month as an integer, then try parsing it as a name.
	let month = match u8::from_str(tokens[1]) {
		Ok(n) => n,
		Err(_) => {
			match tokens[1].to_lowercase().as_str() {
				"jan" | "january"	=>  1,
				"feb" | "febuary"	=>  2,
				"mar" | "march"		=>  3,
				"apr" | "april"		=>  4,
				"may"				=>  5,
				"jun" | "june"		=>  6,
				"jul" | "july"		=>  7,
				"aug" | "august"	=>  8,
				"sep" | "september"	=>  9,
				"oct" | "october"	=> 10,
				"nov" | "november"	=> 11,
				"dec" | "december"	=> 12,
				_ => return Err(BdayError {
					msg: format!("Invalid month in date: {}", input)
				})
			}
		}
	};
	
	Date::new(year, month, day).map_err(|_| BdayError {
		msg: format!("Invalid date: {}", input)
	})
}