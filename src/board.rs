use std::fmt;

pub const MAX_GAME_SCORE: isize = 100000;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    Yellow,
    Red,
}

impl Piece {
    pub fn as_int(&self) -> isize {
        match self {
            Piece::Yellow => 1,
            Piece::Red => -1,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Piece::Yellow => write!(f, "■"),
            Piece::Red => write!(f, "●"),
        }
    }
}

impl std::ops::Not for Piece {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Piece::Yellow => Piece::Red,
            Piece::Red => Piece::Yellow,
        }
    }
}

#[derive(Clone)]
pub enum GameOutcome {
    Winner(Piece),
    Draw,
}

#[derive(PartialEq, Debug)]
pub enum InsertionError {
    FilledSlot,
    GameFinished,
}

#[derive(Clone)]
pub struct Board {
    board_state: [[Option<Piece>; 7]; 6],
    pub outcome: Option<GameOutcome>,
    pub current_turn: Piece,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp = String::new();
        disp += " 1 2 3 4 5 6 7\n";
        for y in 0..6 {
            disp += "|";
            for x in 0..7 {
                match self.board_state[y][x] {
                    Some(piece) => disp += format!("{}|", piece).as_str(),
                    None => disp += " |",
                }
            }
            disp += "\n";
        }
        disp += "===============";
        write!(f, "{}", disp)
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            board_state: [[None; 7]; 6],
            outcome: None,
            current_turn: Piece::Yellow,
        }
    }

    pub fn insert_at(&mut self, col: usize) -> Result<(), InsertionError> {
        assert!(1 <= col);
        assert!(col <= 7);

        if self.outcome.is_some() {
            return Err(InsertionError::GameFinished);
        }

        let col = col - 1;

        // Simulate dropping down
        let mut y = 0;
        while y < 5 && self.board_state[y + 1][col].is_none() {
            y += 1;
        }

        // Add a piece and change turn if slot found is empty
        if self.board_state[y][col].is_none() {
            self.board_state[y][col] = Some(self.current_turn);

            //Update the winner if there is one
            self.update_outcome();

            // Change turn if game is not over
            if self.outcome.is_none() {
                self.current_turn = !self.current_turn;
            }
        } else {
            return Err(InsertionError::FilledSlot);
        }

        Ok(())
    }

    fn update_outcome(&mut self) {
        if self.outcome.is_some() {
            return;
        }
        // No slots are empty
        if !(0..7).into_iter().any(|col| self.is_slot_empty(col)) {
            self.outcome = Some(GameOutcome::Draw)
        }

        let total_score = self.get_total_score();
        if total_score == MAX_GAME_SCORE {
            self.outcome = Some(GameOutcome::Winner(Piece::Yellow))
        } else if total_score == -MAX_GAME_SCORE {
            self.outcome = Some(GameOutcome::Winner(Piece::Red))
        }
    }

    fn is_slot_empty(&self, col: usize) -> bool {
        self.board_state[0][col].is_none()
    }

    pub fn get_valid_moves(&self) -> Vec<usize> {
        let mut moves = Vec::new();
        for x in [3,2,4,1,5,0,6].iter() {
            if self.is_slot_empty(*x) {
                moves.push(x + 1);
            }
        }
        moves
    }

    /// Gets the score from the perspective of Player A (Yellow)
    /// Score is based on the number of consecutive pieces
    /// starting from (x,y) and up to (x+dx*3, y+dy*3), with (dx,dy) in range [-1,1].
    /// Scoring function from https://github.com/gimu/connect-four-js is used as a guide.
    pub fn score(&self, x: usize, y: usize, dx: isize, dy: isize) -> isize {
        if let Some(GameOutcome::Winner(piece)) = self.outcome {
            return MAX_GAME_SCORE*piece.as_int();
        } /*else if let Some(GameOutcome::Draw) = self.outcome {
            return 0;
        }*/

        let mut x = x;
        let mut y = y;

        let mut yellow_points = 0;
        let mut red_points = 0;

        for _ in 0..4 {
            if let Some(piece) = self.board_state[y][x] {
                match piece {
                    Piece::Yellow => yellow_points+=1,
                    Piece::Red => red_points+=1
                }
            }

            x = (x as isize + dx) as usize;
            y = (y as isize + dy) as usize;
        }

        if yellow_points == 4 {
            return MAX_GAME_SCORE;
        } else if red_points == 4 {
            return -MAX_GAME_SCORE;
        }

        /*match self.current_turn {
            Piece::Yellow => yellow_points,
            Piece::Red => -red_points,
        }*/

        yellow_points-red_points
        //red_points*self.current_turn.as_int()
    }

    /// Gets the score from the perspective of Player A (Yellow)
    pub fn get_total_score(&self) -> isize {
        let mut hor = 0;
        let mut vert = 0;
        let mut diag1 = 0;
        let mut diag2 = 0;

        // Horizontal
        for y in 0..6 {
            for x in 0..4 {
                let score = self.score(x,y,1,0);
                if score == MAX_GAME_SCORE { return score }
                if score == -MAX_GAME_SCORE { return score }
                hor += score;
            }
        }

        // Vertical
        for y in 0..3 {
            for x in 0..7 {
                let score = self.score(x,y,0,1);
                if score == MAX_GAME_SCORE { return score }
                if score == -MAX_GAME_SCORE { return score }
                vert += score;
            }
        }

        // Diagonal 1
        for y in 0..3 {
            for x in 0..4 {
                let score = self.score(x,y,1,1);
                if score == MAX_GAME_SCORE { return score }
                if score == -MAX_GAME_SCORE { return score }
                diag1 += score;
            }
        }

        // Diagonal 2
        for y in 3..6 {
            for x in 0..4 {
                let score = self.score(x,y,1,-1);
                if score == MAX_GAME_SCORE { return score }
                if score == -MAX_GAME_SCORE { return score }
                diag2 += score;
            }
        }

        hor+vert+diag1+diag2
    }
}
