use bevy::prelude::*;

use crate::{dialogue::DialogueEvent, player::Player};

pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 480.0;
pub const TILE_SIZE: f32 = 32.0; // Standard tile size for level design

#[derive(Resource)]
pub struct CurrentLevel {
    pub index: usize,
    pub completed: bool,
}

#[derive(Component)]
pub struct LevelBounds {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Resource)]
pub struct LevelData {
    pub levels: Vec<LevelConfig>,
}

pub struct LevelConfig {
    pub bounds: Vec2,
    pub spawn_point: Vec2,
    pub required_dialogue: String,
}

impl Default for LevelData {
    fn default() -> Self {
        Self {
            levels: vec![
                // Level 1: Single screen (600x480)
                LevelConfig {
                    bounds: Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    spawn_point: Vec2::new(WINDOW_WIDTH / 4.0, WINDOW_HEIGHT / 4.0),
                    required_dialogue: "level1_complete".to_string(),
                },
                // Level 2: Double width (1200x480)
                LevelConfig {
                    bounds: Vec2::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT),
                    spawn_point: Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
                    required_dialogue: "level2_complete".to_string(),
                },
                // Level 3: Double width and height (1200x960)
                LevelConfig {
                    bounds: Vec2::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0),
                    spawn_point: Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
                    required_dialogue: "level3_complete".to_string(),
                },
            ],
        }
    }
}

pub fn spawn_current_level(
    mut commands: Commands,
    level_data: Res<LevelData>,
    current_level: Res<CurrentLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let level = &level_data.levels[current_level.index];
    let min = Vec2::new(-level.bounds.x / 2.0, -level.bounds.y / 2.0);
    let max = Vec2::new(level.bounds.x / 2.0, level.bounds.y / 2.0);

    // Spawn level boundaries visualization
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.1))),
        Transform::from_xyz(0.0, 0.0, -0.1).with_scale(level.bounds.extend(1.0)),
        LevelBounds { min, max },
    ));
}

pub fn check_level_completion(
    mut current_level: ResMut<CurrentLevel>,
    mut dialogue_events: EventReader<DialogueEvent>, // You'll need to create this event
    level_data: Res<LevelData>,
) {
    for event in dialogue_events.read() {
        let level = &level_data.levels[current_level.index];
        if event.id == level.required_dialogue {
            current_level.completed = true;
            if current_level.index < level_data.levels.len() - 1 {
                current_level.index += 1;
                current_level.completed = false;
            }
        }
    }
}

pub fn enforce_level_bounds(
    mut player_query: Query<&mut Transform, With<Player>>,
    bounds_query: Query<&LevelBounds>,
) {
    if let (Ok(mut player_transform), Ok(bounds)) =
        (player_query.get_single_mut(), bounds_query.get_single())
    {
        player_transform.translation.x = player_transform
            .translation
            .x
            .clamp(bounds.min.x, bounds.max.x);
        player_transform.translation.y = player_transform
            .translation
            .y
            .clamp(bounds.min.y, bounds.max.y);
    }
}
