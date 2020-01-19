use super::utils::Arguments;

use clap::load_yaml;
use clap::App;
use std::fs::create_dir_all;
use std::io::{
	Error as ioError,
	ErrorKind,
};
use std::path::Path;

/// Setup clap-arguments
pub fn setup_args() -> Result<Arguments, ioError> {
	let yml = load_yaml!("../cli.yml");
	let cli_matches = App::from_yaml(yml).get_matches();
	let mut args = Arguments {
		out:        cli_matches.value_of("out").unwrap().to_owned(), // unwrap, because of a set default
		tmp:        cli_matches.value_of("tmp").unwrap().to_owned(), // unwrap, because of a set default
		url:        cli_matches.value_of("URL").unwrap().to_owned(), // unwrap, because "URL" is required
		tmp_sub:    cli_matches.value_of("tmpcreate").unwrap().to_owned(), // unwrap, because of a set default
		audio_only: cli_matches.is_present("audio_only"),
		debug:      cli_matches.is_present("debug"),
		extra_args: cli_matches
			.values_of("ytdlargs") // get all values after "--"
			.map(|v| return v.collect::<Vec<&str>>()) // because "clap::Values" is an iterator, collect it all as Vec<&str>
			.unwrap_or(Vec::new()) // unwrap the Option<Vec<&str>> or create a bew Vec
			.iter() // Convert the Vec<&str> to an iterator
			.map(|v| return String::from(*v)) // Map every value to String (de-referencing because otherwise it would be "&&str")
			.collect(), // Collect it again as Vec<String>
	};

	args.extra_args.push("--write-thumbnail".to_owned());
	args.tmp = match args.tmp_sub.as_ref() {
		"true" => {
			let lepath = Path::new(&args.tmp).join("rust-yt-dl");

			create_dir_all(&lepath).expect("Couldnt create tmpsub directory");

			lepath
				.canonicalize()
				.expect("failed to canonicalize a path")
				.to_str()
				.expect("failed to parse a PathBuf to a string")
				.to_owned()
		},
		"false" => Path::new(&args.tmp)
			.canonicalize()
			.expect("failed to canonicalize a path")
			.to_str()
			.expect("failed to parse a PathBuf to a string")
			.to_owned(),
		_ => return Err(ioError::new(ErrorKind::Other, "Invalid tmpcreate value!")),
	};

	return Ok(args);
}