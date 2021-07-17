export interface Position {
  x: number;
  y: number;
  z: number;
}

export interface Player {
  uuid: number;
  position: Position;
}

export interface PlayerList {
  players: Player[];
}