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

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new().add_system(hello_world).run();
}
