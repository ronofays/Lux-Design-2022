
pub mod robots {

    use crate::src::lux::config::env_config::EnvConfig;
    use crate::src::lux::cargo::cargo::UnitCargo;

    use crate::src::lux::actions::Action;

    pub enum RobotType {
        Light,
        Heavy,
    }

    pub struct RobotConfig {
        pub unit_type: RobotType,
        pub metal_cost: u32,
        pub power_cost: u32,
        pub cargo_space: u32,
        pub battery_capacity: u32,
        pub charge: u32,
        pub init_power: u32,
        pub move_cost: u32,
        pub rubble_movement_cost: u32,
        pub dig_cost: u32,
        pub dig_rubble_removed: u32,
        pub dig_resource_gain: u32,
        pub dig_lichen_removed: u32,
        pub self_destruct_cost: u32,
        pub rubble_after_destruction: u32,
    }

    pub impl RobotConfig {
        pub const LIGHT_CONFIG: RobotConfig = RobotConfig {
            metal_cost: 10,
            power_cost: 20,
            cargo_space: 100,
            battery_capacity: 150,
            charge: 1,
            init_power: 50,
            move_cost: 1,
            rubble_movement_cost: 0,
            dig_cost: 5,
            dig_rubble_removed: 1,
            dig_resource_gain: 2,
            dig_lichen_removed: 10,
            self_destruct_cost: 5,
            rubble_after_destruction: 1,
        };
        
        pub const HEAVY_CONFIG: RobotConfig = RobotConfig {
            metal_cost: 100,
            power_cost: 500,
            cargo_space: 1000,
            battery_capacity: 3000,
            charge: 10,
            init_power: 500,
            move_cost: 20,
            rubble_movement_cost: 1,
            dig_cost: 100,
            dig_rubble_removed: 10,
            dig_resource_gain: 20,
            dig_lichen_removed: 100,
            self_destruct_cost: 100,
            rubble_after_destruction: 10,
            };
        }
    

    pub struct Robot {
        pub team_id: u32,
        pub unit_id: String,
        pub power: u32,
        pub unit_type: RobotType,
        pub pos: (u32, u32),
        pub cargo: UnitCargo,
        pub action_queue: Vec<Action>,
    }

    pub impl Robot {
        // robot constructor
        pub fn new(team_id: u32,
                     unit_id: String,
                     unit_type: RobotType,
                     pos: (u32, u32)) -> Robot {
                    
            let robot_config = match unit_type {
                RobotType::Light => RobotConfig::LIGHT_CONFIG,
                RobotType::Heavy => RobotConfig::HEAVY_CONFIG,
            };

            Robot {
                team_id,
                unit_id,
                power: robot_config.init_power,
                unit_type,
                pos,
                cargo: UnitCargo::new(),
                action_queue: Vec::new(),
            }
        }            
    }

    // Help organize places where values depend on the unit type.
    pub struct UnitOptions {
        pub light: u32,
        pub heavy: u32,
    }
    
    pub impl UnitOptions {
        pub fn new(light: u32, heavy: u32) -> UnitOptions {
            UnitOptions {
                light,
                heavy,
            }
        }
    }
}