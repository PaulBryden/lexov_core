#![allow(dead_code)]
mod lexov_core {
    use serde::{Deserialize, Serialize};
    use ts_rs::{export, TS};

    #[derive(Serialize, Deserialize, TS)]
    pub struct Position {
        x: f64,
        y: f64,
        z: f64,
    }
    export! {
        Position => "ts/structures.ts"
    }
}
