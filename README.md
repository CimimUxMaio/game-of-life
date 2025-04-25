# Game of Life
Rust implementation of Conway's Game of Life using WASM.

## Quick Start
- To build the project.
```
make
```
- To run the game.
```
make run
```
- To run tests.
```
make test
```
- To clean the current build.
```
make clean
```

## Quick Guide
After running the game, the user will be presented with a blank window.

In the upper left corner, a text label is presented indicating the current mode the game is on.

On start, the game defaults to **EDITING** mode. In this mode, the user is able to bring *cells*
to life in the window by *left-clicking* over an invisible grid. This way, the user can decide
the initial patterns they would like to run in the simulation.

At the bottom of the window another label is shown, which indicates how to switch between each mode.
By pressing the *space bar*, the user can toggle between **EDITING** and **RUNNING** modes.

In **RUNNING** mode, the simulation is run, applying the rules of the Game of Life for each generation.
The user may toggle in and out of **RUNNING**/**EDITING** modes any time during the simulation.

## Game of Life Rules

1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
1. Any live cell with two or three live neighbours lives on to the next generation.
1. Any live cell with more than three live neighbours dies, as if by overpopulation.
1. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

## Dependencies
- rust = "1.86.0"
- macroquad = "0.4"
