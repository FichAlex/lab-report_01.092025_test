# Assets Directory

This directory contains all game assets for Vaultworn.

## Structure:
- `models/` - 3D models (.gltf, .glb files)
- `textures/` - Texture files (.png, .jpg files)
- `sounds/` - Audio files (.ogg, .wav files)
- `materials/` - Material definitions and shaders

## Usage:
Place your asset files in the appropriate subdirectories. The game will load assets from these locations using Bevy's asset system.

### Example asset loading in code:
```rust
// Load a 3D model
let model: Handle<Scene> = asset_server.load("models/character.gltf#Scene0");

// Load a texture
let texture: Handle<Image> = asset_server.load("textures/grass.png");

// Load a sound
let sound: Handle<AudioSource> = asset_server.load("sounds/footstep.ogg");
```