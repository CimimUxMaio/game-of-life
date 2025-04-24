use game_of_life::model;
use macroquad::prelude::*;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;

fn run(prev: &model::Grid, grid: &mut model::Grid) {
    for (x, y) in prev.coordinates() {
        let new_state = model::rules::compute_new_state(grid, x, y);
        grid.update(x, y, new_state);
    }
}

#[macroquad::main("Game of Life")]
async fn main() {
    let mut prev_grid = model::Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut grid = prev_grid.clone();

    // Initial state
    grid.toggle(20, 20);
    grid.toggle(20, 21);
    grid.toggle(20, 22);
    grid.toggle(21, 23);
    grid.toggle(22, 21);
    // grid.toggle(23, 20);
    // grid.toggle(24, 19);
    // grid.toggle(25, 18);

    loop {
        clear_background(WHITE);

        run(&prev_grid, &mut grid);

        grid.draw();

        let fps_text = format!("FPS: {}", get_fps());
        draw_text(fps_text.as_str(), 10.0, 30.0, 30.0, GREEN);

        prev_grid = grid.clone();

        std::thread::sleep(std::time::Duration::from_millis(60));
        next_frame().await
    }
}
