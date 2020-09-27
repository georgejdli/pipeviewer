use ::std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

// 16 byte size buffer
const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        // debug macro better than straight println and having to format the string
        // not for logging!!! don't leave these in actual production code
        //dbg!(silent);
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break, //most specific match first, end of file, more more bytes to read
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            // return to start of the line to it overwrites prevous output, to show progress cleanly
            // not using new "ln"
            eprint!("\r{}", total_bytes);
        }

        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
            // eprintln!("Oh no, an error! {}", e.to_string());
            // std::process::exit(1)
        }
    }
    if !silent {
        eprintln!("\r{}", total_bytes);
    }
    Ok(())
}
