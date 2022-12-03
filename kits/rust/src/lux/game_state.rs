

pub mod game_state {

    use ndarray::Array2;

    use crate::src::lux::config::env_config::EnvConfig;
    use crate::src::lux::factory::factory::Factory;
    use crate::src::lux::robot::robot::Robot;

    pub struct Board {
        pub rubble: Array2<u32>,
        pub ice: Array2<u32>,
        pub ore: Array2<u32>,
        pub lichen: Array2<u32>,
        pub lichen_strains: Array2<u32>,
        pub factory_occupancy_map: Array2<u32>,
        pub factories_per_team: Array2<u32>,
        pub spawns: Array2<u32>,
    }

    pub impl Board {
        pub fn new(env_cfg: EnvConfig) -> Board {
            let size = env_cfg.map_size;
            Board {
                rubble: Array2::zeros((size, size)),
                ice: Array2::zeros((size, size)),
                ore: Array2::zeros((size, size)),
                lichen: Array2::zeros((size, size)),
                lichen_strains: Array2::zeros((size, size)),
                factory_occupancy_map: Array2::zeros((size, size)),
                factories_per_team: Array2::zeros((size, size)),
                spawns: Array2::zeros((size, size)),
            }
        }
    }

    pub struct GameState {
        // A GameState object at step env_steps.
        pub env_steps: u32,
        pub env_cfg: EnvConfig,
        pub board: Board,
        pub weather_schedule: ndarray::Array2<u32>,
        pub robots: Vec<Unit>,
        pub factories: Vec<Factory>,
        pub teams: Vec<Team>,
    }

    impl GameState {
        pub fn new() -> GameState {
            GameState {
                env_steps: 0,
                env_cfg: EnvConfig::new(),
                board: Board::new(env_cfg),
                weather_schedule: ndarray::Array2::zeros((48, 48)),
                robots: Vec::new(),
                factories: Vec::new(),
                teams: Vec::new(),
            }
        }

        pub fn real_env_steps(&self) -> u32 {
            match self.env_cfg.bidding_system {
                true => self.env_steps - (self.board.factories_per_team + 2),
                false => self.env_steps,
            }
        }

        pub fn is_day(&self) -> bool {
            (self.real_env_steps() % self.env_cfg.cycle_length) < self.env_cfg.day_length
        }
    }
}