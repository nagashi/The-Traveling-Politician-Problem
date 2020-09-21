/*
*********************************************************************************
*                                                                               *
* FILE: main.rs                                                                 *
*                                                                               *
* USAGE: redis [-h]                                                             *
*                                                                               *
* DESCRIPTION: The haversine formula, an equation important in                  *
*              navigation, is used here to determine the                        *
*              distance between state capitals, in miles using                  *
*              longitude and latitude which was obtained from                   *
*              the state's zip code.  The end result is to export               *
*              a CSV created file to a Neo4j DB where a query                   *
*              will be ran to compute the shortest distance to                  *
*              the White House.  See README file for more details.              *
*                                                                               *
* OPTIONS: List options for the script [-h]                                     *
*                                                                               *
* ERROR CONDITIONS: exit 1 ---- Invalid option                                  *
*                   exit 2 ----	Cannot find stated file/directory               *
*                   exit 3 ----	git command failed                              *
*                   exit 4 ----	Cannot change to neo4j directory                *
*                   exit 5 ----	make failed                                     *
*                   exit 6 ----	make test failed                                *
*                   exit 99 ---	killed by external forces                       *
*                                                                               *
* DEVELOPER: Charles E. O'Riley Jr.                                             *
* DEVELOPER PHONE: +1 (615) 983-1474                                            *
* DEVELOPER EMAIL: ceoriley@gmail.com                                           *
*                                                                               *
* VERSION: 0.01.0                                                               *
* CREATED DATE-TIME: 20200305-15:02 Central Time Zone USA                       *
*                                                                               *
* VERSION: 0.02.0                                                               *
* REVISION DATE-TIME: YYYYMMDD-HH:MM                                            *
* DEVELOPER MAKING CHANGE: First_name Last_name                                 *
* DEVELOPER MAKING CHANGE: PHONE: +1 (XXX) XXX-XXXX                             *
* DEVELOPER MAKING CHANGE: EMAIL: first.last@email.com                          *
* REVISION MADE:                                                                *
* REVISION DATE-TIME: 20200520-17:00                                            *
* Charles O'Riley: +1 (615) 983-1474: ceoriley@gmail.com#                       *
* REVISION MADE: Added error checking and read and write                        *
*                files from/to the base directory.                              *
* REVISION DATE-TIME: 20200603-20:05                                            *
* Charles O'Riley: +1 (615) 983-1474: ceoriley@gmail.com#                       *
* REVISION MADE: Moved the haversine_dist function to a                         *
*                library.                                                       *
* REVISION DATE-TIME: 20200604-20:25                                            *
* Charles O'Riley: +1 (615) 983-1474: ceoriley@gmail.com#                       *
* REVISION MADE: Added chrono crate to capture output.json                      *
*                file creation.                                                 *
* REVISION DATE-TIME: 20200709-20:34                                            *
* REVISION MADE: Modified Readme.md file                                        *
* REVISION DATE-TIME: 20200809-13:00                                            *
* REVISION MADE: Added logging functionality                                    *
* REVISION DATE-TIME: 20200909-13:41                                            *
* REVISION MADE: Added functionality for csv                                    *
*                file creation & modified module names.                         *
* REVISION DATE-TIME: 20200911-21:32                                            *
* Charles O'Riley: +1 (615) 983-1474: ceoriley@gmail.com#                       *
* REVISION MADE: Converted procedural code to functions and                     *
*                placed in module                                               *
* REVISION DATE-TIME: 20200920-19:57                                            *
* Charles O'Riley: +1 (615) 983-1474: ceoriley@gmail.com#                       *
* REVISION MADE: Added csv module to library. Added error                       *
*                checking functionality                                         *                                                                                  #
*********************************************************************************
*/

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate csv;
extern crate permutohedron;
extern crate read_json as rj;
extern crate serde;
extern crate serde_json;

use chrono::prelude::*;
use csv::Writer;
use log::{debug, error, info, trace, warn, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use permutohedron::Heap;
use serde_json::json;
use std::{env, f64, fs, fs::File, io::Read, string::String};

use rj::{
    csv::{path_exists, write_csv, Location},
    distance::haversine_dist as distance,
    stss::{title, vec_row},
};

#[derive(Deserialize, Debug)]
struct ObjStates {
    from_state: String,
    to_state: String,
}

#[derive(Deserialize, Debug)]
struct ObjLookUp {
    zip_code: String,
    city: String,
    state: String,
    latitude: String,
    longitude: String,
    classification: String,
    population: String,
}

const IA: &str = "IA";
const DC: &str = "DC";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start app begin time
    let start_time = Local::now().time();

    //Set up logging
    let level = log::LevelFilter::Info;
    let file_path = "log/path.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = match FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} Line:{L} {h([{l}])} - {m}{n}",
        )))
        .build(file_path)
    {
        Ok(config) => config,
        Err(error) => {
            let msg = "FAILED: creating log/path.log";
            error!("{:?}: {:?}", msg, error);
            panic!("{:?}: {:?}", msg, error)
        }
    };

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        );

    // Errorcheck for log/path.log
    let config = match config {
        Ok(config) => config,
        Err(error) => {
            let msg = "FAILED: creating log/path.log";
            error!("{:?}: {:?}", msg, error);
            panic!("{:?}: {:?}", msg, error)
        }
    };

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = match log4rs::init_config(config) {
        Ok(_handle) => _handle,
        Err(e) => {
            let msg = "Failed to initialize global logger";
            error!("{:?}: {:?}", msg, e);
            panic!("{:?}: {:?}", msg, e);
        }
    };

    /* error!("Goes to stderr and file");
    warn!("Goes to stderr and file");
    info!("Goes to stderr and file");
    debug!("Goes to file only");
    trace!("Goes to file only"); */

    // Start timing;
    info!("***** BEGIN APP: {:?} *****", start_time);

    // Read the input file to string &
    // Error(2) check for presence of file/directoery
    let mut file_states = match File::open("states.json") {
        Ok(file_states) => {
            let msg = "Success read states.json";
            info!("{:?}: {:?}", msg, file_states);
            file_states
        }
        Err(error) => {
            let msg = "Error opening file states.json";
            error!("{:?}", error);
            panic!("{:?}: {:?}", msg, error)
        }
    };

    let mut contents_states = String::new();
    let result = file_states.read_to_string(&mut contents_states);

    // Errorcheck for read_to_string(&mut contents_states)
    match result {
        Ok(content) => {
            info!("Success file_states.read_to_string content: {}", content);
        }
        Err(error) => {
            let msg = "Failed file_states.read_to_string(&mut contents_states)";
            error!("{:?}: {:?}", msg, error);
            panic!("{:?}: {:?}", msg, error)
        }
    }

    // Deserialize data structure.
    let data_states: Vec<ObjStates> = serde_json::from_str(&contents_states)?;

    // Error check for deserializing file
    match &data_states.len() > &0 {
        true => {
            let msg = "Success deserialized data_states";
            info!("{:?}: {:?}", msg, &data_states);
            &data_states;
        }
        false => {
            let msg = "Failed to deserialize data_states";
            error!("{:?}: ", msg);
            panic!("{:?}: ", msg)
        }
    };

    let from_state: &str = &data_states[0].from_state;
    let to_state: &str = &data_states[0].to_state;

    match from_state.len() > 0 && to_state.len() > 0 {
        true => {
            info!(
                "Success: from_state: {:?} to_state: {:?} ",
                from_state, to_state
            );
        }
        false => {
            let msg = "Failure to initialize from_state or to_state:";
            error!("{:?}: ", msg);
            panic!("{:?}: ", msg)
        }
    }

    // Lookup table &
    // Error(2) check for presence of file/directoery
    let mut file_look_up = match File::open("look_up.json") {
        Ok(file_look_up) => {
            info!("Success initialized file_look_up: {:?}", file_look_up);
            file_look_up
        }
        Err(error) => {
            let msg = "There was a problem opening file lookup.json";
            error!("{:?}: {:?}", msg, error);
            panic!("{:?}: {:?}", msg, error)
        }
    };

    let mut contents_look_up = String::new();
    let result = file_look_up.read_to_string(&mut contents_look_up);

    // Errorcheck for read_to_string(&mut contents_look_up)
    match result {
        Ok(content) => {
            info!("Success file_look_up.read_to_string content: {}", content);
        }
        Err(error) => {
            let msg = "Failed file_look_up.read_to_string(&mut contents_look_up)";
            error!("{:?}: {:?}", msg, error);
            panic!("{:?}: {:?}", msg, error)
        }
    }

    // Error check for deserializing file
    let data_look_up: Vec<ObjLookUp> = match serde_json::from_str(&contents_look_up) {
        Ok(data_look_up) => {
            let msg = "Success deserialized data_look_up";
            info!("{:?}", msg);
            data_look_up
        }
        Err(error) => {
            let msg = "Failed to deserialize data_look_up";
            error!("{:?}: {:?}", msg, error);
            panic!("{:?}: {:?}", msg, error);
        }
    };

    let num = || data_look_up.len();

    // Initialize these values so that they'll be
    // in scope for the haversine_dist function
    let mut lon1: f64 = 1.0;
    let mut lat1: f64 = 1.0;
    let mut lon2: f64 = 1.0;
    let mut lat2: f64 = 1.0;
    let mut from_zipcode: String = "0".to_string();
    let mut to_zipcode: String = "0".to_string();
    info!("Initialize variables for output.json");

    // Loop through lookup file to verify input.
    for x in 0..num() {
        match &data_look_up[x].state {
            // unreachable pattern so impose a guard
            a if a == from_state => {
                lon1 = match data_look_up[x].longitude.parse::<f64>() {
                    Ok(lon1) => lon1,
                    Err(e) => {
                        let msg = "Error converting lon1 to f64";
                        error!("{:?}: {:?}", msg, e);
                        panic!("{:?}: {:?}", msg, e);
                    }
                };

                lat1 = match data_look_up[x].latitude.parse::<f64>() {
                    Ok(lat1) => lat1,
                    Err(e) => {
                        let msg = "Error converting lat1 to f64";
                        error!("{:?}: {:?}", msg, e);
                        panic!("{:?}: {:?}", msg, e);
                    }
                };

                from_zipcode = match data_look_up[x].zip_code.parse::<String>() {
                    Ok(from_zipcode) => from_zipcode,
                    Err(e) => {
                        let msg = "Error converting from_zipcode to String";
                        error!("{:?}: {:?}", msg, e);
                        panic!("{:?}: {:?}", msg, e);
                    }
                };
            }
            // unreachable pattern so impose a guard
            b if b == to_state => {
                lon2 = match data_look_up[x].longitude.parse::<f64>() {
                    Ok(lon2) => lon2,
                    Err(e) => {
                        let msg = "Error converting _lon2 to f64";
                        error!("{:?}: {:?}", msg, e);
                        panic!("{:?}: {:?}", msg, e);
                    }
                };
                lat2 = match data_look_up[x].latitude.parse::<f64>() {
                    Ok(lat2) => lat2,
                    Err(e) => {
                        let msg = "Error converting lat2 to f64";
                        error!("{:?}: {:?}", msg, e);
                        panic!("{:?}: {:?}", msg, e);
                    }
                };
                to_zipcode = match data_look_up[x].zip_code.parse::<String>() {
                    Ok(to_zipcode) => to_zipcode,
                    Err(e) => {
                        let msg = "Error converting to_zipcode to String";
                        error!("{:?}: {:?}", msg, e);
                        panic!("{:?}: {:?}", msg, e);
                    }
                };
            }
            _ => {}
        }
    }

    let d: f64 = distance(lat1, lon1, lat2, lon2); // mod function
    info!(
        "json object: Distance from {} to {}: {:.1} mi ",
        from_state, to_state, d
    );

    let dt = format!("{}", Local::now().format("%a %b %e %T %Y"));

    let obj = json!({
        "beginning_state":from_state.to_string(),
        "beginning_zipcode":from_zipcode.to_string(),
        "ending_state":to_state.to_string(),
        "ending_zipcode":to_zipcode.to_string(),
        "miles_between":d.to_string(),
        "time_created":dt
    });
    trace!("Initialize json object: {:?}", &obj);

    // Write output to file & error check
    let _f = fs::write(
        "output.json",
        match serde_json::to_string_pretty(&obj) {
            Ok(file) => {
                info!("Success writing file output.json {:?}", file);
                file
            }
            Err(e) => {
                let msg = "Error writing file output.json";
                warn!("{:?}: {:?}", msg, e);
                error!("{:?}: {:?}", msg, e);
                panic!("{:?}: {:?}", msg, e)
            }
        },
    );

    // Haversine is finished
    // Permutation begins

    let num = || data_look_up.len() - 46; // Don't allow all 51 entries to be permutated.
    let mut data = Vec::with_capacity(num());
    debug!(
        "Number of states to iterate through w/o IA & DC: {:?}",
        num()
    );

    for x in 0..num() {
        match data_look_up[x].state != IA && // omit Iowa && DC
                data_look_up[x].state != DC
        {
            true => data.push(&data_look_up[x].state),
            false => {}
        }
    }

    let heap = Heap::new(&mut data);
    let mut perm: Vec<&str> = Vec::new();
    let data_len = data_look_up.len();

    let mut iv: isize = 0;

    for data in heap {
        info!("Begin outer loop for heap");
        let mut _lat1: f64 = 0.0;
        let mut _lon1: f64 = 0.0;
        let mut _lat2: f64 = 0.0;
        let mut _lon2: f64 = 0.0;

        let mut sum: f64 = 0.0;

        let nbr = data.len();

        // traverse each vector item
        for x in 0..nbr {
            let _c = data[x];
            debug!(
                "Begin heap inner loop - x: {:?}, nbr: {:?}, _c: {:?}",
                x, nbr, _c
            );

            // match the position of
            // each vector item
            match x {
                0 => {
                    sum = 0.0;
                    // empty vector. clear previous entries
                    perm.clear();
                    let mut a = vec![IA]; // IA is first entry
                    perm.append(&mut a);
                    perm.push(_c); // pump in next state
                }
                a if a == (nbr - 1) => {
                    perm.push(_c); // at vector end, pump in
                                   // next state and then DC
                    let mut b = vec![DC];
                    perm.append(&mut b);
                }
                _ => {
                    // else, push in states
                    perm.push(_c);
                }
            }

            // next step is to lookup the
            // longitute and latitude of eadh
            // state in the vector to compute
            // final distance
            match perm[perm.len() - 1] {
                DC => {
                    let numbr = perm.len();

                    for i in 1..numbr {
                        match i {
                            // match guard for
                            // the expression
                            a if a > 0 => {
                                for ii in 0..data_len {
                                    match &data_look_up[ii].state {
                                        // Shift left to get previous value.
                                        a if a == perm[i - 1] => {
                                            // extract & error check longitude & latitude
                                            _lon1 = match data_look_up[ii].longitude.parse::<f64>()
                                            {
                                                Ok(_lon1) => _lon1,
                                                Err(e) => {
                                                    let msg = "Error converting _lon1 to f64";
                                                    error!("{:?}: {:?}", msg, e);
                                                    panic!("{:?}: {:?}", msg, e);
                                                }
                                            };

                                            _lat1 = match data_look_up[ii].latitude.parse::<f64>() {
                                                Ok(_lat1) => _lat1,
                                                Err(e) => {
                                                    let msg = "Error converting _lat1 to f64";
                                                    error!("{:?}: {:?}", msg, e);
                                                    panic!("{:?}: {:?}", msg, e);
                                                }
                                            };
                                        }
                                        b if b == perm[i] => {
                                            _lon2 = match data_look_up[ii].longitude.parse::<f64>()
                                            {
                                                Ok(_lon2) => _lon2,
                                                Err(e) => {
                                                    let msg = "Error converting _lon2 to f64";
                                                    error!("{:?}: {:?}", msg, e);
                                                    panic!("{:?}: {:?}", msg, e);
                                                }
                                            };
                                            _lat2 = match data_look_up[ii].latitude.parse::<f64>() {
                                                Ok(_lat2) => _lat2,
                                                Err(e) => {
                                                    let msg = "Error converting _lat2 to f64";
                                                    error!("{:?}: {:?}", msg, e);
                                                    panic!("{:?}: {:?}", msg, e);
                                                }
                                            };
                                        }
                                        _ => {}
                                    }
                                }

                                let d2 = distance(_lat1, _lon1, _lat2, _lon2); // mod function
                                sum += d2; // sum up distance from one state to the next
                                           // within the vector
                                sum = (sum * 10.0).round() / 10.0; // compute to 1 digit

                                debug!(
                                    "#{:?} states1: {:?} states2: {:?} d2 {:?}  sum: {:?}",
                                    i,
                                    perm[i - 1],
                                    perm[i],
                                    d2,
                                    sum
                                );
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        {
            // obtain the current path which will be
            // used to push data into a csv file
            // to import into Neo4j
            let mut path_csv = match env::current_dir() {
                Ok(dir) => {
                    let path_csv = match dir.into_os_string().into_string() {
                        Ok(path_csv) => path_csv,
                        Err(error) => {
                            let msg = "Error converting path to string";
                            error!("{:?}: {:?}", msg, error);
                            panic!("{:?}: {:?}", msg, error);
                        }
                    };

                    path_csv
                }
                Err(error) => {
                    let msg = "Error finding path";
                    error!("{:?}: {:?}", msg, error);
                    panic!("{:?}: {:?}", msg, error);
                }
            };

            // append csv file to path.
            path_csv.push_str("/cypher.csv");

            // determine if the file exists
            let boolean = path_exists(path_csv.as_str()); // mod function

            let loc = Location {
                path: path_csv,
                boolean: boolean,
                cnt: iv,
            };

            let file = write_csv(loc); // mod function

            let mut wtr = Writer::from_writer(file);

            iv += 1;

            let vec = vec_row(iv, sum, perm.to_owned()); // mod function
            let vec_len = vec.len();

            match iv {
                1 => {
                    let header: Vec<String> = title(vec_len); //mod function

                    if let Err(e) = wtr.write_record(header) {
                        error!("Could not write header to CSV file: {:?}", e);
                        panic!("Could not write row to CSV file: {:?}", e);
                    }
                }
                _ => {}
            }

            if let Err(e) = wtr.write_record(vec) {
                error!("Could not write row to CSV file: {:?}", e);
                panic!("Could not write row to CSV file: {:?}", e);
            }
        }
    }

    // compute time for program to run
    let end_time = Local::now().time();
    let diff = end_time - start_time;
    let microsec = diff.num_microseconds().unwrap();

    // End timing
    info!("***** END APP: {:?} *****", end_time);

    info!(
        "Total run time: {:?} hour(s), {:?} minute(s), {:?} second(s), {:?} millisecond(s), {:?} microsecond(s)",
        diff.num_hours(),
        diff.num_minutes(),
        diff.num_seconds(),
        diff.num_milliseconds(),
        microsec);

    // catch any '?' try_catch errors.
    Ok(())
}
