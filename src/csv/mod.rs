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
    match (loc.cnt, loc.boolean) {
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
                    let msg = "file truncate error";
                    error!("{}: {}: {}: {}", msg, loc.cnt, loc.boolean, e);
                    panic!("{}: {}: {}: {}", msg, loc.cnt, loc.boolean, e)
                }
            };

            // After truncating, set file variable
            // to append writing.
            let file = match OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(&loc.path)
            {
                Ok(file) => file,
                Err(e) => {
                    let msg = "file append error";
                    error!("{}: {}: {}: {}", msg, loc.cnt, loc.boolean, e);
                    panic!("{}: {}: {}: {}", msg, loc.cnt, loc.boolean, e)
                }
            };
            return file;
        }
        (.., true) => {
            // count > 0 and file exist (true)
            // then continue append writing.
            let file = match OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(loc.path)
            {
                Ok(file) => file,
                Err(e) => {
                    let msg = "file append error";
                    error!("{}: {}: {}: {}", msg, loc.cnt, loc.boolean, e);
                    panic!("{}: {}: {}: {}", msg, loc.cnt, loc.boolean, e)
                }
            };
            return file;
        }
        (.., false) => {
            // file doesn't exist, create
            // & set file properties to
            // append writing.
            let file = match OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(loc.path)
            {
                Ok(file) => file,
                Err(e) => {
                    let msg = "file append error";
                    error!("{}: {}: {}: {}", msg, loc.cnt, loc.boolean, e);
                    panic!("{}: {}: {}: {}", msg, loc.cnt, loc.boolean, e)
                }
            };
            return file;
        }
    };
}
