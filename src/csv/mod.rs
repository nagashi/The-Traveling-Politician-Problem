use log::error;
use std::{fs, fs::OpenOptions};

pub struct Location<'a> {
    pub path: &'a String,
    pub boolean: bool,
    pub cnt: usize,
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn write_csv(loc: Location) -> std::fs::File {
    match (&loc.cnt, &loc.boolean) {
        (0, false) => {
            // first time file writing
            // and the file does not
            // exists so create it
            let file = match OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(&loc.path)
            {
                Ok(f) => f,
                Err(e) => {
                    let msg = "cypher.csv creation error";
                    error!("{}: {}: {}: {}", msg, &loc.cnt, &loc.boolean, e);
                    panic!("{}: {}: {}: {}", msg, &loc.cnt, &loc.boolean, e)
                }
            };
            return file;
        }
        (0, true) => {
            // if this is the first time file
            // writing and the file exists then
            // truncate it.
            let _ = match OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&loc.path)
            {
                Ok(f) => f,
                Err(e) => {
                    let msg = "cypher.csv truncate error";
                    error!("{}: {}: {}: {}", msg, &loc.cnt, &loc.boolean, e);
                    panic!("{}: {}: {}: {}", msg, &loc.cnt, &loc.boolean, e)
                }
            };

            // After truncating, set file variable
            // to append writing.
            let file = match OpenOptions::new().write(true).append(true).open(&loc.path) {
                Ok(file) => file,
                Err(e) => {
                    let msg = "cypher.csv append after truncating error";
                    error!("{}: {}: {}: {}", msg, &loc.cnt, &loc.boolean, e);
                    panic!("{}: {}: {}: {}", msg, &loc.cnt, &loc.boolean, e)
                }
            };
            return file;
        }
        (.., _) => {
            // row > 0 so it does
            // not matter whether file
            // exist or not: just append writing.
            let file = match OpenOptions::new().write(true).append(true).open(loc.path) {
                Ok(file) => file,
                Err(e) => {
                    let msg = "cypher.csv row > 0 append error";
                    error!("{}: {}: {}: {}", msg, &loc.cnt, &loc.boolean, e);
                    panic!("{}: {}: {}: {}", msg, &loc.cnt, &loc.boolean, e)
                }
            };
            return file;
        }
    };
}
