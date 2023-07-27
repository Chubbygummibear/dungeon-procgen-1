use room::Room;
use std::fmt;

pub struct Level {
    pub width: i32,
    pub height: i32,
    pub board: Vec<Vec<i32>>,
    pub tile_size: i32,
    pub mandatory_rooms: Vec<Room>,
    pub open_areas: Vec<Room>,
    pub rooms: Vec<Room>,
    hash: String,
}

impl Level {
    pub fn new(
        width: i32,
        height: i32,
        hash: &String,
        mandatory_elements: Option<Vec<Room>>,
    ) -> Self {
        let mut board = Vec::new();
        for _ in 0..height {
            let row = vec![0; width as usize];
            board.push(row);
        }

        Level {
            width,
            height,
            board,
            tile_size: 16,
            mandatory_rooms: mandatory_elements.unwrap_or(Vec::new()),
            open_areas: Vec::new(),
            rooms: Vec::new(),
            hash: hash.clone(),
        }
    }

    pub fn add_room(&mut self, room: &Room) {
        for row in 0..room.height {
            for col in 0..room.width {
                let y = (room.y + row) as usize;
                let x = (room.x + col) as usize;
                if row == 0 || col == 0 || row == room.height - 1 || col == room.width - 1 {
                    // might just let byond handle the walls
                    self.board[y][x] = 1;
                } else {
                    self.board[y][x] = room.room_type;
                }
            }
        }

        self.rooms.push(*room);
    }

    pub fn add_open_area(&mut self, room: &Room) {
        for row in 0..room.height {
            for col in 0..room.width {
                let y = (room.y + row) as usize;
                let x = (room.x + col) as usize;
                if self.board[y][x] == 0 {
                    if row == 0 || col == 0 || row == room.height - 1 || col == room.width - 1 {
                        // might just let byond handle the walls
                        self.board[y][x] = 1;
                    } else {
                        self.board[y][x] = room.room_type;
                    }
                }
            }
        }

        self.open_areas.push(*room);
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
