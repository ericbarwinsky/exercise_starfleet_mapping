use std::collections::HashMap;
use exercise_starfleet_mapping::{Sector, StarSystem, SectorDatabase};

fn main() {
    let mut starfleet_db = SectorDatabase::new();

    let alpha_sector = Sector("Alpha Quadrant".to_string());

    let mut vulcan_resources = HashMap::new();
    vulcan_resources.insert("Dilithium".to_string(), 1500);
    vulcan_resources.insert("Duranium".to_string(), 3000);

    let vulcan_system = StarSystem {
        name: "Vulcan".to_string(),
        resources: vulcan_resources,
    };

    starfleet_db.register_system(alpha_sector, vulcan_system);

    let total_dilithium = starfleet_db.find_resource_total("Dilithium");
    println!("Total Dilithium registered in database: {}", total_dilithium);
}