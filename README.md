# Tank 2D

A 2D tank battle game built with the Bevy game engine.

<video width="640" height="360" controls>
  <source src="assets/tank_assets/tank2d.mov" type="video/quicktime">
  Your browser does not support the video tag.
</video>

*Tank 2D in action - battle against enemy tanks!*

## Description

Tank 2D is an action-packed tank fighting game where you control a tank and battle against enemy tanks in a top-down 2D environment. Navigate through the battlefield, avoid obstacles, and eliminate enemy tanks with well-aimed shots.

## Features

- Control a player tank with smooth movement and rotation
- Battle against enemy tanks
- Fire projectiles to destroy enemies
- Tile-based map system
- Game state management with pause functionality
- Simple and intuitive controls

## Controls

- **W/A/S/D** - Move your tank forward, rotate left, move backward, and rotate right
- **J** - Fire shells
- **ESC** - Pause game
- **SPACE** - Continue game when paused

## Technologies

- [Bevy](https://bevyengine.org/) - A refreshingly simple data-driven game engine built in Rust
- [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) - A tilemap rendering plugin for Bevy

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) - Make sure you have Rust installed on your system.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/nkien0204/tank-2d.git
   cd tank-2d
   ```

2. Run the game:
   ```bash
   cargo run --release
   ```

## Game Structure

The game is organized into various plugins that handle different aspects of gameplay:

- `CameraPlugin` - Manages the 2D camera view
- `MapPlugin` - Creates and manages the game map
- `TankPlugin` - Handles player tank creation and controls
- `EnemyPlugin` - Manages enemy tanks and their behavior
- `MovementPlugin` - Handles movement physics for all entities
- `CollisionDetectionPlugin` - Detects and responds to collisions between entities
- `GameStatePlugin` - Manages game states (menu, playing, paused, etc.)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Tank assets from [Kenney's Top-down Tanks Redux](https://kenney-assets.itch.io/top-down-tanks-redux)
