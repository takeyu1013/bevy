use bevy::prelude::*;

struct PlayerMoveEvent;

struct Player {
    transform: Transform,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            transform: Transform::default().looking_at(Vec3::X, Vec3::Y),
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let translation = self.transform.translation;
        write!(
            f,
            "X {:.3}, Y: {:.3}, Z: {:.3}",
            translation.x, translation.y, translation.z
        )
    }
}

fn setup_lights(mut commands: Commands) {
    commands.spawn_bundle(DirectionalLightBundle::default());
}

fn setup_cameras(mut commands: Commands, player: Res<Player>) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform {
            translation: player.transform.translation + Vec3::Y,
            rotation: player.transform.rotation,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::BLACK.into()),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });
}

fn setup_texts(mut commands: Commands, asset_server: Res<AssetServer>, player: Res<Player>) {
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(10.0),
                bottom: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        text: Text::with_section(
            format!("{}", player.into_inner()),
            TextStyle {
                font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Left,
                ..default()
            },
        ),
        ..default()
    });
}

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new().add_system(hello_world).run();
}
