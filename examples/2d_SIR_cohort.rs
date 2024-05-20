use bevy::prelude::*;
use bevy_life::{CellState, CellularAutomatonPlugin, MooreCell2d, ComplexCell2d, SimulationBatch};
use rand::Rng;
use rand_distr::{Binomial, Distribution};

#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct SIRCohort {
    S: u64,
    I: u64,
    R: u64
}

impl SIRCohort {
    fn N(&self) -> u64 {
        self.S + self.I + self.R
    }
}

impl CellState for SIRCohort {
    fn new_cell_state<'a>(&self, neighbor_cells: impl Iterator<Item = &'a Self>) -> Self {

        let neighbor_I_count = neighbor_cells.map(|state| state.I).sum::<u64>();
        let total_I_weighted = self.I as f64/2.0 + neighbor_I_count as f64/16.0;

        let I_recoveries = Binomial::new(self.I, 0.5).unwrap().sample(&mut rand::thread_rng());
        let R_deaths = Binomial::new(self.R, 0.05 / 26.).unwrap().sample(&mut rand::thread_rng());
        let I_new = Binomial::new(self.S, 0.9 * total_I_weighted / self.N() as f64).unwrap().sample(&mut rand::thread_rng());

        SIRCohort {
            S: self.S + R_deaths - I_new, 
            I: self.I - I_recoveries + I_new, 
            R: self.R + I_recoveries - R_deaths
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
        .add_plugins(CellularAutomatonPlugin::<ComplexCell2d, SIRCohort>::default())
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

                    let state = Some(SIRCohort {S: 500, I: 10, R: 490});

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
    mut query: Query<(&SIRCohort, &mut Sprite), 
    // Changed<SIRCohort>
    >,
) {
    query
        .par_iter_mut()
        .for_each(|(state, mut sprite)| {

            if (state.I as f64 / state.N() as f64) == 0. {
                sprite.color = Color::rgb(0., 0.8 * state.S as f32 / state.N() as f32, 0.);
            } else {
                sprite.color = Color::rgb(0., 0.8 * state.S as f32 / state.N() as f32, 100. * state.I as f32 / state.N() as f32);
                // sprite.color = Color::CYAN;
            }
        });
}
