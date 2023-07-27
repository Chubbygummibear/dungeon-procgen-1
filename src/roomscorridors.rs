use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::Rng;

use level::Level;
use room::Room;

//use crate::room;

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
        let max_room_height = 15;
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
        let max_room_width = 15;
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
        map.place_rooms(rng);
        map.place_open_areas(rng);
        map.place_corridors(rng);

        map.level
    }

    pub fn place_open_areas(&mut self, rng: &mut StdRng) {
        let max_areas = 10;

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

            self.level.add_open_area(&room);
        }
    }

    pub fn place_rooms(&mut self, rng: &mut StdRng) {
        let max_rooms = 20;

        while self.level.rooms.len() < max_rooms {
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
            // match room_layout {
            //     RoomDimensions::MaintRandomEmpty => room.room_type = 4,
            //     _ => room.room_type = 3,
            // }

            for other_room in &self.level.rooms {
                if room.intersects(&other_room) && other_room.room_type != 4 {
                    collides = true;
                    break;
                }
            }

            if !collides {
                self.level.add_room(&room);
            }
        }
    }

    fn place_corridors(&mut self, rng: &mut StdRng) {
        for i in 0..(self.level.rooms.len() - 1) {
            let room = self.level.rooms[i];
            let other = self.level.rooms[i + 1];

            // randomly pick vert or horz
            match rng.gen_range(0..2) {
                0 => {
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
        }
    }

    fn vert_corridor(&mut self, start_y: i32, end_y: i32, x: i32) {
        for row in start_y..end_y + 1 {
            if self.level.board[row as usize][x as usize] == 0 {
                self.level.board[row as usize][x as usize] = 5;
            }
            // self.level.board[row as usize][x as usize] = Tile::Corridor;
        }
    }
}
