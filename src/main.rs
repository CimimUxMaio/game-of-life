use macroquad::prelude::*;
use game_of_life::model;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;

#[macroquad::main("Game of Life")]
async fn main() {
    let grid = model::Grid::new(GRID_WIDTH, GRID_HEIGHT);

    loop {
        clear_background(WHITE);

        grid.draw();

        draw_text("IT WORKS!", 10.0, 30.0, 30.0, GREEN);

        next_frame().await
    }
}
