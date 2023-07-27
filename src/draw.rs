//use std::fmt;
extern crate cairo;

use level::Level;
use std::fs::File;
use self::cairo::{ Context, Format, ImageSurface };
//use serde::{Serialize, Serializer};

// #[derive(Clone)]
// pub enum Tile {
//     Empty,
//     Corridor,
//     MandatoryRoom,
//     RuinRoom,
//     RandomEmptyRoom,
//     Wall,
// }

// impl Serialize for Tile {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             Tile::Empty => serializer.serialize_i32(0),
//             Tile::Corridor => serializer.serialize_i32(1),
//             Tile::MandatoryRoom => serializer.serialize_i32(2),
//             Tile::RuinRoom => serializer.serialize_i32(3),
//             Tile::RandomEmptyRoom => serializer.serialize_i32(4),
//             Tile::Wall => serializer.serialize_i32(5),
//         }
//     }
// }

// impl fmt::Display for Tile {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Tile::Empty => write!(f, "0"),
//             Tile::Corridor => write!(f, "1"),
//             Tile::MandatoryRoom => write!(f, "2"),
//             Tile::RuinRoom => write!(f, "3"),
//             Tile::RandomEmptyRoom => write!(f, "4"),
//             Tile::Wall => write!(f, "5"),
//         }
//     }
// }

fn draw_tile(context: &Context, x: f64, y: f64, x2: f64, y2: f64, tile:i32) {
    //println!("matching tile: {} with 2 {}", tile, (tile==2));
    match tile {
        //empty (space color)
        0=>context.set_source_rgb(0.06, 0.10, 0.14),
        
        //wall (dark gray)
        1=>context.set_source_rgb(0.2, 0.2, 0.2),
        
        //mandatory room (light blue)
        2=>context.set_source_rgb(0.258, 0.525, 0.956),
        
        //ruin room (red)
        3=>context.set_source_rgb(0.956, 0.2, 0.2),
        
        //empty room (light green)
        4=>context.set_source_rgb(0.1, 0.9, 0.1),
        
        //corridor (light gray)
        5=>context.set_source_rgb(0.7, 0.7, 0.7),
        
        //unknown (pink missing texture)
        _=>context.set_source_rgb(1.0, 0.0, 1.0),
    }
    //context.set_source_rgb(0.06, 0.10, 0.14);
    context.new_path();
    context.move_to(x, y);
    context.line_to(x2, y);
    context.line_to(x, y2);
    context.move_to(x2, y2);
    context.line_to(x2, y);
    context.line_to(x, y2);
    context.close_path();
    context.fill();
}

// fn draw_empty(context: &Context, x: f64, y: f64, x2: f64, y2: f64) {
//     context.set_source_rgb(0.06, 0.10, 0.14);
//     context.new_path();
//     context.move_to(x, y);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.move_to(x2, y2);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.close_path();
//     context.fill();
// }
// fn draw_corridor(context: &Context, x: f64, y: f64, x2: f64, y2: f64) {
//     context.set_source_rgb(0.7, 0.7, 0.7);
//     context.new_path();
//     context.move_to(x, y);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.move_to(x2, y2);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.close_path();
//     context.fill();
// }
// fn draw_mandatory_room(context: &Context, x: f64, y: f64, x2: f64, y2: f64) {
//     context.set_source_rgb(0.258, 0.525, 0.956);
//     context.new_path();
//     context.move_to(x, y);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.move_to(x2, y2);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.close_path();
//     context.fill();
// }
// fn draw_ruin_room(context: &Context, x: f64, y: f64, x2: f64, y2: f64) {
//     context.set_source_rgb(0.956, 0.2, 0.2);
//     context.new_path();
//     context.move_to(x, y);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.move_to(x2, y2);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.close_path();
//     context.fill();
// }
// fn draw_empty_room(context: &Context, x: f64, y: f64, x2: f64, y2: f64) {
//     context.set_source_rgb(0.1, 0.9, 0.1);
//     context.new_path();
//     context.move_to(x, y);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.move_to(x2, y2);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.close_path();
//     context.fill();
// }
// fn draw_wall(context: &Context, x: f64, y: f64, x2: f64, y2: f64) {
//     context.set_source_rgb(0.2, 0.2, 0.2);
//     context.new_path();
//     context.move_to(x, y);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.move_to(x2, y2);
//     context.line_to(x2, y);
//     context.line_to(x, y2);
//     context.close_path();
//     context.fill();
// }


fn draw_tiles(context: &Context, board: &Vec<Vec<i32>>, scale: f64) {
    let mut row = 0;
    for line in board {
        for (col, tile) in line.iter().enumerate() {
            draw_tile(context, col as f64 * scale, row as f64 * scale, col as f64 * scale + scale, row as f64 * scale + scale, *tile);
            // match tile {
            //     0 => draw_empty(context, col as f64 * scale, row as f64 * scale, col as f64 * scale + scale, row as f64 * scale + scale),
            //     Tile::Corridor => draw_corridor(context, col as f64 * scale, row as f64 * scale, col as f64 * scale + scale, row as f64 * scale + scale),
            //     1 => draw_mandatory_room(context, col as f64 * scale, row as f64 * scale, col as f64 * scale + scale, row as f64 * scale + scale),
            //     Tile::RuinRoom => draw_ruin_room(context, col as f64 * scale, row as f64 * scale, col as f64 * scale + scale, row as f64 * scale + scale),
            //     Tile::RandomEmptyRoom => draw_empty_room(context, col as f64 * scale, row as f64 * scale, col as f64 * scale + scale, row as f64 * scale + scale),
            //     Tile::Wall => draw_wall(context, col as f64 * scale, row as f64 * scale, col as f64 * scale + scale, row as f64 * scale + scale),
            //     _ => (),
                
            // }

        }
        row = row + 1;
    }
}

pub fn draw(level: &Level, path: &str, img_name: &str) -> Result<(), ::std::io::Error> {
    let default_output = format!("{}/{}.png", path, img_name);
    let surface = ImageSurface::create(Format::ARgb32, level.width * level.tile_size, level.height * level.tile_size).unwrap();
    let ctx = Context::new(&surface);

    draw_tiles(&ctx, &level.board, level.tile_size as f64);
    let mut file = File::create(default_output)?;
    surface.write_to_png(&mut file).unwrap();

    Ok(())
}