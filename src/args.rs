use clap::{App, Arg};
use std::env;

//collect everything into struct so we don't need to keep changing return values if we change the options
pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("pipeviewer")
            .version("0.1")
            .author("George Li <georgejdli@gmail.com")
            .arg(Arg::with_name("infile").help("Read from a file instad of stdin"))
            .arg(
                Arg::with_name("outfile")
                    .short("o")
                    .long("outfile")
                    .takes_value(true)
                    .help("Write output to file instead of stdout"),
            )
            .arg(
                Arg::with_name("silent")
                    .short("s")
                    .long("silent")
                    .help("Suppress total bytes read"),
            )
            .get_matches();
        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();
        let silent = if matches.is_present("silent") {
            true
        } else {
            !env::var("PV_SILENT").unwrap_or_default().is_empty()
        };
        Self {
            infile,
            outfile,
            silent,
        }
    }
}
