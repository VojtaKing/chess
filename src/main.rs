use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Tile {
    color: Color,
}

#[macroquad::main("Board")]
async fn main() {
    let size = 8;
    let tile_size = 50.0;

    // create board
    let mut board = vec![vec![Tile { color: WHITE }; size]; size];
    for x in 0..size {
        for y in 0..size {
            if (x + y) % 2 == 0 {
                board[x][y].color = BLACK;
            }
        }
    }

    // example: change one tile
    // board[0][0].color = RED;

    loop {
        clear_background(LIGHTGRAY);

        for x in 0..size {
            for y in 0..size {
                draw_rectangle(
                    x as f32 * tile_size,
                    y as f32 * tile_size,
                    tile_size,
                    tile_size,
                    board[x][y].color,
                );
            }
        }

        next_frame().await;
    }
}
