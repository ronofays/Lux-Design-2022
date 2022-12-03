
pub mod config {

    use crate::src::lux::robots::robots::Robot;
    use crate::src::lux::robots::robots::UnitOptions;

    use crate::src::lux::weather::weather::Weather;

    pub struct EnvConfig {
        pub max_episode_length: u32,
        pub map_size: u32, // 48x48
        pub verbose: u32, // 1
        
        pub validate_action_space: bool, // True
        
        // Game constants
        pub max_transfer_amount: u32, // 1000
        pub min_factories: u32, // 2
        pub max_factories: u32, // 5
        pub cycle_length: u32, // 50
        pub day_length: u32, // 30
        pub unit_action_queue_size: u32, // 20
        pub unit_action_queue_power_cost: UnitOptions,
        
        pub max_rubble: u32, // 100
        pub factory_rubble_after_destruction: u32, // 50
        pub init_water_metal_per_factory: u32, // 100
        pub init_power_per_factory: u32, // 100

        // Lichen
        pub min_lichen_to_spread: u32, // 1
        pub lichen_lost_without_water: u32, // 1
        pub lichen_gain_with_water: u32, // 1
        pub max_lichen_per_tile: u32, // 100
        pub lichen_watering_cost_factor: u32, // 1

        pub bidding_system: bool, // True

        // Factories
        pub factory_processing_rate_water: u32, // 50
        pub ice_water_ratio: u32, // 2
        pub factory_processing_rate_metal: u32, // 50
        pub ore_metal_ratio: u32, // 5
        pub factory_charge: u32, // 50
        pub factory_water_consumption: u32, // 1

        // Robots
        pub robot_light: Robot,
        pub robot_heavy: Robot,

        // Weather
        
    }

    impl EnvConfig {
        pub fn new() -> EnvConfig {
            EnvConfig {
                max_episode_length: 1000,
                map_size: 48,
                verbose: 1,

                validate_action_space: true,

                max_transfer_amount: 1000,
                min_factories: 2,
                max_factories: 5,
                cycle_length: 50,
                day_length: 30,
                unit_action_queue_size: 20,
                unit_action_queue_power_cost: UnitOptions::new(1, 10),

                max_rubble: 100,
                factory_rubble_after_destruction: 50,
                init_water_metal_per_factory: 100,
                init_power_per_factory: 100,

                // Lichen
                min_lichen_to_spread: 1,
                lichen_lost_without_water: 1,
                lichen_gain_with_water: 1,
                max_lichen_per_tile: 100,
                lichen_watering_cost_factor: 1,

                bidding_system: true,

                // Factories
                factory_processing_rate_water: 50,
                ice_water_ratio: 2,
                factory_processing_rate_metal: 50,
                ore_metal_ratio: 5,
                factory_charge: 50,
                factory_water_consumption: 1,

                // Robots
                robot_light: Robot::new_light(),
                robot_heavy: Robot::new_heavy(),

                // Weather
                
            }
        }
    }

}