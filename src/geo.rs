// file:///home/lap/Repos/earth-distance-game/src/geo.rs {"mtime":1688222756545,"ctime":1687090795864,"size":1740,"etag":"3ap5gc7pq1p4","orphaned":false,"typeId":""}
use std::fmt;
use yew::Properties;

use gloo::console::log;

use serde::{Deserialize, Serialize};

// use yew::{function_component, html, Html, Properties};
use crate::traits::ChangeAngle;
//
#[derive(Default, Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum AngleUnit {
    #[default]
    Degrees,
    Radians,
}

#[derive(Properties, Debug, Default, Clone, Copy, PartialEq,Deserialize, Serialize)]
pub struct Coord {
    pub latitude: f64,
    pub longitude: f64,
    pub type_: AngleUnit,
}

impl Coord{
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            type_: AngleUnit::Degrees,
        }
    }
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



pub fn score(off_by: f64, radius: f64) -> f64 {
    let half_circum = 40_075.017 / 2.0;

    let norm_maxi_dist = (2.0 * radius).min(half_circum);

    log!("half_circum: ", half_circum);
    log!("off_by: ", off_by);

    100.0 * ((norm_maxi_dist - off_by.abs()) / norm_maxi_dist).max(0.0)
}


