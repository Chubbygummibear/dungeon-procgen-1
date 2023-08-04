use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::Rng;

use level::Level;
use room::{Room, Point};

pub enum IntersectOption {
    NoIntersect = 0,
    IntersectSelf = 1,
    IntersectOther = 2,
    IntersectLesser = 3,

}
pub struct RoomParam {
    priority: i32,
    room_type: i32,
    wall_type: i32,
    
    width_lower_limit: i32,
    width_upper_limit: i32,
    height_lower_limit: i32,
    height_upper_limit: i32,
    
    intersect_behavior: IntersectOption,
    num_of_rooms: i32,
    placement_attempts_limit: i32,
}

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
    fn get_height(&self) -> i32 {
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

    fn get_width(&self) -> i32 {
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
pub struct RoomsCorridors {
    level: Level,
}

impl RoomsCorridors {
    pub fn new(
        width: i32,
        height: i32,
        hash: &String,
        rng: &mut StdRng,
        mandatory_elements: Vec<Room>,
    ) -> Level {
        let level = Level::new(width, height, hash, None);

        let mut map = RoomsCorridors { level };
        println!("using roomscorridors");
        for mut man_room in mandatory_elements {
            man_room.room_type = 2;
            map.level.add_room(&man_room)
        }

        let space_level_terrain = false;
        
        if !space_level_terrain {
            map.place_space_areas(rng);        
        }
        map.place_open_areas(rng);
        map.place_rooms(rng);
        for element in map.level.all_rooms.iter() {
            println!("{:?}", element.centre);
        }
        map.level.all_rooms.sort_by(|a, b| a.get_distance_to(&Point{x:0,y:0}).cmp(&b.get_distance_to(&Point{x:0,y:0})));
        println!("\nafter sorting\n");
        for element in map.level.all_rooms.iter() {
            println!("{:?}", element.centre);
        }
        map.place_corridors(rng);

        map.level
    }

    pub fn place_open_areas(&mut self, rng: &mut StdRng) {
        let max_areas = 40;

        while self.level.open_areas.len() < max_areas {
            let mut x = rng.gen_range(0..self.level.width);
            let mut y = rng.gen_range(0..self.level.height);
            let room_layout = RoomDimensions::MaintRandomEmpty;
            let width = room_layout.get_width();
            let height = room_layout.get_height();

            if x + width > self.level.width {
                x = self.level.width - width;
            }

            if y + height > self.level.height {
                y = self.level.height - height;
            }

            let mut room = Room::new(x, y, width, height);
            room.room_type = 4;

            //self.level.add_open_area(&room);
            let mut collides = false;
            for other_room in &self.level.all_rooms {
                //println!("does {:?} intersect with {:?}: {}",room, other_room, room.intersects(&other_room));
                if room.intersects(&other_room) && &other_room.room_type < &room.room_type {
                    collides = true;
                    break;
                }
            }
            if !collides {
                self.level.add_open_area(&room);
                //attempts = 0;
            }
        }
    }

    pub fn place_space_areas(&mut self, rng: &mut StdRng) {
        let max_areas = 30;
        let max_attempts = 10;
        let mut attempts = 0;

        while self.level.space_areas.len() < max_areas && attempts <= max_attempts {
            attempts+=1;
            let mut x = rng.gen_range(0..self.level.width);
            let mut y = rng.gen_range(0..self.level.height);
            //let room_layout = RoomDimensions::MaintRandomEmpty;
            let width = rng.gen_range(4..15);
            let height = rng.gen_range(4..15);

            if x + width > self.level.width {
                x = self.level.width - width;
            }

            if y + height > self.level.height {
                y = self.level.height - height;
            }
            let mut collides = false;
            let mut room = Room::new(x, y, width, height);
            room.room_type = 0;

            for other_room in &self.level.all_rooms {
                if room.intersects(&other_room) && &other_room.room_type <= &room.room_type {
                    collides = true;
                    break;
                }
            }
            if !collides {
                self.level.add_open_area(&room);
                attempts = 0;
            }

        }
    }

    pub fn place_rooms(&mut self, rng: &mut StdRng) {
        let max_rooms = 30;
        let max_attempts = 10;
        let mut attempts = 0;
        while self.level.rooms.len() < max_rooms && attempts <= max_attempts{
            attempts+=1;
            let mut x = rng.gen_range(0..self.level.width);
            let mut y = rng.gen_range(0..self.level.height);

            // empty maint rooms to add some open areas, and the various ruins that spawn in maint.
            // empty open areas are far more likely to occur than the ruins, and small ruins are more likely than big ones
            let choices = [
                //RoomDimensions::MaintRandomEmpty,
                RoomDimensions::Maint3x3,
                RoomDimensions::Maint3x5,
                RoomDimensions::Maint5x3,
                RoomDimensions::Maint5x4,
                RoomDimensions::Maint10x5,
                RoomDimensions::Maint10x10,
            ];
            let weights = [3, 2, 3, 3, 2, 1];
            let dist = WeightedIndex::new(&weights).unwrap();
            let mut rng = thread_rng();
            let room_layout = &choices[dist.sample(&mut rng)];
            let width = room_layout.get_width();
            let height = room_layout.get_height();

            if x + width > self.level.width {
                x = self.level.width - width;
            }

            if y + height > self.level.height {
                y = self.level.height - height;
            }

            let mut collides = false;
            let mut room = Room::new(x, y, width, height);
            
            room.room_type = 3;

            for other_room in &self.level.rooms {
                if room.intersects(&other_room) && other_room.room_type != 4 {
                    collides = true;
                    break;
                }
            }

            if !collides {
                self.level.add_room(&room);
                attempts = 0;
            }

        }
    }

    fn place_corridors(&mut self, rng: &mut StdRng) {
        for i in 0..(self.level.all_rooms.len() - 1) {
            let room = self.level.all_rooms[i].clone();
            let other = self.level.all_rooms[i + 1].clone();
            println!("corridor from {:?} to {:?}", room.centre, other.centre);
            // randomly pick vert or horz
            match other.centre.x-room.centre.x < other.centre.y-room.centre.y {
                true => {
                    match room.centre.x <= other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y),
                    }
                    match room.centre.y <= other.centre.y {
                        true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x),
                    }
                }
                _ => {
                    match room.centre.y <= other.centre.y {
                        true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x),
                    }
                    match room.centre.x <= other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y),
                    }
                }
            }
        }
    }

    fn horz_corridor(&mut self, start_x: i32, end_x: i32, y: i32) {
        for col in start_x..end_x + 1 {
            if self.level.board[y as usize][col as usize] == 0 {
                self.level.board[y as usize][col as usize] = 5;
            }
            else if self.level.board[y as usize][col as usize] == 1 {
                self.level.board[y as usize][col as usize] = 6;
            }
        }
    }

    fn vert_corridor(&mut self, start_y: i32, end_y: i32, x: i32) {
        for row in start_y..end_y + 1 {
            if self.level.board[row as usize][x as usize] == 0 {
                self.level.board[row as usize][x as usize] = 5;
            }
            else if self.level.board[row as usize][x as usize] == 1 {
                self.level.board[row as usize][x as usize] = 6;
            }
        }
    }
}
