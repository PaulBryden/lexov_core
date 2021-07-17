#![allow(dead_code)]
pub mod data_structures {
    use serde::{Deserialize, Serialize};
    use ts_rs::{export, TS};

    #[derive(Serialize, Deserialize, TS, Clone, Copy)]
    pub struct Position {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Serialize, Deserialize, TS, Clone, Copy)]
    pub struct Player {
        pub uuid: u64,
        pub position: Position,
    }
    
    
    #[derive(Serialize, Deserialize, TS, Clone )]
    pub struct PlayerList {
        pub players: Vec<Player>,
    }
    export! {
        Position, Player, PlayerList => "ts/structures.ts"
    }
}
