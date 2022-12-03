
use serde::{Deserialize, Serialize};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let input: &str = "./sample_observation.json";
    
    let _obs_json = json!(input);

    Ok(())
}
