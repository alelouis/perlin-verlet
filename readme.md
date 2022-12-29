# **perlin-verlet**
![rustc](https://img.shields.io/badge/rustc-1.61.0-important)

Particle flow in Perlin fields using verlet integration.

<p align="center">
  <img width="80%" src="./example.png" alt="animated" />
</p>

## **How to run**
Clone the repository.
```bash
gh clone alelouis/perlin-verlet
```
Then run main binary using cargo.
```bash
cargo run
```
Finally, use the following keybinds for interaction:
- `A`: Draw mode
- `G`: Spawn 10000 random particles
- `C`: Clear particles
- `Space`: Pause / Resume