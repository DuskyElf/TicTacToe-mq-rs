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

#[derive(Clone, Copy)]
pub enum Point {
    I,
    Ii,
    Iii,
}

impl Point {
    pub fn value(&self) -> usize {
        match self {
            Self::I => 0,
            Self::Ii => 1,
            Self::Iii => 2,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Place {
    pub row: Point,
    pub collum: Point,
}

impl Place {
    pub fn up(&mut self) {
        self.row = match self.row {
            Point::I => Point::Iii,
            Point::Ii => Point::I,
            Point::Iii => Point::Ii,
        }
    }

    pub fn down(&mut self) {
        self.row = match self.row {
            Point::I => Point::Ii,
            Point::Ii => Point::Iii,
            Point::Iii => Point::I,
        }
    }

    pub fn left(&mut self) {
        self.collum = match self.collum {
            Point::I => Point::Iii,
            Point::Ii => Point::I,
            Point::Iii => Point::Ii,
        }
    }

    pub fn right(&mut self) {
        self.collum = match self.collum {
            Point::I => Point::Ii,
            Point::Ii => Point::Iii,
            Point::Iii => Point::I,
        }
    }
}

pub struct Board{
    pub board_state: [[Cell; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {board_state: [[Cell::Empty; 3]; 3]}
    }

    pub fn play_move(&mut self, place: &Place, player: &Player) {
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

pub struct Game {
    pub board: Board,
    pub selector: Place,
    pub current_turn: Player,
}

pub enum GameResult {
    Won(Player),
    Draw,
}