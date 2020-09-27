use ::std::env;
use std::io::{self, Read, Write};

// 16 byte size buffer
const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        // debug macro better than straight println and having to format the string
        // not for logging!!! don't leave these in actual production code
        //dbg!(silent);
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break, //most specific match first, end of file, more more bytes to read
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }
    if !silent {
        eprintln!("num_read: {}", total_bytes);
    }
}
