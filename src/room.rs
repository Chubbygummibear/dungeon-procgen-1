#[derive(Debug, Clone, Copy, Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[derive(Clone, Copy, Serialize)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    //pub door_anchors: Vec<Point>,
    pub centre: Point,
    pub room_type: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Room {
            x,
            y,
            x2: x + width,
            y2: y + height,
            width,
            height,
            centre: Point {
                x: x + (width / 2),
                y: y + (height / 2)
            },
            //door_anchors: todo!(),
            room_type: 1,
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
    }
}

pub struct MandatoryRoom {
    pub name: String,
    pub door_anchors: Vec<Point>,
    pub room: Room,
}