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
pub struct System {
    pub bodies: Body,
    pub name: String,
}

impl System {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let star = Body::generate_star();

        let star_roche_limit = star.radius * 0.8064;

        let body_count = rng.gen_range(1..=10);

        let mut body_orbit_radii: Vec<f32> = Vec::new();
        let mut body_masses: Vec<f32> = Vec::new();
        let mut satellites: Vec<Vec<i32>> = Vec::new();

        for i in 1..=body_count {
            body_orbit_radii.push(rng.gen_range(2.9..=20.0).ln() + star_roche_limit);
            body_masses.push(rng.gen_range(2.9..=1.57 * 10 ** 142).ln() * 5.97 * 10 ** 12);
        }

        for i in 0..=body_count-1 {

            // Calculate radius of the Hill sphere
            let hill_sphere_radius: f32 = body_orbit_radii[i] * (body_masses[i] /
                (3 * (body_masses[i] + star.mass))).cbrt();

            for j in 0..=body_count-1 {
                // Check if radius is bigger than difference in orbit radius
                let distance = (body_orbit_radii[i] - body_orbit_radii[j]).abs();

                if distance < hill_sphere_radius {
                    // If so, make the lighter body a satellite of the larger body
                    satellites.push(vec![i, j]);
                }
            }
        }

        System {
            bodies: star,
            name: "".to_string(),
        }
    }
}