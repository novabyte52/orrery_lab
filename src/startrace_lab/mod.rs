use bevy::{color::palettes::css::RED, prelude::*};
use magmagrid_debug::generate_debug_magmagrid;

mod magmagrid_debug;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MagmagridGizmos {}

pub struct StartracePlugin;

impl Plugin for StartracePlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<MagmagridGizmos>()
            .add_systems(Startup, setup)
            .add_systems(Update, (draw_example_collection, update_config))
            .add_systems(Startup, generate_debug_magmagrid);
    }
}

const CONFIG_INSTRUCTIONS: &str = r"[L-Alt]- hold to access gizmo shortcuts
    [M]- toggle magmagrid gizmos";

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // example instructions
    commands.spawn((
        Text::new(CONFIG_INSTRUCTIONS),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn draw_example_collection(
    // mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MagmagridGizmos>,
    // time: Res<Time>,
) {
    my_gizmos.cuboid(Transform::from_xyz(0.0, 1.0, 0.0), RED);
}

fn update_config(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (config, _type_id) = config_store.config_mut::<MagmagridGizmos>();

    if keyboard.pressed(KeyCode::AltLeft) {
        // adjust custom gizmo settings
        if keyboard.just_pressed(KeyCode::KeyM) {
            config.enabled ^= true;
        }
    }
}
