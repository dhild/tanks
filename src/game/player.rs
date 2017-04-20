use physics::*;
use rand::{self, Rng};
use specs;
use tank;
use terrain;

#[derive(Debug)]
pub struct Players {
    players: Vec<Player>,
}

const COLORS: [[f32; 3]; 4] = [
    [1.0, 0.0, 0.0], // red
    [0.0, 0.0, 1.0], // blue
    [1.0, 1.0, 0.0], // yellow
    [0.8, 0.0, 0.8], // purple
];

impl Players {
    pub fn create(world: &mut specs::World, count: usize) {
        let dx = {
            let dim = world.read_resource_now::<Dimensions>();
            dim.game_width() as f32 / ((count + 1) as f32)
        };
        let mut rng = rand::thread_rng();
        let mut players = Vec::new();
        for (i, color) in COLORS.iter().enumerate() {
            let x = (i as f32 * dx) + rng.gen_range(dx / 2.0, 3.0 * dx / 2.0);
            let drawable = tank::Drawable::new(*color);

            let terrain = world.read_resource_now::<terrain::Terrain>();
            let terrain_height = terrain.get_height(x);
            let normal_dir = terrain.get_normal_dir(x);

            let entity = world
                .create()
                .with(tank::Tank::new())
                .with(drawable)
                .with(Position::new(x, terrain_height, normal_dir, 20.0))
                .with(Velocity::new())
                .build();
            players.push(Player {
                             player_number: (i as u8) + 1,
                             tank_id: entity,
                         });
        }
        world.add_resource(Players { players: players });
    }

    pub fn get_remaining(&self, world: &specs::World) -> Vec<Player> {
        self.players
            .iter()
            .filter(|p| world.is_alive(p.tank_id))
            .cloned()
            .collect()
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Player {
    player_number: u8,
    tank_id: specs::Entity,
}

impl Player {
    pub fn player_number(&self) -> u8 {
        self.player_number
    }
    pub fn id(&self) -> specs::Entity {
        self.tank_id
    }
}
