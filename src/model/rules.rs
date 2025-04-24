use super::grid::Grid;

pub fn compute_new_state(grid: &Grid, x: usize, y: usize) -> bool {
    let neighbours = grid.neighbours_state(x, y);

    let cell_is_live = grid.get(x, y);
    let live_neighbour_count = neighbours.iter().filter(|&&state| state).count();

    let underpopulation = live_neighbour_count < 2;
    let overpopulation = live_neighbour_count > 3;
    let reproduction = live_neighbour_count == 3;

    if underpopulation || overpopulation {
        return false;
    }

    if !cell_is_live && reproduction {
        return true;
    }

    cell_is_live
}
