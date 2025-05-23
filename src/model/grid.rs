use macroquad::prelude::{BLACK, draw_rectangle, screen_height, screen_width};

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    matrix: Vec<Vec<bool>>,
}

impl Grid {
    // Create a new Grid object.
    // * `width` - New grid's width (must be greater than 0).
    // * `height` - New grid's height (must be greater than 0).
    pub fn new(width: usize, height: usize) -> Grid {
        if width == 0 || height == 0 {
            panic!("Dimensions for the grid can not be 0");
        }

        let matrix = vec![vec![false; width]; height];

        Grid {
            width,
            height,
            matrix,
        }
    }

    // Draw all cell states in the grid to the screen.
    pub fn draw(&self) {
        let cell_width = screen_width() / self.width as f32;
        let cell_height = screen_height() / self.height as f32;

        for row_n in 0..self.width {
            for col_n in 0..self.height {
                let is_alive = self.matrix[row_n][col_n];

                if !is_alive {
                    // If it is not alive, don't draw cell.
                    continue;
                }

                // If it is alive, draw cell.
                let x = row_n as f32 * cell_width;
                let y = col_n as f32 * cell_height;

                draw_rectangle(x, y, cell_width, cell_height, BLACK);
            }
        }
    }

    // Get a cell's state given its grid coordinates.
    // * `x` - Cell's X grid coordinate.
    // * `y` - Cell's Y grid coordinate.
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.matrix[x][y]
    }

    // Get a cell's state as a mutable reference, given its grid coordinates.
    // * `x` - Cell's X grid coordinate.
    // * `y` - Cell's Y grid coordinate.
    fn get_mut(&mut self, x: usize, y: usize) -> &mut bool {
        &mut self.matrix[x][y]
    }

    // Update a cell's state given its coordinates within the grid.
    // * `x` - Cell's X grid coordinate.
    // * `y` - Cell's Y grid coordinate.
    // * `value` - Cell's new state.
    pub fn update(&mut self, x: usize, y: usize, value: bool) {
        *self.get_mut(x, y) = value;
    }

    // Toggle a cell's state (true/false) given its coordinates within the grid.
    // * `x` - Cell's X grid coordinate.
    // * `y` - Cell's Y grid coordinate.
    pub fn toggle(&mut self, x: usize, y: usize) {
        self.update(x, y, !self.get(x, y));
    }

    // Get all the gird's valid coordinates as a pair of usize integers.
    pub fn coordinates(&self) -> Vec<(usize, usize)> {
        let mut coordinates = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                coordinates.push((x, y));
            }
        }

        coordinates
    }

    // Get all valid neighbours (as coordinates) for a given cell.
    // * `x` - Target cell's X grid coordinate.
    // * `y` - Target cell's Y grid coordinate.
    pub fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                let is_target_cell = (nx, ny) == (x as i32, y as i32);
                let is_out_of_bounds =
                    nx < 0 || nx >= self.width as i32 || ny < 0 || ny >= self.height as i32;

                if is_target_cell || is_out_of_bounds {
                    continue;
                }

                // Cast should not fail after checks
                neighbours.push((nx as usize, ny as usize));
            }
        }

        neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid_with_positive_non_zero_dimensions() {
        let width = 20;
        let height = 15;
        Grid::new(width, height);
    }

    #[test]
    #[should_panic]
    fn new_grid_with_zero_width() {
        let width = 0;
        let height = 15;
        Grid::new(width, height);
    }

    #[test]
    #[should_panic]
    fn new_grid_with_zero_height() {
        let width = 30;
        let height = 0;
        Grid::new(width, height);
    }

    fn create_grid() -> Grid {
        Grid::new(5, 5)
    }

    #[test]
    fn new_grid_should_not_contain_cells() {
        let grid = create_grid();

        for row_n in 0..grid.width {
            for col_n in 0..grid.height {
                assert!(!grid.matrix[row_n][col_n]);
            }
        }
    }

    #[test]
    fn update_should_update_a_given_position() {
        let mut grid = create_grid();
        grid.update(0, 0, true);
        assert!(grid.matrix[0][0]);
        grid.update(0, 0, false);
        assert!(!grid.matrix[0][0]);
    }

    #[test]
    #[should_panic]
    fn update_should_panic_for_overflowing_positions() {
        let mut grid = create_grid();
        grid.update(grid.width, grid.height + 1, true);
        assert!(grid.matrix[0][1]);
    }

    #[test]
    fn toggle_should_toggle_a_given_position() {
        let mut grid = create_grid();
        grid.toggle(0, 0);
        assert!(grid.matrix[0][0]);
        grid.toggle(0, 0);
        assert!(!grid.matrix[0][0]);
    }

    #[test]
    #[should_panic]
    fn toggle_should_panic_for_overflowing_positions() {
        let mut grid = create_grid();
        grid.toggle(grid.width + 2, grid.height + 3);
        assert!(grid.matrix[2][3]);
    }

    #[test]
    fn neighbour_count_for_inner_cell() {
        let grid = create_grid();
        let neighbour_count = grid.neighbours(2, 2).len();
        assert_eq!(neighbour_count, 8);
    }

    #[test]
    fn neighbour_count_for_border_cell() {
        let grid = create_grid();

        // All 4 centered borders
        let targets = [(2, 0), (4, 2), (2, 4), (0, 2)];

        for target in targets {
            let neighbour_count = grid.neighbours(target.0, target.1).len();
            assert_eq!(neighbour_count, 5);
        }
    }

    #[test]
    fn neighbour_count_for_corner_cells() {
        let grid = create_grid();

        // All 4 corners
        let targets = [(0, 0), (4, 0), (4, 4), (0, 4)];

        for target in targets {
            let neighbour_count = grid.neighbours(target.0, target.1).len();
            assert_eq!(neighbour_count, 3);
        }
    }
}
