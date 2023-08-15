extern crate rand;
extern crate sha2;
#[macro_use]
extern crate arrayref;
//#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate serde;
extern crate serde_json;

mod bsp;
mod draw;
mod level;
mod room;
mod roomscorridors;

use clap::{App, Arg};
use rand::distributions::Alphanumeric;
use rand::{distributions::DistString, prelude::*};
use sha2::{Digest, Sha256};

use bsp::BspLevel;
use draw::draw;
use room::Room;
use roomscorridors::RoomsCorridors;

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

enum Algorithm {
    Bsp,
    Rooms,
}

fn main() {
    let matches = App::new("Dungeon")
        .version("1.0")
        .author("James Baum <@whostolemyhat>")
        .arg(
            Arg::with_name("text")
                .short("t")
                .long("text")
                .takes_value(true)
                .help("A string to hash and use as a seed"),
        )
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .takes_value(true)
                .help("An existing seed. Must be 32 characters"),
        )
        .arg(
            Arg::with_name("algo")
                .short("a")
                .long("algorithm")
                .takes_value(true)
                .possible_values(&["rooms", "bsp"])
                .default_value("rooms")
                .help("The type of procedural algorithm to use"),
        )
        .get_matches();

    let seed: String = match matches.value_of("seed") {
        Some(text) => {
            if text.chars().count() < 32 {
                panic!("Seed must be 32 characters long. Use -t option to create a new seed.")
            }
            text.to_string()
        }
        None => match matches.value_of("text") {
            Some(text) => create_hash(&text),
            None => create_hash(&Alphanumeric.sample_string(&mut rand::thread_rng(), 32)),
        },
    };

    let seed_u8 = array_ref!(seed.as_bytes(), 0, 32);
    let mut rng: StdRng = SeedableRng::from_seed(*seed_u8);

    let method = match matches.value_of("algo").expect("Default algorithm not set") {
        "bsp" => Algorithm::Bsp,
        "rooms" => Algorithm::Rooms,
        _ => unreachable![],
    };

    let board_width = 120;
    let board_height = 75;
    let mandatory_elements = vec![
        Room::new(36, 25, 28, 24, 2), //medbay
        Room::new(9, 15, 9, 9, 2),    //engineering room
        Room::new(75, 30, 7, 10, 2),  //tool storage
        Room::new(20, 60, 8, 8, 2),   //cargo storage
    ];

    let level = match method {
        Algorithm::Rooms => RoomsCorridors::new(
            board_width,
            board_height,
            &seed,
            &mut rng,
            mandatory_elements,
        ),
        Algorithm::Bsp => BspLevel::new(
            board_width,
            board_height,
            &seed,
            &mut rng,
            mandatory_elements,
        ),
        //Algorithm::Bsp => BspLevel::new(board_width, board_height, &seed, &mut rng),
    };
    println!("{}", level);

    draw(&level, ".", "level").unwrap();
    //println!("{:?}", level.all_rooms);
    // for element in level.all_rooms.iter() {
    //     println!("{:?}", element.centre);
    // }
    // level.all_rooms.sort_by(|a, b| a.get_distance_to(&Point{x:0,y:0}).cmp(&b.get_distance_to(&Point{x:0,y:0})));
    // println!("\nafter sorting\n");
    // for element in level.all_rooms.iter() {
    //     println!("{:?}", element.centre);
    // }
    // let serialised = serde_json::to_string(&level).unwrap();
    // println!("{}", serialised);
}
