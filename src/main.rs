use game_of_life::model;
use macroquad::prelude::*;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;

// Updates the state of the current grid given a the previous one
fn update(prev: &model::Grid, grid: &mut model::Grid) {
    for (x, y) in prev.coordinates() {
        let new_state = model::rules::compute_new_state(prev, x, y);
        grid.update(x, y, new_state);
    }
}

#[macroquad::main("Game of Life")]
async fn main() {
    let mut prev_grid = model::Grid::new(GRID_WIDTH, GRID_HEIGHT);

    // Initial state
    prev_grid.toggle(24, 22);
    prev_grid.toggle(24, 23);
    prev_grid.toggle(24, 24);
    prev_grid.toggle(23, 23);
    prev_grid.toggle(25, 24);

    let mut grid = prev_grid.clone();

    loop {
        clear_background(WHITE);

        update(&prev_grid, &mut grid);

        grid.draw();

        let fps_text = format!("FPS: {}", get_fps());
        draw_text(fps_text.as_str(), 10.0, 30.0, 30.0, GREEN);

        prev_grid = grid.clone();

        // Make game loop slower to be able too see moving objects better
        std::thread::sleep(std::time::Duration::from_millis(30));

        next_frame().await
    }
}
