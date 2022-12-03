
// This will probably be completely redone...

pub mod weather {

    // Enumerate weather types
    pub enum WeatherType {
        none,
        mars_quake,
        cold_snap,
        dust_storm,
        solar_flare,
    }

    // Object for weather state
    pub struct Weather {
        pub kind: WeatherType,
        pub name: String,
        pub rubble: (u32, u32), // (light, heavy)
        pub power_consumption: f32,
        pub power_gain: f32,
        pub time_range: (u32, u32),
    }

    // Constant weather states

    // Normal weather
    pub const NONE: Weather = Weather {
        kind: WeatherType::none,
        name: "NONE".to_string(),
        rubble: (0, 0),
        power_consumption: 1.0,
        power_gain: 1.0,
        time_range: (0, 0),
    };

    // Mars quake - causes rubble under moving robots
    pub const MARS_QUAKE: Weather = Weather {
        kind: WeatherType::mars_quake,
        name: "MARS_QUAKE".to_string(),
        rubble: (1, 10),
        power_consumption: 1.0,
        power_gain: 1.0,
        time_range: (1, 5),
    };

    // Cold snap - debuffs robot power efficiency
    pub const COLD_SNAP: Weather = Weather {
        kind: WeatherType::cold_snap,
        name: "COLD_SNAP".to_string(),
        rubble: (0, 0),
        power_consumption: 2.0,
        power_gain: 1.0,
        time_range: (10, 30),
    };

    // Dust storm - debuffs power generation
    pub const DUST_STORM: Weather = Weather {
        kind: WeatherType::dust_storm,
        name: "DUST_STORM".to_string(),
        rubble: (0, 0),
        power_consumption: 1.0,
        power_gain: 0.5,
        time_range: (10, 30),
    };

    // Solar flare - buffs power generation
    pub const SOLAR_FLARE: Weather = Weather {
        kind: WeatherType::solar_flare,
        name: "SOLAR_FLARE".to_string(),
        rubble: (0, 0),
        power_consumption: 1.0,
        power_gain: 2.0,
        time_range: (10, 30),
    };

}