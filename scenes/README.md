# Scenes Directory

This directory will contain scene definition files for different areas of the game world.

## Planned Structure:
- `town_square.ron` - Main town area
- `forest_area.ron` - Forest exploration area
- `dungeon_entrance.ron` - Dungeon entry points
- `castle_interior.ron` - Castle rooms and halls

## Scene Format:
Scenes can be defined using Bevy's scene system with RON (Rusty Object Notation) format:

```rust
// Example scene loading
fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene = asset_server.load("scenes/town_square.ron");
    commands.spawn(DynamicSceneBundle { scene, ..default() });
}
```

## Scene Transition System:
Future implementation will include:
- Seamless scene transitions
- Loading screens
- Asset preloading for smooth gameplay