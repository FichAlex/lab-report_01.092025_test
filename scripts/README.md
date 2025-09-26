# Scripts Directory

This directory will contain modular Rust code for game systems.

## Planned Structure:
- `ai_systems.rs` - AI behavior and decision making
- `npc_manager.rs` - NPC lifecycle and interactions
- `player_controller.rs` - Player movement and actions
- `combat_system.rs` - Combat mechanics
- `inventory_system.rs` - Item management
- `procedural/` - Procedural generation modules
  - `terrain_generator.rs`
  - `dungeon_generator.rs`
  - `quest_generator.rs`

## Implementation Notes:
Create modules here and add them to `src/main.rs` using:
```rust
mod scripts {
    pub mod ai_systems;
    pub mod npc_manager;
    // ... other modules
}
```