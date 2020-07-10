/*
*********************************************************************************
*                                                                               *
* FILE: main.rs								                                    *
*										                                        *
* USAGE: redis [-h] 								                            *
*										                                        *
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
* REVISION MADE: Modified Readme.md file                                        *                                                                                #
*********************************************************************************
*/

#[macro_use]
extern crate serde_derive;
extern crate permutohedron;
extern crate serde;
extern crate serde_json;

extern crate read_json as rj;

extern crate chrono;

use serde_json::json;

use std::{error::Error, f64, fs, fs::File, io::Read, string::String};

use chrono::prelude::*;

use permutohedron::Heap;

#[derive(Deserialize, Debug)]
struct ObjStates {
    from_state: String,
    to_state: String,
}

#[allow(dead_code)]
struct Coordinate<'a> {
    from_latitude: &'a f64,
    from_longitude: &'a f64,
    to_latitude: &'a f64,
    to_longitude: &'a f64,
    distance: &'a f64,
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

fn main() -> Result<(), Box<dyn Error>> {
    // Read the input file to string.
    let file_states = File::open("states.json");

    // Error(2) check for presence of file/directoery
    let mut file_states = match file_states {
        Ok(file) => file,
        Err(error) => {
            let msg = "There was a problem opening file states.json";
            panic!("{:?}: {:?}", msg, error)
        }
    };

    let mut contents_states = String::new();
    let result = file_states.read_to_string(&mut contents_states);

    // Errorcheck for read_to_string(&mut contents_states)
    match result {
        Ok(content) => {
            println!("Success file_states.read_to_string content: {}", content);
        } // Use content when implementing logging.
        Err(error) => {
            let msg = "Failed file_states.read_to_string(&mut contents_states)";
            panic!("{:?}: {:?}", msg, error)
        }
    }

    // Deserialize data structure.
    let data_states: Vec<ObjStates> = serde_json::from_str(&contents_states)?;

    // Error check for deserializing file
    match &data_states.len() > &0 {
        true => {
            &data_states;
            let msg = "Successfully deserialized data_states";
            println!("\n{:?}: ", msg);
        }
        false => {
            let msg = "Failed to deserialize data_states";
            panic!("{:?}: ", msg)
        }
    };

    let from_state: &str = &data_states[0].from_state;
    let to_state: &str = &data_states[0].to_state;

    // Lookup table
    let file_look_up = File::open("look_up.json");

    // Error(2) check for presence of file/directoery
    let mut file_look_up = match file_look_up {
        Ok(file) => file,
        Err(error) => {
            let msg = "There was a problem opening file lookup.json";
            panic!("{:?}: {:?}", msg, error)
        }
    };

    let mut contents_look_up = String::new();
    let result = file_look_up.read_to_string(&mut contents_look_up);

    // Errorcheck for read_to_string(&mut contents_look_up)
    match result {
        Ok(content) => {
            println!("Success file_look_up.read_to_string content: {}", content);
        } // Use content when implementing logging.
        Err(error) => {
            let msg = "Failed file_look_up.read_to_string(&mut contents_look_up)";
            panic!("{:?}: {:?}", msg, error)
        }
    }

    // Deserialize and print Rust data structure.
    let data_look_up: Vec<ObjLookUp> = serde_json::from_str(&contents_look_up)?;

    // Error check for deserializing file
    match &data_look_up.len() > &0 {
        true => {
            &data_look_up;
            let msg = "Successfully deserialized data_look_up";
            println!("\n{:?}: ", msg);
        }
        false => {
            let msg = "Failed to deserialize data_look_up";
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
    println!(
        "Distance from {} to {}: {:.1} mi \n",
        from_state, to_state, d
    );

    let local: DateTime<Local> = Local::now();

    let obj = json!({
        "beginning_state":from_state.to_string(),
        "beginning_zipcode":from_zipcode.to_string(),
        "ending_state":to_state.to_string(),
        "ending_zipcode":to_zipcode.to_string(),
        "miles_between":d.to_string(),
        "time_created":local
    });

    // Write output to file.
    let f = fs::write("output.json", serde_json::to_string_pretty(&obj).unwrap());
    // Error check for writing file
    match f {
        Ok(file) => file,
        Err(e) => {
            let msg = "There was a problem writing file output.json";
            panic!("{:?}: {:?}", msg, e)
        }
    };

    // Haversine is finished
    // Permutation begins

    let num = data_look_up.len() - 47; // Don't allow all 51 entries to be permutated.
    let mut data = Vec::with_capacity(num);

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
        let mut _lat1: f64 = 0.0;
        let mut _lon1: f64 = 0.0;
        let mut _lat2: f64 = 0.0;
        let mut _lon2: f64 = 0.0;

        let mut sum: f64 = 0.0;

        let nbr = data.len();
        for x in 0..nbr {
            let _c = data[x];

            if x == 0 {
                sum = 0.0;
                perm.clear();
                let mut a = vec![IA];
                perm.append(&mut a);
                perm.push(_c);                     
            } else if x == nbr - 1 {
                perm.push(_c);
                let mut b = vec![DC];
                perm.append(&mut b);
            } else {
                perm.push(_c);
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

                        println!(
                            "\n #{:?} states1: {:?} states2: {:?} d2 {:?}  sum: {:?}\n",
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

            println!("{:?}) {:?}   {:?}\n", iv, &data, &perm);
        }
    }
    // catch any '?' try_catch errors.
    Ok(())
}
