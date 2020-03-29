use clap::{App, Arg};
use std::{fs, io};

fn main() -> io::Result<()> {
    let folder = parse_args();
    let mut entries = fs::read_dir(&folder)?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    for entry in entries {
        println!("{:?}", entry);
    }

    Ok(())
}

fn parse_args() -> String {
    App::new("Lumiere")
        .arg(
            Arg::with_name("folder")
                .help("Select the folder to use")
                .required(true),
        )
        .get_matches()
        .value_of("folder")
        .unwrap()
        .to_owned()
}
