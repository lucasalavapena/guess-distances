use std::fmt;
use yew::Properties;

use csv::{Reader, ReaderBuilder};
use gloo::console::log;
use std::error::Error;
use std::fs::File;

// use yew::{function_component, html, Html, Properties};
use crate::traits::ChangeAngle;
//
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum AngleUnit {
    #[default]
    Degrees,
    Radians,
}

pub struct CoordSystem {
    flattening: f64,
    equatorial_radius: f64,
}

pub const WGS84: CoordSystem = CoordSystem {
    flattening: 1.0 / 298.257223563, // figure out units
    equatorial_radius: 6_378_137.0,  // m
};

#[derive(Properties, Debug, Default, Clone, Copy, PartialEq)]
pub struct Coord {
    pub latitude: f64,
    pub longitude: f64,
    pub type_: AngleUnit,
}

fn degrees_to_radians(angle: f64) -> f64 {
    angle.to_radians()
}

fn radians_to_degrees(angle: f64) -> f64 {
    angle.to_degrees()
}

impl ChangeAngle for Coord {
    fn change_unit(&self) -> Self {
        let (new_type, conversion_func): (AngleUnit, fn(f64) -> f64) = match self.type_ {
            AngleUnit::Degrees => (AngleUnit::Radians, degrees_to_radians),
            AngleUnit::Radians => (AngleUnit::Degrees, radians_to_degrees),
        };

        Coord {
            latitude: conversion_func(self.latitude),
            longitude: conversion_func(self.longitude),
            type_: new_type,
        }
    }
}
// show to the nearest 5 decimal points?
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.latitude, self.longitude)
    }
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct City {
    pub coordinates: Coord,
    pub country_name: String,
    pub city_name: String,
    pub country_code: Option<[char; 2]>,
    pub continent_name: String,
}

impl City {
    pub fn full_name(&self) -> String {
        format!("{}, {}", self.city_name, self.country_name)
    }
}

impl Default for City {
    fn default() -> Self {
        Self {
            coordinates: Coord {
                latitude: 1.3521,
                longitude: 103.8198,
                type_: AngleUnit::Degrees,
            },
            country_name: String::from("Singapore"),
            city_name: String::from("Singapore"),
            country_code: Some(['S', 'G']),
            continent_name: String::from("Asia"),
        }
    }
}

impl ChangeAngle for City {
    fn change_unit(&self) -> Self {
        Self {
            coordinates: self.coordinates.change_unit(),
            country_name: self.country_name.clone(),
            city_name: self.city_name.clone(),
            country_code: self.country_code,
            continent_name: self.continent_name.clone(),
        }
    }
}

fn get_country_code_from_string(country_code: &str) -> Option<[char; 2]> {
    match country_code {
        "NULL" => None,
        _ => Some(
            country_code
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap(),
        ),
    }
}

pub fn load_cities_from_csv(file_path: &str) -> Result<Vec<City>, Box<dyn Error>> {
    let mut res = vec![];
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);

    for result in reader.records() {
        let record = result?;

        let curr_city = City {
            coordinates: Coord {
                latitude: record.get(2).unwrap().parse::<f64>().unwrap(),
                longitude: record.get(3).unwrap().parse::<f64>().unwrap(),
                type_: AngleUnit::Degrees,
            },
            country_name: record.get(0).unwrap().to_owned(),
            city_name: record.get(1).unwrap().to_owned(),
            country_code: get_country_code_from_string(record.get(4).unwrap()),
            continent_name: record.get(5).unwrap().to_owned(),
        };
        res.push(curr_city);
    }
    Ok(res)
}

pub fn load_cities_from_str(csv_string: &str) -> Result<Vec<City>, Box<dyn Error>> {
    let mut res = vec![];
    let mut reader = ReaderBuilder::new().from_reader(csv_string.as_bytes());

    for result in reader.records() {
        let record = result?;

        let curr_city = City {
            coordinates: Coord {
                latitude: record.get(2).unwrap().parse::<f64>().unwrap(),
                longitude: record.get(3).unwrap().parse::<f64>().unwrap(),
                type_: AngleUnit::Degrees,
            },
            country_name: record.get(0).unwrap().to_owned(),
            city_name: record.get(1).unwrap().to_owned(),
            country_code: get_country_code_from_string(record.get(4).unwrap()),
            continent_name: record.get(5).unwrap().to_owned(),
        };
        res.push(curr_city);
    }
    Ok(res)
}

fn reduce_lat(latitude: f64, flattening: f64) -> f64 {
    ((1.0 - flattening) * (latitude.tan())).atan()
}

fn compute_central_angle(red_lat_a: f64, long_a: f64, red_lat_b: f64, long_b: f64) -> f64 {
    ((red_lat_a.cos() * red_lat_b.cos() * (long_b - long_a).cos())
        + (red_lat_a.sin() * red_lat_b.sin()))
    .acos()
}

// Lambert's formula for long lines
// https://en.wikipedia.org/wiki/Geographical_distance#cite_note-13
pub fn compute_distance(coord_a: &Coord, coord_b: &Coord, coord_system: CoordSystem) -> f64 {
    let reduced_lat_a = reduce_lat(coord_a.latitude, coord_system.flattening);
    let reduced_lat_b = reduce_lat(coord_b.latitude, coord_system.flattening);

    let sigma = compute_central_angle(
        reduced_lat_a,
        coord_a.longitude,
        reduced_lat_b,
        coord_b.longitude,
    );

    let P = (reduced_lat_a + reduced_lat_b) / 2.0;
    let Q = (reduced_lat_a - reduced_lat_b) / 2.0;

    let X: f64 = (sigma - sigma.sin()) * (P.sin().powf(2.0) * Q.cos().powf(2.0))
        / ((sigma / 2.0).cos().powf(2.0));
    let Y: f64 = (sigma + sigma.sin()) * (Q.sin().powf(2.0) * P.cos().powf(2.0))
        / ((sigma / 2.0).sin().powf(2.0));

    let distance =
        coord_system.equatorial_radius * (sigma - (coord_system.flattening / 2.0) * (X + Y));
    distance.abs()
}

// pub fn abs_dist<T>(d1: T, d2: T) -> T
// where T: std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialOrd,
// {
//     (d1 - d2).abs()
// }

pub fn abs_dist(d1: usize, d2: usize) -> usize {
    d1.max(d2) - d1.min(d2)
}

pub fn is_within(d1: f64, d2: f64, margin: f64) -> bool {
    (d1 - d2).abs() <= margin
}

pub fn score(off_by: f64) -> f64 {
    let half_circum = 40_075.017 / 2.0;
    log!("half_circum: ", half_circum);
    log!("off_by: ", off_by);

    100.0 * ((half_circum - off_by.abs()) / half_circum).max(0.0)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_within() {
        let test1 = is_within(10500.00, 10540.00, 50.0);
        assert_eq!(test1, true);

        let test2 = is_within(10500.00, 10550.00, 50.0);
        assert_eq!(test2, true);

        let test3 = is_within(10500.00, 10550.00, 45.0);
        assert_eq!(test3, false);
    }
    #[test]
    fn test_compute_distance() {
        let singapore = Coord {
            latitude: 1.3521,
            longitude: 103.8198,
            type_: AngleUnit::Degrees,
        }
        .change_unit();
        let london = Coord {
            latitude: 51.5072,
            longitude: -0.1276,
            type_: AngleUnit::Degrees,
        }
        .change_unit();

        println!("{}", singapore);

        let res = compute_distance(&singapore, &london, WGS84);
        println!("{}", res);
        // https://www.distance.to/London/Singapore
        let expected = 10_845_290.0;
        assert!(is_within(res, expected, 50_000.0))
    }
}

#[test]
fn test_read_csv_file() {
    load_cities_from_csv("/home/lap/Repos/earth-distance-calculator-game/src/data/citys.csv");
}

#[test]
fn test_nathan() {
    assert_eq!(((0.776_f32).sin()).powf(2.0), (0.776_f32).sin().sin())
}
