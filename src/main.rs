/*
#*******************************************************************************#
#										                                        #
# FILE: main.rs								                                    #
#										                                        #
# USAGE: redis.sh [-h] 								                            #
#										                                        #
# DESCRIPTION: The haversine formula, an equation important in		            #
#              navigation, is used here to determine the                        #
#              distance between zip codes in miles.                             #
#										                                        #
# OPTIONS: List options for the script [-h]					                    #
#										                                        #
# ERROR CONDITIONS: exit 1 ---- Invalid option					                #
#                   exit 2 ----	Cannot find stated directory                    #
#                   exit 3 ----	git command failed				                #
#                   exit 4 ----	Cannot change to redis directory		        #
#                   exit 5 ----	make failed					                    #
#                   exit 6 ----	make test failed				                #
#                   exit 99 ---	killed by external forces			            #
#										                                        #
# DEVELOPER: Charles E. O'Riley Jr.							                    #
# DEVELOPER PHONE: +1 (615) 983-1474						                    #
# DEVELOPER EMAIL: ceoriley@gmail.com 					                        #
#										                                        #
# VERSION: 0.01.0								                                #
# CREATED DATE-TIME: 20200305-15:02 Central Time Zone USA			            #
#										                                        #
# VERSION: 0.1.0								                                #
# REVISION DATE-TIME: YYYYMMDD-HH:MM						                    #
# DEVELOPER MAKING CHANGE: First_name Last_name					                #
# DEVELOPER MAKING CHANGE: PHONE: +1 (XXX) XXX-XXXX				                #
# DEVELOPER MAKING CHANGE: EMAIL: first.last@email.com				            #
#                                                                               #
#*******************************************************************************#
#
*/

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate permutohedron;

use serde_json::json;

use std::{
    f64,
    error::Error,
    fs,
    fs::File,
    io::{Read},
    string::String    
};

use permutohedron::Heap;

#[derive(Deserialize, Debug)]
struct ObjStates {
    from_state: String,
    to_state: String
}

#[derive(Deserialize, Debug)]
struct ObjLookUp {
    zip_code: String,
    city: String,
    state: String,
    latitude: String,
    longitude: String,
    classification: String,
    population: String
}

// single, non mutable, precise memory address for R i.e. (static R: f64)
static R: f64 = 6371.0; // Earth radius in kilometers
 
fn haversine_dist(mut th1: f64, mut ph1: f64, mut th2: f64, ph2: f64) -> f64 {
    ph1 -= ph2;
    ph1 = ph1.to_radians();
    th1 = th1.to_radians();
    th2 = th2.to_radians();
    let dz: f64 = th1.sin() - th2.sin();
    let dx: f64 = ph1.cos() * th1.cos() - th2.cos();
    let dy: f64 = ph1.sin() * th1.cos();
    ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R
}

fn main() {    
    try_main().unwrap();    
}

fn try_main() -> Result<(), Box<dyn Error>> {
    // Read the input file to string.
    let mut file_states = File::open("src/states.json")?;
    let mut contents_states = String::new();
    file_states.read_to_string(&mut contents_states)?;
    
    // Deserialize and print Rust data structure.
    let data_states: Vec<ObjStates> = serde_json::from_str(&contents_states)?;

    let from_state: &str = &data_states[0].from_state;
    let to_state: &str = &data_states[0].to_state;
    
    // Lookup table
    let mut file_look_up = File::open("src/look_up.json")?;
    let mut contents_look_up = String::new();
        file_look_up.read_to_string(&mut contents_look_up)?;

    // Deserialize and print Rust data structure.
    let data_look_up: Vec<ObjLookUp> = serde_json::from_str(&contents_look_up)?;
    
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
            println!("\n{} IA-lat1: {:#?} IA-lon1: {:#?}\n", x, lat1, lon1);
        } 
        if data_look_up[x].state == to_state {
            lon2 = data_look_up[x].longitude.parse().unwrap();
            lat2 = data_look_up[x].latitude.parse().unwrap();
            to_zipcode = data_look_up[x].zip_code.parse().unwrap();
            println!("{} DC-lat2: {:#?} DC-lon2: {:#?}\n", x, lat2, lon2);
        }
        
    }

    let km_to_mi: f64 = 1.609344;     
    let d: f64 = haversine_dist(lat1, lon1, lat2, lon2);
    println!("Distance from {} to {}: {:.1} km, {:.1} mi \n", from_state, to_state, d, d / km_to_mi);
    
    // Compute to 1 digit after decimal point
    let dist = ( ( (d/km_to_mi)  * 10.0).round() / 10.0).to_string();

    let obj = json!({
        "from_state":from_state.to_string(),
        "from_zipcode":from_zipcode,
        "to_state":to_state.to_string(),
        "to_zipcode":to_zipcode,
        "distance":dist,
    });

    // Write output to file.
    fs::write("src/output.json", 
            serde_json::to_string_pretty(&obj).unwrap()).ok();

    // Haversine is finished
    // Permutation begins

    let num = data_look_up.len();
    let mut data = Vec::new(); 

    for x in 0..num - 47 {  // Don't allow all 51 states to be permutated.
        if data_look_up[x].state != from_state && 
            data_look_up[x].state != to_state {
                data.push( &data_look_up[x].state );
            }                         
    }
    
    let heap = Heap::new(&mut data);
    let mut i: usize = 0;
    let mut permutations = Vec::new();
    
    for data in heap {    
        permutations.push(data.clone() );
        i += 1;
        println!("\n#{:?}: {:?}", i, data);                   
    }
    
    // will throw a panic error if not equal
    assert_eq!(permutations.len(), i);
    
    // catch any '?' try_catch errors.
    Ok(())
}
