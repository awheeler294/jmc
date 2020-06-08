# Janus Mining Colony
A roguelike mining colony game

Currently features a very basic procedural terrain generator and scrollable, zoomable map.

### Building
`cargo run --release`

### Controls
Up, Down, Left, Right: Scroll Camera
W, A, S, D: Move Player
[, ]: zoom out, in
comma, period: Move camera up, down one level

### Roadmap
* Rendering system and basic map generation works, before I get further into map generation I want to integrate an ECS, probably [Specs](https://docs.rs/specs/0.7.0/specs/). I need a more efficent data structure for the map, three layers of hashmaps is fine at a regular zoom level, but I want to be able to zoom out quite a bit. I believe storing map tiles as Specs entities will help there since Specs is very efficient at storing and accesing entities.

* Re-assess if Quicksilver is the best engine for my needs, or should I switch to something designed specifically for roguelikes like [Bracketlib](https://github.com/thebracket/bracket-lib).

* Character terrain interactions

Special thanks to <https://github.com/tomassedovic/quicksilver-roguelike> for providing an excellent beginnig to building a roguelike in rust.
