use game_of_life::model;
use macroquad::{miniquad::window::screen_size, prelude::*};

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;

// Convert screen coordinates in pixels to grid coordinates.
// * `screen_pos` - A tuple representing screen coordinates.
fn screen_to_grid(screen_pos: (f32, f32)) -> (usize, usize) {
    let grid_screen_ratio = (
        GRID_WIDTH as f32 / screen_width(),
        GRID_HEIGHT as f32 / screen_height(),
    );

    let x = (grid_screen_ratio.0 * screen_pos.0).floor();
    let y = (grid_screen_ratio.1 * screen_pos.1).floor();

    (x as usize, y as usize) // Will always be greater than or equal to 0
}

// Handle user input for edit mode.
// * `grid` - Current grid structure to be edited by the user.
fn update_edit_mode(grid: &mut model::Grid) {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    let (x, y) = screen_to_grid(mouse_position());
    grid.toggle(x, y);
}

// Update the grid state with a new cell generation.
// * `grid` - Grid structure to be updated.
fn update_execution_mode(grid: &mut model::Grid) {
    let prev = grid.clone();

    for (x, y) in prev.coordinates() {
        let new_state = model::rules::compute_cell_new_state(&prev, x, y);
        grid.update(x, y, new_state);
    }
}

// Draw the game's UI.
// `is_editing` - Wether the game is in editing mode or not.
fn draw_ui(is_editing: bool) {
    let (width, height) = screen_size();

    let mode = if is_editing { "EDITING" } else { "RUNNING" };
    let mode_label = format!("Mode: {}", mode);
    draw_text(mode_label.as_str(), width * 0.01, 30.0, 30.0, BLUE);

    let help_label = "Press <space> to toggle in and out of EDITING and RUNNING modes.";
    draw_text(help_label, width * 0.01, height * 0.97, 20., BLUE);
}

#[macroquad::main("Game of Life")]
async fn main() {
    let mut grid = model::Grid::new(GRID_WIDTH, GRID_HEIGHT);

    let mut is_editing = true;

    loop {
        clear_background(WHITE);

        if is_editing {
            update_edit_mode(&mut grid);
        } else {
            update_execution_mode(&mut grid);
        }

        if is_key_pressed(KeyCode::Space) {
            is_editing = !is_editing;
        }

        grid.draw();

        draw_ui(is_editing);

        // Make game loop slower to be able too see moving objects better
        std::thread::sleep(std::time::Duration::from_millis(30));

        next_frame().await
    }
}
