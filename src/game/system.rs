// Ad Infinitum - a 4x strategy set in a procedurally generated galaxy
// Copyright (C) 2024 Egor Kosachev
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::game::body::{Body, StarType};
use rand::Rng;
use crate::game::body::PlanetZone::{HabitableZone, InnerRing, OuterRing};

#[derive(Clone)]
pub struct System {
    pub bodies: Vec<Body>,
    pub star: Body,
    pub name: String,
}

impl System {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let mut star = Body::generate_star();

        let star_roche_limit = star.radius * 1.6;

        let body_count = rng.gen_range(1..=10);

        let mut planets: Vec<Body> = Vec::new();

        match body_count {
            1..=3 => {
                planets.append(&mut Body::generate_planet_with_count(&HabitableZone, 1));
                planets.append(&mut Body::generate_planet_with_count(&OuterRing, body_count - 1));
            }
            4..=5 => {
                planets.append(&mut Body::generate_planet_with_count(&InnerRing, 1));
                planets.append(&mut Body::generate_planet_with_count(&HabitableZone, 1));
                planets.append(&mut Body::generate_planet_with_count(&OuterRing, body_count - 2));
            }
            6..=7 => {
                planets.append(&mut Body::generate_planet_with_count(&InnerRing, 1));
                planets.append(&mut Body::generate_planet_with_count(&HabitableZone, 2));
                planets.append(&mut Body::generate_planet_with_count(&OuterRing, body_count - 3));
            }
            8..=10 => {
                planets.append(&mut Body::generate_planet_with_count(&InnerRing, 2));
                planets.append(&mut Body::generate_planet_with_count(&HabitableZone, 2));
                planets.append(&mut Body::generate_planet_with_count(&OuterRing, body_count - 4));
            }

            _ => unreachable!()
        }
        
        let mut bodies = Vec::new();
        
        for mut planet in planets {
            planet.orbit_radius = Some(rng.gen_range(2.9..=20.0) * 10.0f32.powi(10) + star_roche_limit);
            planet.orbit_period = Some(f32::sqrt(planet.orbit_radius.unwrap().powi(2) / 7.5));
            bodies.push(planet);
        }

        System {
            bodies: bodies.clone(),
            star,
            name: "Not solar system".to_string(),
        }
    }
}