/*
*********************************************************************************
*                                                                               *
* FILE: main.rs                                                                 *
*                                                                               *
* USAGE: redis [-h]                                                             *
*                                                                               *
* DESCRIPTION: The haversine formula, an equation important in                  *
*              navigation, is used here to determine the                        *
*              distance between states in miles using                           *
*              longitude and latitude.                                          *
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
* REVISION MADE: Added logging functionality                                    *                                                                                #
*********************************************************************************
*/

#[macro_use]
extern crate serde_derive;
extern crate permutohedron;
extern crate serde;
extern crate serde_json;

extern crate read_json as rj;

extern crate chrono;

use chrono::prelude::*;
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
use std::{f64, fs, fs::File, io::Read, string::String};

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
    
    // Begin app begin time
    let start_time = Local::now().time();

    //Set up logging
    let level = log::LevelFilter::Info;
    let file_path = "log/path.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} Line:{L} {h([{l}])} - {m}{n}",
        )))
        .build(file_path)
        .unwrap();

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
        //.unwrap(); Commented due to
        // implementation of error check

        // Errorcheck for log/path.log
    let config = match config {
        Ok(config) => {
            info!("SUCCESS created log/path.log: {:?}", config);
            config
        } // Use content when implementing logging.
        Err(error) => {
            let msg = "FAILED created log/path.log";
            error!("{:?}: {:?}", msg, error);
            panic!("{:?}: {:?}", msg, error)
        }
    };

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;

    //error!("Goes to stderr and file");
    //warn!("Goes to stderr and file");
    //info!("Goes to stderr and file");
    //debug!("Goes to file only");
    //trace!("Goes to file only");

    // Start timing;
    info!("***** BEGIN APP: {:?} *****", start_time);

    // Read the input file to string.
    let file_states = File::open("states.json");

    // Error(2) check for presence of file/directoery
    let mut file_states = match file_states {
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
        } // Use content when implementing logging.
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

    match from_state.len() > 0 {
        true => {
            info!("Success initialized from_state: {:?}", from_state);
        }
        false => {
            let msg = "Failed to initialize from_state:";
            error!("{:?}: ", msg);
            panic!("{:?}: ", msg)
        }
    }

    match to_state.len() > 0 {
        true => {
            info!("Success initialized to_state: {:?}", to_state);
        }
        false => {
            let msg = "Failed to initialize to_state:";
            error!("{:?}: ", msg);
            panic!("{:?}: ", msg)
        }
    }

    // Lookup table
    let file_look_up = File::open("look_up.json");

    // Error(2) check for presence of file/directoery
    let mut file_look_up = match file_look_up {
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

    // Deserialize and print Rust data structure.
    let data_look_up: Vec<ObjLookUp> = serde_json::from_str(&contents_look_up).unwrap();

    // Error check for deserializing file
    match &data_look_up.len() > &0 {
        true => {
            &data_look_up;
            let msg = "Success deserialized data_look_up";
            info!("{:?}", msg);
            &data_look_up
        }
        false => {
            let msg = "Failed to deserialize data_look_up";
            error!("{:?}: ", msg);
            panic!("{:?}: ", msg)
        }
    };

    let num = data_look_up.len();

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
    for x in 0..num {
        if data_look_up[x].state == from_state {
            lon1 = data_look_up[x].longitude.parse().unwrap();
            lat1 = data_look_up[x].latitude.parse().unwrap();
            from_zipcode = data_look_up[x].zip_code.parse().unwrap();
        }
        if data_look_up[x].state == to_state {
            lon2 = data_look_up[x].longitude.parse().unwrap();
            lat2 = data_look_up[x].latitude.parse().unwrap();
            to_zipcode = data_look_up[x].zip_code.parse().unwrap();
        }
    }

    let d: f64 = rj::gap::haversine_dist(lat1, lon1, lat2, lon2);
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

    // Write output to file.
    let f = fs::write("output.json", serde_json::to_string_pretty(&obj).unwrap());
    // Error check for writing file
    match f {
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
    };

    // Haversine is finished
    // Permutation begins

    let num = data_look_up.len() - 45; // Don't allow all 51 entries to be permutated.
    let mut data = Vec::with_capacity(num);
    info!("Number of states to iterate through w/o IA & DC: {:?}", num);

    for x in 0..num {
        if data_look_up[x].state != IA && // omit Iowa && DC
            data_look_up[x].state != DC
        {
            data.push(&data_look_up[x].state);
        }
    }

    let heap = Heap::new(&mut data);
    let mut perm = Vec::new();
    let data_len = data_look_up.len();

    let mut iv: i128 = 0;

    for data in heap {
        info!("Begin outer loop for heap");
        let mut _lat1: f64 = 0.0;
        let mut _lon1: f64 = 0.0;
        let mut _lat2: f64 = 0.0;
        let mut _lon2: f64 = 0.0;

        let mut sum: f64 = 0.0;

        let nbr = data.len();
        for x in 0..nbr {
            let _c = data[x];
            debug!(
                "Begin heap inner loop - x: {:?}, nbr: {:?}, _c: {:?}",
                x, nbr, _c
            );

            if x == 0 {
                sum = 0.0;
                // new vector. clear past entries
                perm.clear();
                let mut a = vec![IA];
                perm.append(&mut a);
                perm.push(_c);
            //debug!("perm x == 0: {:?}", perm);
            } else if x == nbr - 1 {
                perm.push(_c);
                let mut b = vec![DC];
                perm.append(&mut b);
            //debug!("perm x == nbr - 1: {:?}", perm);
            } else {
                perm.push(_c);
                //debug!("perm else: {:?}", perm);
            }

            if perm[perm.len() - 1] == DC {
                let numbr = perm.len();

                for i in 1..numbr {
                    if i > 0 {
                        for ii in 0..data_len {
                            if data_look_up[ii].state == perm[i - 1] {
                                _lon1 = data_look_up[ii].longitude.parse().unwrap();
                                _lat1 = data_look_up[ii].latitude.parse().unwrap();
                            }

                            if data_look_up[ii].state == perm[i] {
                                _lon2 = data_look_up[ii].longitude.parse().unwrap();
                                _lat2 = data_look_up[ii].latitude.parse().unwrap();
                            }
                        }

                        let d2 = rj::gap::haversine_dist(_lat1, _lon1, _lat2, _lon2);
                        sum += d2;
                        sum = (sum * 10.0).round() / 10.0;

                        debug!(
                            "#{:?} states1: {:?} states2: {:?} d2 {:?}  sum: {:?}",
                            i,
                            perm[i - 1],
                            perm[i],
                            d2,
                            sum
                        );
                    }
                }
            }
        }

        {
            iv += 1;
            let s = format!("{:.1}", sum);
            let s: &'static str = rj::gap::string_to_static_str(s);
            let mut s = vec![s];
            perm.append(&mut s);

            debug!("{:?}) {:?}", iv, &perm);
            //debug!("{:?}) {:?}   {:?}", iv, &data, &perm);
        }
    }

    // compute time for program to run
    let end_time = Local::now().time();
    let diff = end_time - start_time;
    let microsec = diff.num_microseconds().unwrap();

    // End timing
    info!("***** END APP: {:?} *****", end_time);

    info!("diff {:?}", diff.num_seconds());

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
