

pub mod obs{
    
    use serde_json::json;

    pub struct TeamUnits {
        pub player_one: Vec<Robot>,
        pub player_two: Vec<Robot>,
    }

    pub struct TeamFactories {
        pub player_one: Vec<Factory>,
        pub player_two: Vec<Factory>,
    }

    pub struct Teams {
        player_one: TeamResources,
        player_two: TeamResources,
    }

    pub struct Observation {
        pub units: TeamUnits,
        pub factories: TeamFactories,
        pub teams: TeamResources,
    }

    pub fn process_obs(obs: &str) -> Observation {
        let obs_json = json!(obs);
        let units = obs_json["units"];
        let factories = obs_json["factories"];
        let teams = obs_json["teams"];

        let player_one_units = units["player_one"];
        let player_two_units = units["player_two"];

        let player_one_factories = factories["player_one"];
        let player_two_factories = factories["player_two"];

        let player_one_team = teams["player_one"];
        let player_two_team = teams["player_two"];

        let mut player_one_units_vec = Vec::new();
        let mut player_two_units_vec = Vec::new();

        let mut player_one_factories_vec = Vec::new();
        let mut player_two_factories_vec = Vec::new();

        for unit in player_one_units {
            let unit = Robot::new(unit);
            player_one_units_vec.push(unit);
        }

        for unit in player_two_units {
            let unit = Robot::new(unit);
            player_two_units_vec.push(unit);
        }

        for factory in player_one_factories {
            let factory = Factory::new(factory);
            player_one_factories_vec.push(factory);
        }

        for factory in player_two_factories {
            let factory = Factory::new(factory);
            player_two_factories_vec.push(factory);
        }

        let player_one_team = TeamResources::new(player_one_team);
        let player_two_team = TeamResources::new(player_two_team);

        let team_units = TeamUnits {
            player_one: player_one_units_vec,
            player_two: player_two_units_vec,
        };

        let team_factories = TeamFactories {
            player_one: player_one_factories_vec,
            player_two: player_two_factories_vec,
        };

        let teams = TeamResources {
            player_one: player_one_team,
            player_two: player_two_team,
        };

        let obs = Observation {
            units: team_units,
            factories: team_factories,
            teams: teams,
        };

        obs
    }

}
    