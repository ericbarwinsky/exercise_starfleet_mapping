use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sector(pub String);

#[derive(Debug, Clone)]
pub struct StarSystem {
    pub name: String,
    pub resources: HashMap<String, u32>,
}

#[derive(Debug, Default)]
pub struct SectorDatabase {
    systems: HashMap<Sector, Vec<StarSystem>>,
}

impl SectorDatabase {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
        }
    }

    pub fn register_system(&mut self, sector: Sector, system: StarSystem) {
        self.systems.entry(sector).or_default().push(system);
    }

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