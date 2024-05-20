use bevy::prelude::*;
use bevy_life::{CellState, CellularAutomatonPlugin, MooreCell2d, ComplexCell2d, SimulationBatch};
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub enum SIR {
    S(f32),
    I,
}

impl CellState for SIR {
    fn new_cell_state<'a>(&self, neighbor_cells: impl Iterator<Item = &'a Self>) -> Self {
        let count = neighbor_cells.filter(|state| *state == &Self::I).count();
        let mut rng = rand::thread_rng();

        match (self, rng.gen_range(0.0..=1.0)) {
            (Self::I, x) if x < 0.5 => Self::S(0.0),
            (Self::S(s), x) if x < (*s * 0.4 * count as f32) => Self::I,
            (Self::S(s), _) => Self::S(*s + 0.015 * (1.0 - *s)),
            _ => *self
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SIR".to_string(),
                resolution: [1200.0, 800.0].into(),
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(CellularAutomatonPlugin::<MooreCell2d, SIR>::default())
        .add_plugins(CellularAutomatonPlugin::<ComplexCell2d, SIR>::default())
        .insert_resource(SimulationBatch)
        .add_systems(Startup, (setup_camera, setup_map))
        .add_systems(Update, color_sprites)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_map(mut commands: Commands) {
    spawn_map(&mut commands);
}

fn spawn_map(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (600, 400);
    let sprite_size = 8.;
    let color = Color::rgba(0., 0., 0., 0.);

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            -(size_x as f32 * sprite_size) / 2.,
            -(size_y as f32 * sprite_size) / 2.,
            0.,
        )))
        .with_children(|builder| {
            for y in 0..=size_y {
                for x in 0..=size_x {
                    let state = match rng.gen_range(0.0..=1.0) {
                        x if x < 0.15 => Some(SIR::I),
                        x if x < 0.8 => Some(SIR::S(rng.gen_range(0.0..=1.0))),
                        _ => None,
                    };
                    if let Some(state) = state {
                        builder.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(sprite_size*rng.gen_range(0.0..=1.0))),
                                    color,
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    sprite_size * x as f32,
                                    sprite_size * y as f32,
                                    0.,
                                ),
                                ..default()
                            },
                            // MooreCell2d::new(IVec2::new(x, y)),
                            ComplexCell2d::new(IVec2::new(x, y)),
                            state,
                        ));
                    }
                }
            }
        });
    println!("map generated");
}

pub fn color_sprites(
    mut query: Query<(&SIR, &mut Sprite), Changed<SIR>>,
) {
    query
        .par_iter_mut()
        .for_each(|(state, mut sprite)| match state {
            SIR::S(s) => sprite.color = Color::rgb(0., *s * 0.8, 0.),
            SIR::I => sprite.color = Color::CYAN, // CYAN, ORANGE_RED
        });
}
