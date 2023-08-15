//use std::fmt;
extern crate cairo;

use self::cairo::{Context, Format, ImageSurface};
use level::Level;
use std::fs::File;

fn draw_tile(context: &Context, x: f64, y: f64, x2: f64, y2: f64, tile: i32) {
    //println!("matching tile: {} with 2 {}", tile, (tile==2));

    match tile {
        //empty (space color)
        0 => context.set_source_rgb(0.06, 0.10, 0.14),

        //wall (dark gray)
        1 => context.set_source_rgb(0.2, 0.2, 0.2),

        //mandatory room (light blue)
        2 => context.set_source_rgb(0.258, 0.525, 0.956),

        //ruin room (red)
        3 => context.set_source_rgb(0.956, 0.2, 0.2),

        //empty room (light green)
        4 => context.set_source_rgb(0.1, 0.9, 0.1),

        //corridor (light gray)
        5 => context.set_source_rgb(0.5, 0.5, 0.5),

        //door (white)
        6 => context.set_source_rgb(0.9, 0.9, 0.9),

        //glass (light blue)
        7 => context.set_source_rgb(0.68, 0.85, 0.9),

        //unknown (pink missing texture)
        _ => context.set_source_rgb(1.0, 0.0, 1.0),
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
    context.move_to(x + 1.0, y + 18.0);
    context.set_font_size(8.0);
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.show_text(&format!("({},{})", &x / 32.0, &y / 32.0));
}

fn draw_tiles(context: &Context, board: &Vec<Vec<i32>>, scale: f64) {
    let mut row = 0;
    for line in board {
        for (col, tile) in line.iter().enumerate() {
            draw_tile(
                context,
                col as f64 * scale,
                row as f64 * scale,
                col as f64 * scale + scale,
                row as f64 * scale + scale,
                *tile,
            );
        }
        row = row + 1;
    }
}

pub fn draw(level: &Level, path: &str, img_name: &str) -> Result<(), ::std::io::Error> {
    let default_output = format!("{}/{}.png", path, img_name);
    let surface = ImageSurface::create(
        Format::ARgb32,
        level.width * level.tile_size,
        level.height * level.tile_size,
    )
    .unwrap();
    let ctx = Context::new(&surface);

    draw_tiles(&ctx, &level.board, level.tile_size as f64);
    let mut file = File::create(default_output)?;
    surface.write_to_png(&mut file).unwrap();

    Ok(())
}
