# Bevy Cellular Automaton

[![workflow](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml)

`bevy_life` is a generic plugin for [cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton).
From the classic 2D [Conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) to [WireWorld](https://en.wikipedia.org/wiki/Wireworld) and 3D rules, the plugin is completely generic and dynamic.

See:
 - [Game of life variations](https://cs.stanford.edu/people/eroberts/courses/soco/projects/2008-09/modeling-natural-systems/gameOfLife2.html)
 - [Wireworld implementation](https://www.quinapalus.com/wi-index.html)
 
## Examples

For every example pressing space reloads the board

### Classic 2D

Run `cargo run --example 2d_classic --features auto-coloring --release`

![Alt](./docs/2d_classic_demo.gif "classic demo gif")

### Cyclic 2D

Run `cargo run --example 2d_cyclic --features auto-coloring --release`

![Alt](./docs/2d_cyclic_demo.gif "cyclic demo gif")

### Wire World 2D

Run `cargo run --example 2d_wireworld --features auto-coloring --release`

The example is dynamic, use the left mouse click to create a conductor cell on an empty space or to create an electron head

![Alt](./docs/2d_wireworld_demo.gif "wireworld demo gif")