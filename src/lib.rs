#![allow(dead_code)]
pub mod data_structures {
    use serde::{Deserialize, Serialize};
    use ts_rs::{export, TS};

    #[derive(Serialize, Deserialize, TS, Clone)]
    pub struct Position {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    export! {
        Position => "ts/structures.ts"
    }
}
