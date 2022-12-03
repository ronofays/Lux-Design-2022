
pub mod factory {

    use crate::src::lux::cargo::cargo::UnitCargo;
    use crate::src::lux::robots::robots::Robot;
    use crate::src::lux::config::env_config::EnvConfig;

    pub struct Factory {
        pub team_id: u32,
        pub unit_id: String,
        pub strain_id: u32,
        pub power: u32,
        pub cargo: UnitCargo,
        pub pos: (u32, u32),
        pub env_config: EnvConfig,
    }


}