use std::io;
use crate::dynamo_db;

struct Moon {
    name: String,
}

impl Moon {
    fn new(name: &str) -> Moon {
        Moon {
            name: name.to_string(),
        }
    }
}

struct Planet {
    name: String,
    moons: Vec<Moon>,
}

impl Planet {
    fn new(name: &str) -> Planet {
        Planet {
            name: name.to_string(),
            moons: Vec::new(),
        }
    }

    fn add_moon(&mut self, moon: Moon) {
        self.moons.push(moon);
    }
}

struct SolarSystem {
    planets: Vec<Planet>,
}

impl SolarSystem {
    fn new() -> SolarSystem {
        SolarSystem {
            planets: Vec::new(),
        }
    }

    fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }
}

pub fn add_solar_system() {
    let mut solar_system = SolarSystem::new();

    loop {
        println!("Solar System Name:");
        
        let mut solar_system_name = String::new();
        io::stdin()
            .read_line(&mut solar_system_name)
            .expect("Failed to read line");

        let solar_system_name = solar_system_name.trim();

        if solar_system_name.is_empty() {
            println!("Solar System name cannot be empty");
            continue;
        }

        dynamo_db::save_solar_system(solar_system_name);

        return;
    }
}
