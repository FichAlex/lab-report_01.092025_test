use bevy::prelude::*;

// Marker component for the player cube
#[derive(Component)]
struct Player;

// Marker component for the ground plane
#[derive(Component)]
struct Ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Vaultworn - Medieval RPG Prototype".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_scene, setup_camera, setup_player, setup_ground))
        .add_systems(Update, (rotate_player, camera_controls))
        .run();
}

/// Set up the basic scene with lighting
fn setup_scene(mut commands: Commands) {
    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.9, 0.9, 1.0),
        brightness: 300.0,
    });

    // Add directional light (sun)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.8),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Set up the camera with basic controls
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Set up the player as a spinning cube
fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a colorful cube to represent the player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.2, 0.2), // Red color for visibility
                metallic: 0.1,
                perceptual_roughness: 0.9,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0), // Slightly above ground
            ..default()
        },
        Player,
    ));
}

/// Set up a simple ground plane
fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a large plane for the ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.5, 0.3), // Green ground
                metallic: 0.0,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
        Ground,
    ));
}

/// Rotate the player cube to show it's animated
fn rotate_player(mut player_query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    for mut transform in &mut player_query {
        transform.rotate_y(time.delta_seconds() * 1.0); // Rotate 1 radian per second
    }
}

/// Basic camera controls (placeholder for future expansion)
fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    time: Res<Time>,
) {
    for mut camera_transform in &mut camera_query {
        let mut movement = Vec3::ZERO;
        let speed = 5.0;
        
        // Basic WASD movement (placeholder - will be expanded for full camera system)
        if keyboard.pressed(KeyCode::KeyW) {
            movement += *camera_transform.forward();
        }
        if keyboard.pressed(KeyCode::KeyS) {
            movement -= *camera_transform.forward();
        }
        if keyboard.pressed(KeyCode::KeyA) {
            movement -= *camera_transform.right();
        }
        if keyboard.pressed(KeyCode::KeyD) {
            movement += *camera_transform.right();
        }
        
        camera_transform.translation += movement * speed * time.delta_seconds();
    }
}

// TODO: Expand this section for advanced RPG features
//
// === FUTURE EXPANSION AREAS ===
//
// 1. **AI Systems** (scripts/ai_systems.rs):
//    - Behavior trees for NPCs
//    - Pathfinding using A* or flow fields
//    - State machines for different AI behaviors (idle, patrol, combat, flee)
//    - Group AI for coordinated behaviors
//
// 2. **NPC Management** (scripts/npc_manager.rs):
//    - NPC spawning and lifecycle management
//    - Dialogue system with branching conversations
//    - Quest giver functionality
//    - Dynamic NPC schedules (day/night cycles)
//    - Faction relationships and reputation systems
//
// 3. **Procedural Generation** (scripts/procedural/):
//    - Terrain generation using noise functions
//    - Dungeon generation algorithms
//    - Quest generation system
//    - Item and loot generation
//    - Settlement and structure placement
//
// 4. **Game Systems to Add**:
//    - Player inventory and equipment system
//    - Combat system with skills and abilities
//    - Magic system with spell crafting
//    - Crafting and resource gathering
//    - Save/load game functionality
//    - Audio management for ambient sounds and music
//
// 5. **Scene Organization** (scenes/):
//    - Create separate scene files for different areas
//    - Implement scene transition system
//    - Loading screens and asset streaming
//
// 6. **Asset Pipeline** (assets/):
//    - 3D model loading and management
//    - Texture atlas generation
//    - Audio asset organization
//    - Material and shader system
//
// 7. **Performance Optimizations**:
//    - Level-of-detail (LOD) systems
//    - Frustum culling
//    - Spatial partitioning for large worlds
//    - Asset streaming and unloading