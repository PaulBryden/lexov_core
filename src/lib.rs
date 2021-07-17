#![allow(dead_code)]
use serde::{Serialize, Deserialize};
use ts_rs::{TS, export};

#[derive(Serialize, Deserialize, TS)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

export! {
    Position => "ts/structures.ts"
}