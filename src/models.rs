use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn print(&self) -> &str {
         match self {
            Self::X => return "X",
            Self::O => return "O",
         }
     }
 }

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Filled(Player),
}

impl Cell {
   pub fn print(&self) -> &str {
        match self {
            Self::Empty => return " ",
            Self::Filled(player) => {
                match player {
                    Player::X => return "X",
                    Player::O => return "O",
                }
            }
        }
    }
}

pub enum Point {
    I,
    Ii,
    Iii,
}

impl Point {
    fn value(&self) -> usize {
        match self {
            Self::I => 0,
            Self::Ii => 1,
            Self::Iii => 2,
        }
    }
}

pub struct Place {
    pub row: Point,
    pub collum: Point,
}

pub struct Board{
    pub board_state: [[Cell; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {board_state: [[Cell::Empty; 3]; 3]}
    }

    pub fn play_move(&mut self, place: Place, player: &Player) {
        self[&place] = Cell::Filled(player.clone());
    }
}

impl Index<&Place> for Board {
    type Output = Cell;

    fn index(&self, index: &Place) -> &Self::Output {
        &self.board_state[index.row.value()][index.collum.value()]
    } 
}

impl IndexMut<&Place> for Board {
    fn index_mut(&mut self, index: &Place) -> &mut Cell {
        &mut self.board_state[index.row.value()][index.collum.value()]
    } 
}

pub enum Winner {
    Won(Player),
    Draw,
}

pub struct GameResult {
    pub winner: Winner,
    pub game_lap: u8,
}