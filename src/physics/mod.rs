use specs;

mod components;
mod inertia;
mod settle;

pub type Delta = f32;
pub type Planner = specs::Planner<Delta>;

pub use self::components::{Position, Velocity};
pub use self::inertia::InertiaSystem;
pub use self::settle::SettleSystem;
