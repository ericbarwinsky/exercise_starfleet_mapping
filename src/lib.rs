use std::collections::HashMap;

/// Represents a galactic sector in the Starfleet database.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sector(pub String);

/// Represents a star system containing various resources.
#[derive(Debug, Clone)]
pub struct StarSystem {
    /// The name of the star system.
    pub name: String,
    /// A mapping of resource names to their respective quantities.
    pub resources: HashMap<String, u32>,
}

/// A database mapping sectors to the star systems they contain.
#[derive(Debug, Default)]
pub struct SectorDatabase {
    systems: HashMap<Sector, Vec<StarSystem>>,
}

impl SectorDatabase {
    /// Creates a new, empty sector database.
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
        }
    }

    /// Registers a new star system within a specific sector.
    pub fn register_system(&mut self, sector: Sector, system: StarSystem) {
        self.systems.entry(sector).or_default().push(system);
    }

    /// Calculates the total amount of a specific resource across all registered systems.
    pub fn find_resource_total(&self, resource_name: &str) -> u32 {
        let mut total = 0;
        for systems in self.systems.values() {
            for system in systems {
                if let Some(&amount) = system.resources.get(resource_name) {
                    total += amount;
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_mapping() {
        let mut db = SectorDatabase::new();
        let sector = Sector("Beta Quadrant".to_string());

        let mut resources = HashMap::new();
        resources.insert("Latinum".to_string(), 500);

        let system = StarSystem {
            name: "Ferenginar".to_string(),
            resources,
        };

        db.register_system(sector, system);

        assert_eq!(db.find_resource_total("Latinum"), 500);
        assert_eq!(db.find_resource_total("Unknown"), 0);
    }
}