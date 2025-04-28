use super::grid::Grid;

// Compute the state of a given cell on the next generation.
// * `grid` - Latest grid state with neighbour information.
// * `x` - Target cell's X grid coordinate.
// * `y` - Target cell's Y grid coordinate.
pub fn compute_cell_new_state(grid: &Grid, x: usize, y: usize) -> bool {
    let cell_is_alive = grid.get(x, y);

    let neighbour_count = grid
        .neighbours(x, y)
        .iter()
        .filter(|&&(nx, ny)| grid.get(nx, ny))
        .count();

    let underpopulation = neighbour_count < 2;
    let overpopulation = neighbour_count > 3;
    let reproduction = neighbour_count == 3;

    if underpopulation || overpopulation {
        return false;
    }

    if !cell_is_alive && reproduction {
        return true;
    }

    cell_is_alive
}

#[cfg(test)]
mod tests {
    use crate::model::*;

    const CENTER: (usize, usize) = (1, 1);

    fn create_small_grid() -> Grid {
        Grid::new(3, 3)
    }

    #[test]
    fn compute_state_with_underpopulation_for_dead() {
        let mut grid = create_small_grid();
        grid.toggle(0, 0); // Only one live neighbour
        assert!(!compute_cell_new_state(&grid, CENTER.0, CENTER.1));
    }

    #[test]
    fn compute_state_with_underpopulation_for_live() {
        let mut grid = create_small_grid();
        grid.toggle(0, 0); // Only one live neighbour
        grid.toggle(CENTER.0, CENTER.1); // Center is live
        assert!(!compute_cell_new_state(&grid, CENTER.0, CENTER.1));
    }

    #[test]
    fn compute_state_with_overpopulation_for_dead() {
        let mut grid = create_small_grid();

        // 4 neighbours
        grid.toggle(0, 0);
        grid.toggle(2, 2);
        grid.toggle(1, 2);
        grid.toggle(1, 0);

        assert!(!compute_cell_new_state(&grid, CENTER.0, CENTER.1));
    }

    #[test]
    fn compute_state_with_overpopulation_for_live() {
        let mut grid = create_small_grid();

        // 4 neighbours
        grid.toggle(0, 0);
        grid.toggle(2, 2);
        grid.toggle(1, 2);
        grid.toggle(1, 0);

        grid.toggle(CENTER.0, CENTER.1); // Center is live
        assert!(!compute_cell_new_state(&grid, CENTER.0, CENTER.1));
    }

    #[test]
    fn compute_state_with_reproduction_for_dead() {
        let mut grid = create_small_grid();

        // 3 neighbours
        grid.toggle(1, 2);
        grid.toggle(1, 0);
        grid.toggle(2, 1);

        assert!(compute_cell_new_state(&grid, CENTER.0, CENTER.1));
    }

    #[test]
    fn compute_state_with_reproduction_for_live() {
        let mut grid = create_small_grid();

        // 3 neighbours
        grid.toggle(0, 0);
        grid.toggle(2, 2);
        grid.toggle(1, 2);

        grid.toggle(CENTER.0, CENTER.1); // Center is live

        // In repreduction conditions live cells stay live.
        assert!(compute_cell_new_state(&grid, CENTER.0, CENTER.1));
    }
}
