use macroquad::prelude::{draw_rectangle, screen_width, screen_height, BLACK};

pub struct Grid {
    pub width: usize,
    pub height: usize,
    matrix: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let mut matrix = Vec::new();

        for _ in 0..width {
            let row = vec![true; height];
            matrix.push(row);
        }

        Grid { width, height, matrix }
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

                draw_rectangle(
                    x, y,
                    cell_width,
                    cell_height,
                    BLACK
                );
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
}
