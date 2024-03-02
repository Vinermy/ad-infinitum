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
use std::fs;
use std::ops::Deref;
use rand::{Rng, thread_rng};
use rand::seq::IndexedRandom;
use ratatui::style::Color;

#[derive(Clone)]
pub enum BodyType {
    Star(StarType),
    Planet(PlanetType),
}

#[derive(Clone)]
pub enum StarType {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

#[derive(Clone)]
pub enum PlanetType {
    AsteroidRing,
    Earthlike,
    Ice,
    Rock,
    Desert,
    GasGiant,
}

impl Into<Color> for BodyType {
    fn into(self) -> Color {
        match self {
            BodyType::Planet(PlanetType::AsteroidRing) => { Color::Rgb(100, 100, 100) }
            BodyType::Planet(PlanetType::Earthlike) => { Color::LightBlue }
            BodyType::Planet(PlanetType::Ice) => { Color::LightCyan }
            BodyType::Planet(PlanetType::Rock) => { Color::DarkGray }
            BodyType::Planet(PlanetType::Desert) => { Color::LightYellow }
            BodyType::Planet(PlanetType::GasGiant) => { Color::LightRed }

            BodyType::Star(StarType::O) => { Color::Indexed(27) }
            BodyType::Star(StarType::B) => { Color::Indexed(33) }
            BodyType::Star(StarType::A) => { Color::Indexed(195) }
            BodyType::Star(StarType::F) => { Color::Indexed(231) }
            BodyType::Star(StarType::G) => { Color::Indexed(230) }
            BodyType::Star(StarType::K) => { Color::Indexed(216) }
            BodyType::Star(StarType::M) => { Color::Indexed(160) }
        }
    }
}

#[derive(Clone)]
pub enum PlanetZone {
    InnerRing,
    HabitableZone,
    OuterRing,
}

impl StarType {
    fn to_str(&self) -> String {
        match self {
            StarType::O => { "O".to_owned() }
            StarType::B => { "B".to_owned() }
            StarType::A => { "A".to_owned() }
            StarType::F => { "F".to_owned() }
            StarType::G => { "G".to_owned() }
            StarType::K => { "K".to_owned() }
            StarType::M => { "M".to_owned() }
        }
    }
}

#[derive(Clone)]
pub struct Body {
    pub name: String,
    pub kind: BodyType,
    pub radius: f32,
    pub mass: f32,
    pub orbit_radius: Option<f32>,
    pub orbit_period: Option<f32>,
    pub satellites: Vec<Body>,
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
        ) * 1.98 * 10.0f32.powi(18);

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
        ) * 6.957 * 10.0f32.powi(5);

        let name = format!("{}-{}", class.to_str(), rng.gen_range(10000..=999999)).to_owned();

        Body {
            name,
            kind: BodyType::Star(class),
            radius,
            mass,
            orbit_radius: None,
            orbit_period: None,
            satellites: Vec::new(),
        }
    }

    pub fn generate_planet(zone: &PlanetZone) -> Self {
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
            PlanetType::Earthlike => { rng.gen_range(7.0..=17.0) * 1000.0 }
            PlanetType::Ice => { rng.gen_range(1.0..=10.0) * 1000.0 }
            PlanetType::Rock => { rng.gen_range(1.0..=10.0) * 1000.0 }
            PlanetType::Desert => { rng.gen_range(4.0..=14.0) * 1000.0 }
            PlanetType::GasGiant => { rng.gen_range(2.0..=18.0) * 10000.0 }
        } / 2.0 * 1000.0;

        let density = match planet_type {
            PlanetType::GasGiant => { rng.gen_range(0.7..=1.6) * 1000.0 }
            _ => { rng.gen_range(3.5..=5.4) * 1000.0 }
        };
        
        let mass = 4.0 * std::f32::consts::PI * f32::powi(radius, 2) * density;

        let file_contents: String = fs::read_to_string("./assets/system_namelist.txt").unwrap();
        let names: Vec<&str> = file_contents.split("\n").collect();
        let name: &str = *names.choose(&mut thread_rng()).unwrap();

        Body {
            name: String::from(name),
            kind: BodyType::Planet(planet_type),
            radius,
            mass,

            orbit_radius: None,
            orbit_period: None,
            satellites: Vec::new(),
        }
    }

    pub fn generate_planet_with_count(planet_zone: &PlanetZone, count: i32) -> Vec<Body> {
        let mut res: Vec<Body> = Vec::new();
        for i in 0..count {
            res.push(Self::generate_planet(planet_zone));
        }
        res
    }

    fn get_class_as_string(&self) -> String {
        match self.clone().kind {
            BodyType::Star(class) => {
                String::from(class.to_str() + " class star")
            }
            BodyType::Planet(kind) => {
                match kind {
                    PlanetType::AsteroidRing => { String::from("Asteroid ring") }
                    PlanetType::Earthlike => { String::from("Earthlike planet") }
                    PlanetType::Ice => { String::from("Frozen planet") }
                    PlanetType::Rock => { String::from("Rocky planet") }
                    PlanetType::Desert => { String::from("Deserted planet") }
                    PlanetType::GasGiant => { String::from("Gas giant") }
                }
            }
        }
    }

    pub fn make_info(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        res.push(format!("Name: {}", self.name).to_owned());
        res.push(format!("Mass: {:.3e} kg", self.mass).to_owned());
        res.push(format!("Radius: {:.3e} m", self.radius).to_owned());
        res.push(format!("Type: {}", self.get_class_as_string()));
        res.push(format!("Orbit radius: {:.3e} km", self.orbit_radius.unwrap_or(0.0)));

        res
    }
}