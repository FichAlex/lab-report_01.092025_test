# Vaultworn - Medieval RPG Prototype

A prototype open-world medieval RPG built with the Bevy game engine in Rust.

## Features

- **Basic 3D Environment**: A main window with a 3D camera, spinning player cube, and ground plane
- **Modular Architecture**: Organized folder structure for future expansion
- **Performance Optimized**: Configured for development with dynamic linking and optimized builds

## Project Structure

```
vaultworn/
├── src/
│   └── main.rs          # Main application entry point
├── assets/              # Game assets
│   ├── models/          # 3D models (.gltf, .glb)
│   ├── textures/        # Texture files (.png, .jpg)
│   ├── sounds/          # Audio files (.ogg, .wav)
│   └── materials/       # Material definitions and shaders
├── scripts/             # Modular game systems (future expansion)
├── scenes/              # Scene definition files (.ron)
└── Cargo.toml          # Project configuration
```

## Getting Started

### Prerequisites

- Rust 1.89.0 or later
- System dependencies for graphics (OpenGL/Vulkan/DirectX)

### Building and Running

```bash
# Check if the project compiles
cargo check

# Build the project
cargo build

# Run the game (requires graphics support)
cargo run
```

## Current Implementation

The prototype currently includes:

- **Main Window**: 1280x720 resizable window titled "Vaultworn - Medieval RPG Prototype"
- **3D Camera**: Positioned at (0, 5, 10) looking at the origin with WASD movement controls
- **Player Representation**: A red spinning cube at (0, 0.5, 0)
- **Ground Plane**: A 20x20 green plane representing the world surface
- **Lighting**: Ambient lighting and directional light (sun) with shadows

## Future Expansion Areas

The codebase includes detailed comments for future expansion in these areas:

### AI Systems (`scripts/ai_systems.rs`)
- Behavior trees for NPCs
- Pathfinding algorithms (A*, flow fields)
- State machines for AI behaviors
- Group AI coordination

### NPC Management (`scripts/npc_manager.rs`)
- NPC lifecycle management
- Dialogue system with branching conversations
- Quest systems
- Faction relationships

### Procedural Generation (`scripts/procedural/`)
- Terrain generation using noise functions
- Dungeon generation algorithms
- Dynamic quest generation
- Procedural item and loot systems

### Core Game Systems
- Inventory and equipment system
- Combat mechanics with skills
- Magic system and spell crafting
- Resource gathering and crafting
- Save/load functionality

## Controls

- **WASD**: Move camera around the scene
- **Mouse**: Look around (to be implemented)
- **ESC**: Exit game (to be implemented)

## Technical Notes

- Built with Bevy 0.14
- Uses custom feature selection to minimize dependencies
- Optimized for development with dynamic linking
- Cross-platform graphics support (OpenGL, Vulkan, DirectX)
- Audio features disabled for headless environments

## Contributing

This is a prototype project designed to demonstrate the foundation for a medieval RPG. The modular structure allows for easy expansion of game systems.
