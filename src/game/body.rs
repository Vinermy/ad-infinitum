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

use std::fmt::{Display, Formatter};
use rand::Rng;

pub enum BodyType {
    Star(StarType),
    Planet(PlanetType),
    Moon,
    Ring,
}

#[derive(Debug)]
pub enum StarType {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

pub enum PlanetType {
    AsteroidRing,
    Earthlike,
    Ice,
    Rock,
    Desert,
    GasGiant,
}

pub enum PlanetZone {
    InnerRing,
    HabitableZone,
    OuterRing,
}

impl From<StarType> for char {
    fn from(star_type: StarType) -> char {
        match star_type {
            StarType::O => { 'O' }
            StarType::B => { 'B' }
            StarType::A => { 'A' }
            StarType::F => { 'F' }
            StarType::G => { 'G' }
            StarType::K => { 'K' }
            StarType::M => { 'M' }
        }
    }
}

pub struct Orbit {
    pub radius: f32,
    pub period: f32,
    pub body: Body
}
pub struct Body {
    pub name: String,
    pub kind: BodyType,
    pub radius: f32,
    pub mass: f32,
    pub children: Vec<Orbit>,
}

impl Body {
    pub fn generate_star() -> Self {
        let mut rng = rand::thread_rng();

        let class: StarType = match rng.gen_range(0..=100) {
            0..1     => StarType::O,  // Oh
            1..2     => StarType::B,  // Be
            2..3     => StarType::A,  // A
            3..6     => StarType::F,  // Fine
            6..14    => StarType::G,  // Girl
            14..26   => StarType::K,  // Kiss
            26..=100 => StarType::M,  // Me
            _ => unreachable!()
        };

        let mass: f32 = rng.gen_range(
            match class {
                StarType::O => { 16.0..=150.0 }
                StarType::B => { 2.10..=16.00 }
                StarType::A => { 1.40..=2.100 }
                StarType::F => { 1.04..=1.400 }
                StarType::G => { 0.80..=1.040 }
                StarType::K => { 0.45..=0.800 }
                StarType::M => { 0.08..=0.450 }
            }
        ) * 1.98 * 10 ** 18;

        let radius: f32 = rng.gen_range(
            match class {
                StarType::O => { 6.60..=100.0 }
                StarType::B => { 1.80..=6.600 }
                StarType::A => { 1.40..=1.800 }
                StarType::F => { 1.15..=1.400 }
                StarType::G => { 0.96..=1.150 }
                StarType::K => { 0.70..=0.960 }
                StarType::M => { 0.10..=0.700 }
            }
        ) * 6.957 * 10 ** 5;

        let name = format!("{}-{}", char::from(&class), rng.gen_range(10000..=999999)).to_owned();

        Body {
            name,
            kind: BodyType::Star(class),
            radius,
            mass,
            children: vec![],
        }
    }

    pub fn generate_planet(zone: PlanetZone) -> Self {
        let mut rng = rand::thread_rng();

        #[rustfmt::skip]
        let planet_type: PlanetType = match (zone, rng.gen_range(1..=100)) {
            (_,                         1..=5)    => { PlanetType::AsteroidRing },

            (PlanetZone::InnerRing,     6..=60)   => { PlanetType::Rock         },
            (PlanetZone::InnerRing,     61..=100) => { PlanetType::Desert       },

            (PlanetZone::HabitableZone, 6..=8)    => { PlanetType::GasGiant     },
            (PlanetZone::HabitableZone, 9..=40)   => { PlanetType::Rock         },
            (PlanetZone::HabitableZone, 41..=90)  => { PlanetType::Desert       },
            (PlanetZone::HabitableZone, 91..=100) => { PlanetType::Earthlike    },

            (PlanetZone::OuterRing,     6..=75)   => { PlanetType::GasGiant     },
            (PlanetZone::OuterRing,     76..=80)  => { PlanetType::Rock         },
            (PlanetZone::OuterRing,     81..=90)  => { PlanetType::Ice          },
            (PlanetZone::OuterRing,     91..=100) => { PlanetType::Desert       },

            _ => unreachable!()
        };

        let radius = match planet_type {
            PlanetType::AsteroidRing => { -1.0 }
            PlanetType::Earthlike => { rng.gen_range(7.0..=17.0) * 1000 }
            PlanetType::Ice => { rng.gen_range(1.0..=10.0) * 1000 }
            PlanetType::Rock => { rng.gen_range(1.0..=10.0) * 1000 }
            PlanetType::Desert => { rng.gen_range(4.0..=14.0) * 1000 }
            PlanetType::GasGiant => { rng.gen_range(2.0..=18.0) * 10000 }
        };

        Body {
            name: String::from("Not an Earth"),
            kind: BodyType::Planet(planet_type),
            radius,
            mass: rng.gen_range(2.9..=1.57 * 10 ** 142).ln() * 5.97 * 10 ** 24,
            children: vec![],
        }
    }
}