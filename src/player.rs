use bevy::prelude::*;
use std::collections::HashSet;

const GRID_SIZE: f32 = 16.0; // Size of each cell in pixels

#[derive(Resource)]
pub struct ExploredAreas {
    pub(crate) visited: HashSet<(i32, i32)>, // Stores permanently blacked-out tiles
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    view_radius: f32,
}

#[derive(Component)]
pub struct FogOfWar {
    visited: bool,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(32.0)),
        Player {
            speed: 150.0,
            view_radius: 100.0,
        },
    ));
}

// System to handle player movement - always moving forward
//TODO: movement is broken, doesn't seem to update the player's position
pub fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((player, mut transform)) = player_query.get_single_mut() {
        let mut movement = Vec2::ZERO;
        
        // Use arrow keys for movement
        if input.pressed(KeyCode::ArrowLeft) {
            movement.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            movement.x += 1.0;
        }
        if input.pressed(KeyCode::ArrowUp) {
            movement.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            movement.y -= 1.0;
        }

        // Normalize and apply movement
        if movement != Vec2::ZERO {
            movement = movement.normalize() * player.speed * time.delta_secs();
            transform.translation += movement.extend(0.0);
        }
    }
}

// Update the fog of war system to black out visited areas
pub fn update_fog_of_war(
    player_query: Query<(&Player, &Transform)>,
    mut fog_query: Query<(&mut FogOfWar, &Transform, &mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut explored_areas: ResMut<ExploredAreas>,
) {
    if let Ok((player, player_transform)) = player_query.get_single() {
        for (mut fog, fog_transform, mut material) in fog_query.iter_mut() {
            let distance = fog_transform
                .translation
                .distance(player_transform.translation);

            if distance <= player.view_radius && !fog.visited {
                // Only update when first visiting an area
                fog.visited = true;
                let grid_coords = (
                    (fog_transform.translation.x / GRID_SIZE) as i32,
                    (fog_transform.translation.y / GRID_SIZE) as i32,
                );
                explored_areas.visited.insert(grid_coords);

                // Update material color to black out visited area
                *material = materials.add(ColorMaterial::from(Color::srgba(0.0, 0.0, 0.0, 0.8))).into();
            }
        }
    }
}

// Modify spawn_fog_of_war to start with transparent fog
pub fn spawn_fog_of_war(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let fog_size = GRID_SIZE;
    let grid_size = 50;

    for x in 0..grid_size {
        for y in 0..grid_size {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::NONE))), // Start transparent
                Transform::from_xyz(x as f32 * fog_size, y as f32 * fog_size, 0.5) // Slightly above other elements
                    .with_scale(Vec3::splat(fog_size)),
                FogOfWar { 
                    visited: false,
                },
            ));
        }
    }
}

// Camera follow system
pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
