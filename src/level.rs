use room::Room;
use std::fmt;

pub enum _IntersectOption {
    NoIntersect = 0,
    IntersectSelf = 1,
    IntersectOther = 2,
    IntersectLesser = 3,
}
pub struct _RoomParam {
    priority: i32,
    room_type: i32,
    wall_type: i32,

    width_lower_limit: i32,
    width_upper_limit: i32,
    height_lower_limit: i32,
    height_upper_limit: i32,

    intersect_behavior: _IntersectOption,
    num_of_rooms: i32,
    placement_attempts_limit: i32,
}

pub struct Level {
    pub width: i32,
    pub height: i32,
    pub board: Vec<Vec<i32>>,
    pub tile_size: i32,
    pub all_rooms: Vec<Room>,
    pub mandatory_rooms: Vec<Room>,
    pub open_areas: Vec<Room>,
    pub space_areas: Vec<Room>,
    pub rooms: Vec<Room>,
    //hash: String,
}

impl Level {
    pub fn new(
        width: i32,
        height: i32,
        _hash: &String,
        mandatory_elements: Option<Vec<Room>>,
    ) -> Self {
        let mut new_level = Level {
            width,
            height,
            board: Vec::new(),
            tile_size: 32,
            all_rooms: Vec::new(),
            mandatory_rooms: mandatory_elements.unwrap_or(Vec::new()),
            open_areas: Vec::new(),
            space_areas: Vec::new(),
            rooms: Vec::new(),
            //hash: hash.clone(),
        };
        new_level.update_board();
        new_level
    }

    pub fn update_board(&mut self) -> Vec<Vec<i32>> {
        let mut new_board = Vec::new();
        for index in 0..self.height {
            let space_tile = 0;
            let wall_tile = 1;
            let floor_tile = 5;
            let gen_floor_first = true;

            let mut row = vec![floor_tile; self.width as usize];
            if !gen_floor_first {
                row = vec![space_tile; self.width as usize];
            }
            if gen_floor_first {
                if index == 0 || index == self.height - 1 {
                    row = vec![wall_tile; self.width as usize];
                }

                row[0] = wall_tile;
                row[self.width as usize - 1] = wall_tile;
            }

            new_board.push(row);
        }
        for room in &self.all_rooms {
            for row in 0..room.height {
                for col in 0..room.width {
                    let y = (room.y + row) as usize;
                    let x = (room.x + col) as usize;
                    if row == 0 || col == 0 || row == room.height - 1 || col == room.width - 1 {
                        // might just let byond handle the walls
                        new_board[y][x] = 1;
                    } else {
                        new_board[y][x] = room.room_type;
                    }
                }
            }
        }
        self.board = new_board.clone();
        new_board
    }

    pub fn add_room(&mut self, room: &Room) {
        match room.room_type {
            2 => {
                self.mandatory_rooms.push(room.clone());
            }
            _ => self.rooms.push(room.clone()),
        }
        self.all_rooms.push(room.clone());
        self.update_board();
    }

    pub fn add_open_area(&mut self, room: &Room) {
        match room.room_type {
            0 => {
                self.space_areas.push(room.clone());
            }

            4 => {
                self.open_areas.push(room.clone());
            }
            _ => self.rooms.push(room.clone()),
        }
        self.all_rooms.push(room.clone());
        self.update_board();
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                write!(f, "{} ", self.board[row][col])?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}
