use macroquad::prelude::{BLACK, draw_rectangle, screen_height, screen_width};

pub struct Grid {
    pub width: usize,
    pub height: usize,
    matrix: Vec<Vec<bool>>,
}

impl Grid {
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

    pub fn draw(&self) {
        let cell_width = screen_width() / self.width as f32;
        let cell_height = screen_height() / self.height as f32;

        for row_n in 0..self.width {
            for col_n in 0..self.height {
                let is_live = self.matrix[row_n][col_n];

                if !is_live {
                    // If not live don't draw.
                    continue;
                }

                // If live, draw cell.
                let x = row_n as f32 * cell_width;
                let y = col_n as f32 * cell_height;

                draw_rectangle(x, y, cell_width, cell_height, BLACK);
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.matrix[x % self.width][y % self.height]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut bool {
        &mut self.matrix[x % self.width][y % self.height]
    }

    pub fn update(&mut self, x: usize, y: usize, value: bool) {
        *self.get_mut(x, y) = value;
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        self.update(x, y, !self.get(x, y));
    }

    pub fn coordinates(&self) -> Vec<(usize, usize)> {
        let mut coordinates = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                coordinates.push((x, y));
            }
        }

        coordinates
    }

    pub fn neighbours_state(&self, x: usize, y: usize) -> Vec<bool> {
        let mut neighbours = Vec::new();

        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    // Ignore target cell: (x, y)
                    continue;
                }

                let nx = (x as i32 + dx) as usize % self.width;
                let ny = (y as i32 + dy) as usize % self.height;
                neighbours.push(self.get(nx, ny));
            }
        }

        neighbours
    }
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            width: self.width,
            height: self.height,
            matrix: self.matrix.clone(),
        }
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
    fn update_should_wrap_overflowing_positions() {
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
    fn toggle_should_wrap_overflowing_positions() {
        let mut grid = create_grid();
        grid.toggle(grid.width + 2, grid.height + 3);
        assert!(grid.matrix[2][3]);
    }
}
