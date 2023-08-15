// use rand::distributions::WeightedIndex;
// use rand::rngs::StdRng;

use rand::prelude::*;
use rand::Rng;


pub enum RoomDimensions {
    MaintRandomEmpty,
    Maint3x3,
    Maint3x5,
    Maint5x3,
    Maint5x4,
    Maint10x5,
    Maint10x10,
}
impl RoomDimensions {
    pub fn get_height(&self) -> i32 {
        let mut rng = thread_rng();
        let min_room_height = 5;
        let max_room_height = 20;
        let height: i32;
        match *self {
            RoomDimensions::MaintRandomEmpty => {
                height = rng.gen_range(min_room_height..max_room_height)
            }
            RoomDimensions::Maint3x3 => height = 3,
            RoomDimensions::Maint3x5 => height = 5,
            RoomDimensions::Maint5x3 => height = 3,
            RoomDimensions::Maint5x4 => height = 4,
            RoomDimensions::Maint10x5 => height = 5,
            RoomDimensions::Maint10x10 => height = 10,
        }
        height + 2
    }

    pub fn get_width(&self) -> i32 {
        let mut rng = thread_rng();
        let min_room_width = 5;
        let max_room_width = 20;
        let width: i32;
        match *self {
            RoomDimensions::MaintRandomEmpty => {
                width = rng.gen_range(min_room_width..max_room_width)
            }
            RoomDimensions::Maint3x3 => width = 3,
            RoomDimensions::Maint3x5 => width = 3,
            RoomDimensions::Maint5x3 => width = 5,
            RoomDimensions::Maint5x4 => width = 5,
            RoomDimensions::Maint10x5 => width = 10,
            RoomDimensions::Maint10x10 => width = 10,
        }
        return width + 2;
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    pub centre: Point,
    pub room_type: i32,
    pub layout: Vec<Vec<i32>>,
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32, room_type: i32) -> Self {
        Room {
            x,
            y,
            x2: x + width,
            y2: y + height,
            width,
            height,
            centre: Point {
                x: x + (width / 2),
                y: y + (height / 2),
            },
            //door_anchors: todo!(),
            room_type: room_type,
            layout: Vec::new(),
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
    }
    pub fn get_distance_to(&self, other: &Point) -> i32 {
        (((other.x - self.centre.x).pow(2) + (other.y - self.centre.y).pow(2)) as f64).sqrt() as i32
    }
}
