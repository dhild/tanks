use specs;

mod components;
mod inertia;
mod gravity;

pub type Delta = f32;
pub type Planner = specs::Planner<Delta>;

pub use self::components::*;
pub use self::inertia::InertiaSystem;
pub use self::gravity::{GRAVITY, GravitySystem};
